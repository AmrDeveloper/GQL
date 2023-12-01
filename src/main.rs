use gitql_cli::arguments;
use gitql_cli::arguments::Arguments;
use gitql_cli::arguments::Command;
use gitql_cli::render;
use gitql_cli::reporter;
use gitql_cli::reporter::DiagnosticReporter;
use gitql_engine::engine;
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
            let mut reporter = reporter::DiagnosticReporter::default();
            let git_repos_result = validate_git_repositories(&arguments.repos);
            if git_repos_result.is_err() {
                reporter.report_error(git_repos_result.err().unwrap().as_str());
                return;
            }

            let repos = git_repos_result.ok().unwrap();
            execute_gitql_query(query, &arguments, &repos, &mut reporter);
        }
        Command::Help => {
            arguments::print_help_list();
        }
        Command::Version => {
            let version = env!("CARGO_PKG_VERSION");
            println!("GitQL version {}", version);
        }
        Command::Error(error_mssage) => {
            println!("{}", error_mssage);
        }
    }
}

fn launch_gitql_repl(arguments: Arguments) {
    let mut reporter = reporter::DiagnosticReporter::default();
    let git_repos_result = validate_git_repositories(&arguments.repos);
    if git_repos_result.is_err() {
        reporter.report_error(git_repos_result.err().unwrap().as_str());
        return;
    }

    let git_repositories = git_repos_result.ok().unwrap();

    let mut input = String::new();
    loop {
        print!("gql > ");
        std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!");

        match std::io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(_err) => reporter.report_error("Invalid input"),
        }

        let trimed_input = input.trim();
        if trimed_input.is_empty() || trimed_input == "\n" {
            continue;
        }

        if trimed_input == "exit" {
            println!("Goodbye!");
            break;
        }

        execute_gitql_query(
            trimed_input.to_owned(),
            &arguments,
            &git_repositories,
            &mut reporter,
        );

        input.clear();
    }
}

fn execute_gitql_query(
    query: String,
    arguments: &Arguments,
    repos: &[gix::Repository],
    reporter: &mut DiagnosticReporter,
) {
    let front_start = std::time::Instant::now();
    let tokenizer_result = tokenizer::tokenize(query);
    if tokenizer_result.is_err() {
        reporter.report_gql_error(tokenizer_result.err().unwrap());
        return;
    }

    let tokens = tokenizer_result.ok().unwrap();
    let parser_result = parser::parse_gql(tokens);
    if parser_result.is_err() {
        reporter.report_gql_error(parser_result.err().unwrap());
        return;
    }

    let statements = parser_result.ok().unwrap();
    let front_duration = front_start.elapsed();

    let engine_start = std::time::Instant::now();
    let evaluation_result = engine::evaluate(repos, statements);

    // Report Runtime exceptions if they exists
    if evaluation_result.is_err() {
        reporter.report_runtime_error(evaluation_result.err().unwrap());
        return;
    }

    let mut evaluation_values = evaluation_result.ok().unwrap();
    render::render_objects(
        &mut evaluation_values.groups,
        &evaluation_values.hidden_selections,
        arguments.pagination,
        arguments.page_size,
    );

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
    for repsitory in repositories {
        let git_repository = gix::open(repsitory);
        if git_repository.is_err() {
            return Err(git_repository.err().unwrap().to_string());
        }
        git_repositories.push(git_repository.ok().unwrap());
    }
    Ok(git_repositories)
}
