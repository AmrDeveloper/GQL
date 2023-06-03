mod engine;
mod engine_function;
mod expression;
mod object;
mod parser;
mod statement;
mod tokenizer;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        println!("Invalid number of arguments");
        println!("Usage: {} <repository path>", args[0]);
        return;
    }

    let working_path = &args[1];
    let repository = git2::Repository::open(working_path);
    if repository.is_err() {
        let error = repository.err();
        println!("ERROR: {}", error.unwrap().message());
        return;
    }

    let mut input = String::new();

    loop {
        print!("gql > ");

        std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!");

        match std::io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(err) => println!("Invalid input {}", err),
        }

        if input.trim() == "exit" {
            println!("Bye (^_*)");
            break;
        }

        let tokenizer_result = tokenizer::tokenize(input.trim().to_string());
        if tokenizer_result.is_err() {
            println!("ERROR: {}", tokenizer_result.err().unwrap());
            return;
        }

        let tokens = tokenizer_result.ok().unwrap();
        let parser_result = parser::parse_gql(tokens);
        if parser_result.is_err() {
            println!("ERROR: {}", parser_result.err().unwrap());
            return;
        }

        let statements = parser_result.ok().unwrap();
        let repo = repository.as_ref().unwrap();
        engine::evaluate(repo, statements);

        input.clear();
    }
}
