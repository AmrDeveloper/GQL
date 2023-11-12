use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::hash::Hasher;
use std::vec;

use gitql_ast::object::GQLObject;
use gitql_ast::statement::GQLQuery;
use gitql_ast::statement::SelectStatement;

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

pub struct EvaluationValues {
    pub groups: Vec<Vec<GQLObject>>,
    pub hidden_selections: Vec<std::string::String>,
}

pub fn evaluate(
    repos: &Vec<git2::Repository>,
    query: GQLQuery,
) -> Result<EvaluationValues, String> {
    let mut groups: Vec<Vec<GQLObject>> = Vec::new();
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
                            statement,
                            &repos[0],
                            &mut groups,
                            &mut alias_table,
                            &hidden_selections,
                        )?;
                        continue;
                    }

                    // If table name is not empty, must perform it on each repository
                    for repo in repos {
                        execute_statement(
                            statement,
                            repo,
                            &mut groups,
                            &mut alias_table,
                            &hidden_selections,
                        )?;
                    }

                    // If the main group is empty, no need to perform other statements
                    if groups.is_empty() || groups[0].is_empty() {
                        return Ok(EvaluationValues {
                            groups: vec![],
                            hidden_selections,
                        });
                    }

                    // If Select statement has table name and distinct flag, keep only unique values
                    if !select_statement.table_name.is_empty() && select_statement.is_distinct {
                        apply_distinct_on_objects_group(&mut groups, &hidden_selections);
                    }
                }
                _ => {
                    // Any other statement can be performend on first or non repository
                    execute_statement(
                        statement,
                        first_repo,
                        &mut groups,
                        &mut alias_table,
                        &hidden_selections,
                    )?;
                }
            }
        }
    }

    // If there are many groups that mean group by is executed before.
    // must merge each group into only one element
    if groups.len() > 1 {
        for group in groups.iter_mut() {
            if group.len() > 1 {
                group.drain(1..);
            }
        }
    }
    // If it a single group but it select only aggregations function,
    // should return only first element in the group
    else if groups.len() == 1 && !query.has_group_by_statement && query.has_aggregation_function {
        let group: &mut Vec<GQLObject> = groups[0].as_mut();
        if group.len() > 1 {
            group.drain(1..);
        }
    }

    // Return the groups and hidden selections to be used later in GUI or TUI ...etc
    Ok(EvaluationValues {
        groups: groups.to_owned(),
        hidden_selections,
    })
}

fn apply_distinct_on_objects_group(groups: &mut Vec<Vec<GQLObject>>, hidden_selections: &[String]) {
    if groups.is_empty() {
        return;
    }

    let titles: Vec<&str> = groups[0][0]
        .attributes
        .keys()
        .filter(|s| !hidden_selections.contains(s))
        .map(|k| k.as_ref())
        .collect();

    let titles_count = titles.len();

    let objects = &groups[0];
    let mut new_objects: Vec<GQLObject> = vec![];
    let mut values_set: HashSet<u64> = HashSet::new();

    for object in objects {
        // Build row of the selected only values
        let mut row_values: Vec<String> = Vec::with_capacity(titles_count);
        for key in &titles {
            row_values.push(object.attributes.get(key as &str).unwrap().literal());
        }

        // Compute the hash for row of values
        let mut hash = DefaultHasher::new();
        row_values.hash(&mut hash);
        let values_hash = hash.finish();

        // If this hash is unique, insert the row
        if values_set.insert(values_hash) {
            new_objects.push(object.to_owned());
        }
    }

    // If number of total rows is changed, update the main group rows
    if objects.len() != new_objects.len() {
        groups[0].clear();
        groups[0].append(&mut new_objects);
    }
}
