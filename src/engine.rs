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
    let mut objects: Vec<GQLObject> = Vec::new();
    let statements_map = query.statements;

    for gql_command in GQL_COMMANDS_IN_ORDER {
        if statements_map.contains_key(gql_command) {
            let statements = statements_map.get(gql_command).unwrap();
            statements.execute(repo, &mut objects);
        }
    }

    render_objects(&objects);
}
