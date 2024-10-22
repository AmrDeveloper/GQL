Now after we creating our `Schema`, `DataProvider`, may or may not add custom functions, types and values, it's time to make SDK components work together

```rust linenums="1"
// Create instance of your Schema
let schema = Schema {
    tables_fields_names: tables_fields_names().clone(),
    tables_fields_types: tables_fields_types().clone(),
};

// Pass  the standard functions, or your custom functions or mix of them to the env
let std_signatures = standard_functions();
let std_functions = standard_function_signatures();

let aggregation_signatures = aggregation_function_signatures();
let aggregation_functions = aggregation_functions();

let mut env = Environment::new(schema);
env.with_standard_functions(&std_signatures, std_functions);
env.with_aggregation_functions(&aggregation_signatures, aggregation_functions);

// Create instance of the diagnostic reporter, to report errors, warns ...etc
let mut reporter = DiagnosticReporter::default();

// Pass the query to the tokenizer to get List of tokens or error
let tokensOrError = tokenizer::tokenize(query.clone());

// If tokenizer return error, report it and stop
if tokensOrError.is_err() {
    let diagnostic = tokensOrError.err().unwrap();
    reporter.report_diagnostic(&query, *diagnostic);
    return;
}

// Get the list of tokens
let tokens = tokensOrError.ok().unwrap();

// Start the parser to get AST or error
let astOrError = parser::parse_gql(tokens, env);

// Same like tokenizer if it return error, report it and stop
if astOrError.is_err() {
    let diagnostic = astOrError.err().unwrap();
    reporter.report_diagnostic(&query, *diagnostic);
    return;
}

let query_ast = astOrError.ok().unwrap();

// Create instance of your data provider
let provider: Box<dyn DataProvider> = Box::new(FileDataProvider::new(repos.to_vec()));

// Pass the ast and provider to the execution engine to get result or error
let evaluation_result = engine::evaluate(env, &provider, query_node);

// Report Runtime exceptions if they exists
if evaluation_result.is_err() {
    reporter.report_diagnostic(
        &query,
        Diagnostic::exception(&evaluation_result.err().unwrap()),
    );
    return;
}

let execution_result = evaluation_result.ok().unwrap();

// When you get result with selected groups, you can print them like table, json, csv or your custom format
if let SelectedGroups(mut groups) = engine_result {
    let pagination = true;
    let page_size = 10;
    let printer = Box::new(TablePrinter::new(pagination, page_size));
    printer.print(&mut groups);
}
```

Thats it, now you can create a customizable query language with your own schema, data, types and functions.

Enjoy.