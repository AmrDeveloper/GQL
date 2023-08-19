use gitql_ast::object::GQLObject;
use gitql_ast::statement::GQLQuery;

use crate::engine_executor::execute_statement;

const GQL_COMMANDS_IN_ORDER: [&'static str; 8] = [
    "select",
    "where",
    "group",
    "aggregation",
    "having",
    "order",
    "offset",
    "limit",
];

pub fn evaluate(
    repos: &Vec<git2::Repository>,
    query: GQLQuery,
) -> (Vec<Vec<GQLObject>>, Vec<std::string::String>) {
    let mut groups: Vec<Vec<GQLObject>> = Vec::new();
    let statements_map = query.statements;
    let first_repo = repos.first().unwrap();

    for gql_command in GQL_COMMANDS_IN_ORDER {
        if statements_map.contains_key(gql_command) {
            let statement = statements_map.get(gql_command).unwrap();
            match gql_command {
                "select" => {
                    // Select statement should be performed on all repositories, can be executed in parallel
                    for repo in repos {
                        execute_statement(&statement, repo, &mut groups);
                    }
                }
                _ => {
                    // Any other statement can be performend on first or non repository
                    execute_statement(&statement, first_repo, &mut groups);
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
    else if groups.len() == 1 && query.select_aggregations_only {
        let group: &mut Vec<GQLObject> = groups[0].as_mut();
        group.drain(1..);
    }

    // Return the groups and hidden selections to be used later in GUI or TUI ...etc
    return (groups.to_owned(), query.hidden_selections.to_owned());
}
