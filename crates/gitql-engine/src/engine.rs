use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::hash::Hasher;
use std::vec;

use gitql_ast::environment::Environment;
use gitql_ast::object::GitQLObject;
use gitql_ast::object::Group;
use gitql_ast::object::Row;
use gitql_ast::statement::GQLQuery;
use gitql_ast::statement::Query;
use gitql_ast::statement::SelectStatement;

use crate::engine_executor::execute_global_variable_statement;
use crate::engine_executor::execute_statement;

const GQL_COMMANDS_IN_ORDER: [&str; 8] = [
    "select",
    "where",
    "group",
    "aggregation",
    "having",
    "order",
    "offset",
    "limit",
];

pub enum EvaluationResult {
    SelectedGroups(GitQLObject, Vec<std::string::String>),
    SetGlobalVariable,
}

pub fn evaluate(
    env: &mut Environment,
    repos: &[gix::Repository],
    query: Query,
) -> Result<EvaluationResult, String> {
    match query {
        Query::Select(gql_query) => evaluate_select_query(env, repos, gql_query),
        Query::GlobalVariableDeclaration(global_variable) => {
            execute_global_variable_statement(env, &global_variable)?;
            Ok(EvaluationResult::SetGlobalVariable)
        }
    }
}

pub fn evaluate_select_query(
    env: &mut Environment,
    repos: &[gix::Repository],
    query: GQLQuery,
) -> Result<EvaluationResult, String> {
    let mut gitql_object = GitQLObject::default();
    let mut alias_table: HashMap<String, String> = HashMap::new();

    let hidden_selections = query.hidden_selections;
    let mut statements_map = query.statements;
    let first_repo = repos.first().unwrap();

    for gql_command in GQL_COMMANDS_IN_ORDER {
        if statements_map.contains_key(gql_command) {
            let statement = statements_map.get_mut(gql_command).unwrap();

            match gql_command {
                "select" => {
                    // Select statement should be performed on all repositories, can be executed in parallel
                    let select_statement = statement
                        .as_any()
                        .downcast_ref::<SelectStatement>()
                        .unwrap();

                    // If table name is empty no need to perform it on each repository
                    if select_statement.table_name.is_empty() {
                        execute_statement(
                            env,
                            statement,
                            &repos[0],
                            &mut gitql_object,
                            &mut alias_table,
                            &hidden_selections,
                        )?;

                        // If the main group is empty, no need to perform other statements
                        if gitql_object.is_empty() || gitql_object.groups[0].is_empty() {
                            return Ok(EvaluationResult::SelectedGroups(
                                gitql_object,
                                hidden_selections,
                            ));
                        }

                        continue;
                    }

                    // If table name is not empty, must perform it on each repository
                    for repo in repos {
                        execute_statement(
                            env,
                            statement,
                            repo,
                            &mut gitql_object,
                            &mut alias_table,
                            &hidden_selections,
                        )?;
                    }

                    // If the main group is empty, no need to perform other statements
                    if gitql_object.is_empty() || gitql_object.groups[0].is_empty() {
                        return Ok(EvaluationResult::SelectedGroups(
                            gitql_object,
                            hidden_selections,
                        ));
                    }

                    // If Select statement has table name and distinct flag, keep only unique values
                    if !select_statement.table_name.is_empty() && select_statement.is_distinct {
                        apply_distinct_on_objects_group(&mut gitql_object, &hidden_selections);
                    }
                }
                _ => {
                    // Any other statement can be performed on first or non repository
                    execute_statement(
                        env,
                        statement,
                        first_repo,
                        &mut gitql_object,
                        &mut alias_table,
                        &hidden_selections,
                    )?;
                }
            }
        }
    }

    // If there are many groups that mean group by is executed before.
    // must merge each group into only one element
    if gitql_object.len() > 1 {
        for group in gitql_object.groups.iter_mut() {
            if group.len() > 1 {
                group.rows.drain(1..);
            }
        }
    }
    // If it a single group but it select only aggregations function,
    // should return only first element in the group
    else if gitql_object.len() == 1
        && !query.has_group_by_statement
        && query.has_aggregation_function
    {
        let group: &mut Group = &mut gitql_object.groups[0];
        if group.len() > 1 {
            group.rows.drain(1..);
        }
    }

    // Return the groups and hidden selections to be used later in GUI or TUI ...etc
    Ok(EvaluationResult::SelectedGroups(
        gitql_object,
        hidden_selections,
    ))
}

fn apply_distinct_on_objects_group(gitql_object: &mut GitQLObject, hidden_selections: &[String]) {
    if gitql_object.is_empty() {
        return;
    }

    let titles: Vec<&String> = gitql_object
        .titles
        .iter()
        .filter(|s| !hidden_selections.contains(s))
        .collect();

    let titles_count = titles.len();

    let objects = &gitql_object.groups[0].rows;
    let mut new_objects: Group = Group { rows: vec![] };
    let mut values_set: HashSet<u64> = HashSet::new();

    for object in objects {
        // Build row of the selected only values
        let mut row_values: Vec<String> = Vec::with_capacity(titles_count);
        for index in 0..titles.len() {
            row_values.push(object.values.get(index).unwrap().to_string());
        }

        // Compute the hash for row of values
        let mut hash = DefaultHasher::new();
        row_values.hash(&mut hash);
        let values_hash = hash.finish();

        // If this hash is unique, insert the row
        if values_set.insert(values_hash) {
            new_objects.rows.push(Row {
                values: object.values.clone(),
            });
        }
    }

    // If number of total rows is changed, update the main group rows
    if objects.len() != new_objects.len() {
        gitql_object.groups[0].rows.clear();
        gitql_object.groups[0].rows.append(&mut new_objects.rows);
    }
}
