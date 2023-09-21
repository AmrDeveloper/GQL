use gitql_cli::arguments;
use gitql_cli::render;
use gitql_cli::reporter;
use gitql_engine::engine;
use gitql_parser::parser;
use gitql_parser::tokenizer;

fn main() {
    if cfg!(debug_assertions) {
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    let arguments = arguments::parse_arguments();
    let mut reporter = reporter::DiagnosticReporter::new();
    let mut git_repositories: Vec<git2::Repository> = vec![];
    for repsitory in arguments.repos {
        let git_repository = git2::Repository::open(repsitory);
        if git_repository.is_err() {
            reporter.report_error(git_repository.err().unwrap().message());
            return;
        }
        git_repositories.push(git_repository.ok().unwrap());
    }

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

        let front_start = std::time::Instant::now();
        let tokenizer_result = tokenizer::tokenize(trimed_input.to_string());
        if tokenizer_result.is_err() {
            reporter.report_gql_error(tokenizer_result.err().unwrap());
            input.clear();
            continue;
        }

        let tokens = tokenizer_result.ok().unwrap();
        let parser_result = parser::parse_gql(tokens);
        if parser_result.is_err() {
            reporter.report_gql_error(parser_result.err().unwrap());
            input.clear();
            continue;
        }

        let statements = parser_result.ok().unwrap();
        let front_duration = front_start.elapsed();

        let engine_start = std::time::Instant::now();
        let evaluation_result = engine::evaluate(&git_repositories, statements);

        // Report Runtime exceptions if they exists
        if evaluation_result.is_err() {
            reporter.report_runtime_error(evaluation_result.err().unwrap());
            input.clear();
            continue;
        }

        let evaluation_values = evaluation_result.ok().unwrap();
        render::render_objects(
            &evaluation_values.groups,
            &evaluation_values.hidden_selections,
        );

        let engine_duration = engine_start.elapsed();
        input.clear();

        if arguments.analysis {
            println!("\n");
            println!("Analysis:");
            println!("Frontend : {:?}", front_duration);
            println!("Engine   : {:?}", engine_duration);
            println!("Total    : {:?}", (front_duration + engine_duration));
            println!("\n");
        }
    }
}
