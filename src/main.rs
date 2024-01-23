use atty::Stream;
use gitql_ast::environment::Environment;
use gitql_cli::arguments;
use gitql_cli::arguments::Arguments;
use gitql_cli::arguments::Command;
use gitql_cli::arguments::OutputFormat;
use gitql_cli::diagnostic_reporter;
use gitql_cli::diagnostic_reporter::DiagnosticReporter;
use gitql_cli::render;
use gitql_engine::engine;
use gitql_engine::engine::EvaluationResult::SelectedGroups;
use gitql_parser::diagnostic::Diagnostic;
use gitql_parser::parser;
use gitql_parser::tokenizer;

fn main() {
    if cfg!(debug_assertions) {
        std::env::set_var("RUST_BACKTRACE", "1");
        std::env::set_var("RUST_LIB_BACKTRACE", "1");
    }

    let args = std::env::args().collect();
    let command = arguments::parse_arguments(&args);

    match command {
        Command::ReplMode(arguments) => {
            launch_gitql_repl(arguments);
        }
        Command::QueryMode(query, arguments) => {
            let mut reporter = diagnostic_reporter::DiagnosticReporter::default();
            let git_repos_result = validate_git_repositories(&arguments.repos);
            if git_repos_result.is_err() {
                reporter.report_diagnostic(
                    &query,
                    Diagnostic::error(git_repos_result.err().unwrap().as_str()),
                );
                return;
            }

            let repos = git_repos_result.ok().unwrap();
            let mut env = Environment::default();
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

fn launch_gitql_repl(arguments: Arguments) {
    let mut reporter = diagnostic_reporter::DiagnosticReporter::default();
    let git_repos_result = validate_git_repositories(&arguments.repos);
    if git_repos_result.is_err() {
        reporter.report_diagnostic(
            "",
            Diagnostic::error(git_repos_result.err().unwrap().as_str()),
        );
        return;
    }

    let mut global_env = Environment::default();
    let git_repositories = git_repos_result.ok().unwrap();

    let mut input = String::new();

    loop {
        // Render Prompt only if input is received from terminal
        if atty::is(Stream::Stdin) {
            print!("gql > ");
        }

        std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!");
        match std::io::stdin().read_line(&mut input) {
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
            &arguments,
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
    let tokenizer_result = tokenizer::tokenize(query.clone());
    if tokenizer_result.is_err() {
        let diagnostic = tokenizer_result.err().unwrap();
        reporter.report_diagnostic(&query, *diagnostic);
        return;
    }

    let tokens = tokenizer_result.ok().unwrap();
    let parser_result = parser::parse_gql(tokens, env);
    if parser_result.is_err() {
        let diagnostic = parser_result.err().unwrap();
        reporter.report_diagnostic(&query, *diagnostic);
        return;
    }

    let query_node = parser_result.ok().unwrap();
    let front_duration = front_start.elapsed();

    let engine_start = std::time::Instant::now();
    let evaluation_result = engine::evaluate(env, repos, query_node);

    // Report Runtime exceptions if they exists
    if evaluation_result.is_err() {
        reporter.report_diagnostic(
            &query,
            Diagnostic::exception(&evaluation_result.err().unwrap()),
        );
        return;
    }

    // Render the result only if they are selected groups not any other statement
    let engine_result = evaluation_result.ok().unwrap();
    if let SelectedGroups(mut groups, hidden_selection) = engine_result {
        match arguments.output_format {
            OutputFormat::Render => {
                render::render_objects(
                    &mut groups,
                    &hidden_selection,
                    arguments.pagination,
                    arguments.page_size,
                );
            }
            OutputFormat::JSON => {
                let mut indexes = vec![];
                for (index, title) in groups.titles.iter().enumerate() {
                    if hidden_selection.contains(title) {
                        indexes.insert(0, index);
                    }
                }

                if groups.len() > 1 {
                    groups.flat()
                }

                for index in indexes {
                    groups.titles.remove(index);

                    for row in &mut groups.groups[0].rows {
                        row.values.remove(index);
                    }
                }

                if let Ok(json) = groups.as_json() {
                    println!("{}", json);
                }
            }
            OutputFormat::CSV => {
                let mut indexes = vec![];
                for (index, title) in groups.titles.iter().enumerate() {
                    if hidden_selection.contains(title) {
                        indexes.insert(0, index);
                    }
                }

                if groups.len() > 1 {
                    groups.flat()
                }

                for index in indexes {
                    groups.titles.remove(index);

                    for row in &mut groups.groups[0].rows {
                        row.values.remove(index);
                    }
                }

                if let Ok(csv) = groups.as_csv() {
                    println!("{}", csv);
                }
            }
        }
    }

    let engine_duration = engine_start.elapsed();

    if arguments.analysis {
        println!("\n");
        println!("Analysis:");
        println!("Frontend : {:?}", front_duration);
        println!("Engine   : {:?}", engine_duration);
        println!("Total    : {:?}", (front_duration + engine_duration));
        println!("\n");
    }
}

fn validate_git_repositories(repositories: &Vec<String>) -> Result<Vec<gix::Repository>, String> {
    let mut git_repositories: Vec<gix::Repository> = vec![];
    for repository in repositories {
        let git_repository = gix::open(repository);
        if git_repository.is_err() {
            return Err(git_repository.err().unwrap().to_string());
        }
        git_repositories.push(git_repository.ok().unwrap());
    }
    Ok(git_repositories)
}
