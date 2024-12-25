use gitql_core::environment::Environment;
use gitql_core::schema::Schema;
use gitql_schema::tables_fields_names;
use gitql_schema::tables_fields_types;
use gitql_std::aggregation::aggregation_function_signatures;
use gitql_std::aggregation::aggregation_functions;
use gitql_std::window::window_function_signatures;
use gitql_std::window::window_functions;

pub(crate) mod functions;
pub(crate) mod gitql_data_provider;
pub(crate) mod gitql_line_editor;
pub(crate) mod gitql_schema;
pub(crate) mod types;
pub(crate) mod values;

pub(crate) fn create_gitql_environment() -> Environment {
    let schema = Schema {
        tables_fields_names: tables_fields_names().to_owned(),
        tables_fields_types: tables_fields_types().to_owned(),
    };

    let std_signatures = functions::gitql_std_signatures();
    let std_functions = functions::gitql_std_functions();

    let aggregation_signatures = aggregation_function_signatures();
    let aggregation_functions = aggregation_functions();

    let window_signatures = window_function_signatures();
    let window_function = window_functions();

    let mut env = Environment::new(schema);
    env.with_standard_functions(&std_signatures, std_functions);
    env.with_aggregation_functions(&aggregation_signatures, aggregation_functions);
    env.with_window_functions(&window_signatures, window_function);
    env
}

pub(crate) fn validate_git_repositories(
    repositories: &Vec<String>,
) -> Result<Vec<gix::Repository>, String> {
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
