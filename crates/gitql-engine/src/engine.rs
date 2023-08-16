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
    repo: &git2::Repository,
    query: GQLQuery,
) -> (Vec<Vec<GQLObject>>, Vec<std::string::String>) {
    let mut objects: Vec<Vec<GQLObject>> = Vec::new();
    let statements_map = query.statements;

    for gql_command in GQL_COMMANDS_IN_ORDER {
        if statements_map.contains_key(gql_command) {
            let statement = statements_map.get(gql_command).unwrap();
            execute_statement(&statement, repo, &mut objects);
        }
    }

    // If there are many groups that mean group by is executed before.
    // must merge each group into only one element
    if objects.len() > 1 {
        for group in objects.iter_mut() {
            group.drain(1..);
        }
    }
    // If it a single group but it select only aggregations function,
    // should return only first element in the group
    else if objects.len() == 1 && query.select_aggregations_only {
        let group: &mut Vec<GQLObject> = objects[0].as_mut();
        group.drain(1..);
    }

    // Return the groups and hidden selections to be used later in GUI or TUI ...etc
    return (objects.to_owned(), query.hidden_selections.to_owned());
}
