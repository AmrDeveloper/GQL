mod aggregation;
mod colored_stream;
mod diagnostic;
mod engine;
mod engine_function;
mod expression;
mod object;
mod parser;
mod render;
mod statement;
mod tokenizer;
mod transformation;
mod types;

use diagnostic::DiagnosticEngine;

fn main() {
    let print_analysis = false;
    let mut diagnostics = DiagnosticEngine::new();

    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        diagnostics.report_error("Invalid number of arguments");
        diagnostics.report_error("Usage: gql <repository path>");
        return;
    }

    let working_path = &args[1];
    let repository = git2::Repository::open(working_path);
    if repository.is_err() {
        let error = repository.err();
        diagnostics.report_error(error.unwrap().message());
        return;
    }

    let mut input = String::new();

    loop {
        print!("gql > ");

        std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!");

        match std::io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(_err) => diagnostics.report_error("Invalid input"),
        }

        if input.trim() == "exit" {
            println!("Bye");
            break;
        }

        let front_start = std::time::Instant::now();
        let tokenizer_result = tokenizer::tokenize(input.trim().to_string());
        if tokenizer_result.is_err() {
            diagnostics.report_gql_error(tokenizer_result.err().unwrap());
            input.clear();
            continue;
        }

        let tokens = tokenizer_result.ok().unwrap();
        let parser_result = parser::parse_gql(tokens);
        if parser_result.is_err() {
            diagnostics.report_gql_error(parser_result.err().unwrap());
            input.clear();
            continue;
        }

        let statements = parser_result.ok().unwrap();
        let repo = repository.as_ref().unwrap();
        let front_duration = front_start.elapsed();

        let engine_start = std::time::Instant::now();
        engine::evaluate(repo, statements);
        let engine_duration = engine_start.elapsed();
        input.clear();

        if print_analysis {
            println!("\n");
            println!("Analysis:");
            println!("Frontend : {:?}", front_duration);
            println!("Engine   : {:?}", engine_duration);
            println!("\n");
        }
    }
}
