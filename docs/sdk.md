The architecture for GitQL designed to enable you to embedded the full engine with all required components and work with dynamic data so for example you can run the SQL query on files, API response, So this design help you to easy create a tool that can run SQL like query on any structured data such as Files, API Response, Logs, Abstract syntax tree ...etc.

## SDK Components
| Component    |                  Description                  |                      Install |
| ------------ | :-------------------------------------------: | ---------------------------: |
| gitql-core   |       Core components Types and Values        |   `cargo install gitql-core` |
| gitql-std    |      Standard and Aggregation functions       |    `cargo install gitql-std` |
| gitql-cli    | CLI components like args parser, cli reporter |    `cargo install gitql-cli` |
| gitql-ast    | structures components such as AST, functions  |    `cargo install gitql-ast` |
| gitql-parser |      Parser and Type checker components       | `cargo install gitql-parser` |
| gitql-engine |          Execution engine component           | `cargo install gitql-engine` |

To use the GitQL SDK with different data you need to define two things `Schema` and `DataProvider` for the data so the SDK know how to load and validate the data.

Note: Most of the times you may don't need to use the `gitql-cli` component and write your own args parser using `clap` or implement your own code.

### Define your own Schema
To allow using GitQL SDK on different data you need to define the data schema so it can be used to validate the symbols and types on the query.

The Schema is just a 2 maps

The tables Fields names map is used to define which tables we expect and what fields each table contains for example for Files schema.

```rs
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TABLES_FIELDS_TYPES: HashMap<&'static str, DataType> = {
        let mut map = HashMap::new();
        map.insert("path", DataType::Text);
        map.insert("parent", DataType::Text);
        map.insert("extension", DataType::Text);
        map.insert("size", DataType::Integer);
        map
    };
}
```

The other map is for types so it define the type of each field on the schema for example for Files schema.

```rs
use gitql_ast::types::DataType;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TABLES_FIELDS_NAMES: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("files", vec!["path", "parent", "extension", "size"]);
        map
    };
}
```

Then create the Schema object from the two maps

```rs
use gitql_ast::schema::Schema;

let schema = Schema {
    tables_fields_names: TABLES_FIELDS_NAMES.to_owned(),
    tables_fields_types: TABLES_FIELDS_TYPES.to_owned(),
};
```

### Define your own DataProvider

The DataProvider is a simple component that used to load any kind of data and map them to the GitQLObject so the engine can deal with it, you should implement the `DataProvider` trait for your data and can work with one or more data sources but make sure that your schema matches the data, for example to work with Files.

```rs
use std::path::Path;

use gitql_ast::environment::Environment;
use gitql_ast::expression::Expression;
use gitql_ast::expression::SymbolExpression;
use gitql_ast::object::GitQLObject;
use gitql_ast::object::Group;
use gitql_ast::object::Row;
use gitql_ast::value::Value;
use gitql_engine::data_provider::DataProvider;
use gitql_engine::engine_evaluator::evaluate_expression;

pub struct FileDataProvider {
    pub base_path: String,
}

impl FileDataProvider {
    pub fn new(base_path: String) -> Self {
        Self { base_path }
    }
}

impl DataProvider for FileDataProvider {
    fn provide(
        &self,
        env: &mut Environment,
        _table: &str,
        fields_names: &[String],
        titles: &[String],
        fields_values: &[Box<dyn Expression>],
    ) -> GitQLObject {
        let files = traverse_file_path(&self.base_path);
        let mut groups: Vec<Group> = vec![];
        let mut rows: Vec<Row> = vec![];

        let names_len = fields_names.len() as i64;
        let values_len = fields_values.len() as i64;
        let padding = names_len - values_len;

        for file in files {
            let mut values: Vec<Value> = vec![];

            for index in 0..names_len {
                let field_name = &fields_names[index as usize];

                if (index - padding) >= 0 {
                    let value = &fields_values[(index - padding) as usize];
                    if value.as_any().downcast_ref::<SymbolExpression>().is_none() {
                        let evaluated = evaluate_expression(env, value, titles, &values);
                        values.push(evaluated.unwrap_or(Value::Null));
                        continue;
                    }
                }

                if field_name == "path" {
                    let path = Path::new(&file);
                    let file_path_string = path.to_str().unwrap_or("");
                    values.push(Value::Text(file_path_string.to_string()));
                    continue;
                }

                if field_name == "parent" {
                    let path = Path::new(&file);
                    let parent_path = if let Some(parent) = path.parent() {
                        parent.to_str().unwrap_or("")
                    } else {
                        ""
                    };
                    values.push(Value::Text(parent_path.to_string()));
                    continue;
                }

                if field_name == "extension" {
                    let path = Path::new(&file);
                    let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
                    values.push(Value::Text(extension.to_string()));
                    continue;
                }

                if field_name == "size" {
                    let file_size = if let Ok(meta_data) = std::fs::metadata(&file) {
                        meta_data.len() as i64
                    } else {
                        0
                    };
                    values.push(Value::Integer(file_size));
                    continue;
                }

                values.push(Value::Null);
            }

            rows.push(Row { values });
        }

        groups.push(Group { rows });
        GitQLObject {
            titles: titles.to_vec(),
            groups,
        }
    }
}

fn traverse_file_path(dir_path: &String) -> Vec<String> {
    let mut file_paths = Vec::new();
    let mut stack: Vec<String> = vec![dir_path.clone()];

    while let Some(path) = stack.pop() {
        if let Ok(entries) = std::fs::read_dir(&path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_type = entry.file_type().unwrap();
                    let subpath = entry.path();

                    if file_type.is_dir() {
                        stack.push(subpath.to_str().unwrap_or("").to_string());
                    } else {
                        if let Some(file_path) = subpath.to_str() {
                            file_paths.push(file_path.to_string());
                        }
                    }
                }
            }
        }
    }

    file_paths
}
```

Now you have the Schema and DataProvider it's time to config the other SDK Components.

Note: that the path and query can be come from command line arguments or passed by your GUI app.

```rs
let base_path = ...;
let query = ...;

let schema = Schema {
    tables_fields_names: TABLES_FIELDS_NAMES.to_owned(),
    tables_fields_types: TABLES_FIELDS_TYPES.to_owned(),
};

// Register default standard and aggregation function or add your own with modifications
let std_signatures = standard_function_signatures();
let std_functions = standard_functions();

let aggregation_signatures = aggregation_function_signatures();
let aggregation_functions = aggregation_functions();

let mut env = Environment::new(schema);
env.with_standard_functions(std_signatures, std_functions);
env.with_aggregation_functions(aggregation_signatures, aggregation_functions);

let mut reporter = DiagnosticReporter::default();
let tokenizer_result = tokenizer::tokenize(query.to_owned());
let tokens = tokenizer_result.ok().unwrap();
if tokens.is_empty() {
    return;
}

let parser_result = parser::parse_gql(tokens, &mut env);
if parser_result.is_err() {
    let diagnostic = parser_result.err().unwrap();
    reporter.report_diagnostic(&query, *diagnostic);
    return;
}

let query_node = parser_result.ok().unwrap();
let provider: Box<dyn DataProvider> = Box::new(FileDataProvider::new(base_path.to_owned()));
let evaluation_result = engine::evaluate(&mut env, &provider, query_node);

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
    match format {
        OutputFormat::Render => {
            render::render_objects(&mut groups, &hidden_selection, pagination, page_size);
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
```

Note: render the result as table, json or csv not the only option you can send them using API or send them to GUI.