use std::collections::HashMap;
use std::sync::OnceLock;

use gitql_ast::types::base::DataType;
use gitql_ast::types::boolean::BoolType;
use gitql_ast::types::date::DateType;
use gitql_ast::types::integer::IntType;
use gitql_ast::types::text::TextType;

pub fn tables_fields_types() -> HashMap<&'static str, Box<dyn DataType>> {
    let mut map: HashMap<&'static str, Box<dyn DataType>> = HashMap::new();
    map.insert("commit_id", Box::new(TextType));
    map.insert("title", Box::new(TextType));
    map.insert("message", Box::new(TextType));
    map.insert("name", Box::new(TextType));
    map.insert("author_name", Box::new(TextType));
    map.insert("author_email", Box::new(TextType));
    map.insert("committer_name", Box::new(TextType));
    map.insert("committer_email", Box::new(TextType));
    map.insert("full_name", Box::new(TextType));
    map.insert("insertions", Box::new(IntType));
    map.insert("deletions", Box::new(IntType));
    map.insert("files_changed", Box::new(IntType));
    map.insert("email", Box::new(TextType));
    map.insert("type", Box::new(TextType));
    map.insert("datetime", Box::new(DateType));
    map.insert("is_head", Box::new(BoolType));
    map.insert("is_remote", Box::new(BoolType));
    map.insert("commit_count", Box::new(IntType));
    map.insert("parents_count", Box::new(IntType));
    map.insert("updated", Box::new(DateType));
    map.insert("repo", Box::new(TextType));
    map
}

pub fn tables_fields_names() -> &'static HashMap<&'static str, Vec<&'static str>> {
    static HASHMAP: OnceLock<HashMap<&'static str, Vec<&'static str>>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut map = HashMap::new();
        map.insert("refs", vec!["name", "full_name", "type", "repo"]);
        map.insert(
            "commits",
            vec![
                "commit_id",
                "title",
                "message",
                "author_name",
                "author_email",
                "committer_name",
                "committer_email",
                "datetime",
                "parents_count",
                "repo",
            ],
        );
        map.insert(
            "branches",
            vec![
                "name",
                "commit_count",
                "is_head",
                "is_remote",
                "updated",
                "repo",
            ],
        );
        map.insert(
            "diffs",
            vec![
                "commit_id",
                "name",
                "email",
                "insertions",
                "deletions",
                "files_changed",
                "datetime",
                "repo",
            ],
        );
        map.insert("tags", vec!["name", "repo"]);
        map
    })
}
