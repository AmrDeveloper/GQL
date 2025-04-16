use std::fs;
use std::io;
use std::io::IsTerminal;

use gitql::create_gitql_environment;
use gitql::gitql_data_provider::GitQLDataProvider;
use gitql::validate_git_repositories;
use gitql_cli::arguments;
use gitql_cli::arguments::Arguments;
use gitql_cli::arguments::Command;
use gitql_cli::diagnostic_reporter;
use gitql_cli::diagnostic_reporter::DiagnosticReporter;
use gitql_cli::printer::BaseOutputPrinter;
use gitql_cli::printer::CSVPrinter;
use gitql_cli::printer::JSONPrinter;
use gitql_cli::printer::OutputFormatKind;
use gitql_cli::printer::TablePrinter;
use gitql_cli::printer::YAMLPrinter;
use gitql_core::environment::Environment;
use gitql_engine::data_provider::DataProvider;
use gitql_engine::engine;
use gitql_engine::engine::EvaluationResult::SelectedGroups;
use gitql_parser::diagnostic::Diagnostic;
use gitql_parser::parser;
use gitql_parser::tokenizer::Tokenizer;
use lineeditor::LineEditorResult;

mod gitql;

fn main() {
    if cfg!(debug_assertions) {
        std::env::set_var("RUST_BACKTRACE", "full");
        std::env::set_var("RUST_LIB_BACKTRACE", "1");
    }

    let args: Vec<String> = std::env::args().collect();
    let command = arguments::parse_arguments(&args);

    match command {
        Command::ReplMode(arguments) => {
            launch_gitql_repl(&arguments);
        }
        Command::ScriptMode(script_file, arguments) => {
            let mut reporter = diagnostic_reporter::DiagnosticReporter::default();
            let git_repos_result = validate_git_repositories(&arguments.repos);
            if git_repos_result.is_err() {
                reporter.report_diagnostic(
                    "",
                    Diagnostic::error(git_repos_result.err().unwrap().as_str()),
                );
                std::process::exit(1);
            }

            let repos = git_repos_result.ok().unwrap();
            let mut env = create_gitql_environment();
            let query =
                fs::read_to_string(script_file).expect("Should have been able to read the file");
            execute_gitql_query(query, &arguments, &repos, &mut env, &mut reporter);
        }
        Command::QueryMode(query, arguments) => {
            let mut reporter = diagnostic_reporter::DiagnosticReporter::default();
            let git_repos_result = validate_git_repositories(&arguments.repos);
            if git_repos_result.is_err() {
                reporter.report_diagnostic(
                    &query,
                    Diagnostic::error(git_repos_result.err().unwrap().as_str()),
                );
                std::process::exit(1);
            }

            let repos = git_repos_result.ok().unwrap();
            let mut env = create_gitql_environment();

            execute_gitql_query(query, &arguments, &repos, &mut env, &mut reporter);
        }
        Command::Help => {
            arguments::print_help_list();
        }
        Command::Version => {
            println!("GitQL version {}", env!("CARGO_PKG_VERSION"));
        }
        Command::Error(error_message) => {
            println!("{}", error_message);
        }
    }
}
use gitql::gitql_line_editor::create_new_line_editor;

fn launch_gitql_repl(arguments: &Arguments) {
    let mut reporter = diagnostic_reporter::DiagnosticReporter::default();
    let git_repos_result = validate_git_repositories(&arguments.repos);
    if git_repos_result.is_err() {
        reporter.report_diagnostic(
            "",
            Diagnostic::error(git_repos_result.err().unwrap().as_str()),
        );
        std::process::exit(1);
    }

    let git_repositories = git_repos_result.ok().unwrap();
    let mut global_env = create_gitql_environment();

    // Launch the right line editor if the flag is enabled
    // Later this line editor will be the default editor
    if arguments.enable_line_editor {
        let mut line_editor = create_new_line_editor();
        loop {
            if let Ok(LineEditorResult::Success(input)) = line_editor.read_line() {
                println!();

                if input.is_empty() || input == "\n" {
                    continue;
                }

                if input == "exit" {
                    break;
                }

                execute_gitql_query(
                    input.to_owned(),
                    arguments,
                    &git_repositories,
                    &mut global_env,
                    &mut reporter,
                );

                global_env.clear_session();
            }
        }
        return;
    }

    let mut input = String::new();
    loop {
        let stdin = io::stdin();

        // Render Prompt only if input is received from terminal
        if stdin.is_terminal() {
            print!("gitql > ");
        }

        std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!");
        match stdin.read_line(&mut input) {
            Ok(buffer_length) => {
                if buffer_length == 0 {
                    break;
                }
            }
            Err(error) => {
                reporter.report_diagnostic(&input, Diagnostic::error(&format!("{}", error)));
            }
        }

        let stdin_input = input.trim();
        if stdin_input.is_empty() || stdin_input == "\n" {
            continue;
        }

        if stdin_input == "exit" {
            println!("Goodbye!");
            break;
        }

        execute_gitql_query(
            stdin_input.to_owned(),
            arguments,
            &git_repositories,
            &mut global_env,
            &mut reporter,
        );

        input.clear();
        global_env.clear_session();
    }
}

fn execute_gitql_query(
    query: String,
    arguments: &Arguments,
    repos: &[gix::Repository],
    env: &mut Environment,
    reporter: &mut DiagnosticReporter,
) {
    let front_start = std::time::Instant::now();
    let tokenizer_result = Tokenizer::tokenize(query.clone());
    if tokenizer_result.is_err() {
        let diagnostic = tokenizer_result.err().unwrap();
        reporter.report_diagnostic(&query, *diagnostic);
        std::process::exit(1);
    }

    let tokens = tokenizer_result.ok().unwrap();
    if tokens.is_empty() {
        std::process::exit(1);
    }

    let parser_result = parser::parse_gql(tokens, env);
    if parser_result.is_err() {
        let diagnostic = parser_result.err().unwrap();
        reporter.report_diagnostic(&query, *diagnostic);
        std::process::exit(1);
    }

    let query_node = parser_result.ok().unwrap();
    let front_duration = front_start.elapsed();

    let engine_start = std::time::Instant::now();
    let provider: Box<dyn DataProvider> = Box::new(GitQLDataProvider::new(repos.to_vec()));
    let evaluation_result = engine::evaluate(env, &provider, query_node);
    let engine_duration = engine_start.elapsed();

    // Report Runtime exceptions if they exists
    if evaluation_result.is_err() {
        let exception = Diagnostic::exception(&evaluation_result.err().unwrap());
        reporter.report_diagnostic("", exception);
        std::process::exit(1);
    }

    let printer: Box<dyn BaseOutputPrinter> = match arguments.output_format {
        OutputFormatKind::Table => {
            Box::new(TablePrinter::new(arguments.pagination, arguments.page_size))
        }
        OutputFormatKind::JSON => Box::new(JSONPrinter),
        OutputFormatKind::CSV => Box::new(CSVPrinter),
        OutputFormatKind::YAML => Box::new(YAMLPrinter),
    };

    // Render the result only if they are selected groups not any other statement
    let evaluations_results = evaluation_result.ok().unwrap();
    for evaluation_result in evaluations_results {
        let mut rows_count = 0;
        if let SelectedGroups(mut groups) = evaluation_result {
            if !groups.is_empty() {
                rows_count += groups.groups[0].len();
                printer.print(&mut groups);
            }
        }

        if arguments.analysis {
            let total_time = front_duration + engine_duration;
            println!(
                "{} row in set (total: {:?}, front: {:?}, engine: {:?})",
                rows_count, total_time, front_duration, engine_duration
            );
        }
    }
}
