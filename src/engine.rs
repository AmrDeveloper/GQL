use crate::object::{render_objects, GQLObject};
use crate::statement::GQLQuery;

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

pub fn evaluate(repo: &git2::Repository, query: GQLQuery) {
    let mut objects: Vec<Vec<GQLObject>> = Vec::new();
    let statements_map = query.statements;

    for gql_command in GQL_COMMANDS_IN_ORDER {
        if statements_map.contains_key(gql_command) {
            let statements = statements_map.get(gql_command).unwrap();
            statements.execute(repo, &mut objects);
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

    render_objects(&objects);
}
