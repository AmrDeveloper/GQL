use crate::object::{render_objects, GQLObject};
use crate::statement::GQLQuery;

pub fn evaluate(repo: &git2::Repository, query: GQLQuery) {
    let mut objects: Vec<GQLObject> = Vec::new();
    let statements_map = query.statements;

    if statements_map.contains_key("select") {
        let statements = statements_map.get("select").unwrap();
        statements.execute(repo, &mut objects);
    }

    if statements_map.contains_key("where") {
        let statements = statements_map.get("where").unwrap();
        statements.execute(repo, &mut objects);
    }

    if statements_map.contains_key("order") {
        let statements = statements_map.get("order").unwrap();
        statements.execute(repo, &mut objects);
    }

    if statements_map.contains_key("offset") {
        let statements = statements_map.get("offset").unwrap();
        statements.execute(repo, &mut objects);
    }

    if statements_map.contains_key("limit") {
        let statements = statements_map.get("limit").unwrap();
        statements.execute(repo, &mut objects);
    }

    render_objects(&objects);
}
