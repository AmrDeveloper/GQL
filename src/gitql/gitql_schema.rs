use std::collections::HashMap;
use std::sync::OnceLock;

use gitql_ast::types::boolean::BoolType;
use gitql_ast::types::datetime::DateTimeType;
use gitql_ast::types::integer::IntType;
use gitql_ast::types::text::TextType;
use gitql_ast::types::DataType;

use crate::gitql::types::diff_changes::DiffChangesType;

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
    map.insert("removals", Box::new(IntType));
    map.insert("diff_changes", Box::new(DiffChangesType));
    map.insert("files_changed", Box::new(IntType));
    map.insert("type", Box::new(TextType));
    map.insert("datetime", Box::new(DateTimeType));
    map.insert("is_head", Box::new(BoolType));
    map.insert("is_remote", Box::new(BoolType));
    map.insert("commit_count", Box::new(IntType));
    map.insert("parents_count", Box::new(IntType));
    map.insert("updated", Box::new(DateTimeType));
    map.insert("path", Box::new(TextType));
    map.insert("mode", Box::new(TextType));
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
                "author_name",
                "author_email",
                "insertions",
                "removals",
                "files_changed",
                "diff_changes",
                "datetime",
                "repo",
            ],
        );
        map.insert(
            "diffs_changes",
            vec![
                "commit_id",
                "insertions",
                "removals",
                "mode",
                "path",
                "repo",
            ],
        );
        map.insert("tags", vec!["name", "repo"]);
        map
    })
}
