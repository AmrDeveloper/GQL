use gitql_ast::types::DataType;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TABLES_FIELDS_TYPES: HashMap<&'static str, DataType> = {
        let mut map = HashMap::new();
        map.insert("commit_id", DataType::Text);
        map.insert("title", DataType::Text);
        map.insert("message", DataType::Text);
        map.insert("name", DataType::Text);
        map.insert("full_name", DataType::Text);
        map.insert("insertions", DataType::Integer);
        map.insert("deletions", DataType::Integer);
        map.insert("files_changed", DataType::Integer);
        map.insert("email", DataType::Text);
        map.insert("type", DataType::Text);
        map.insert("datetime", DataType::DateTime);
        map.insert("is_head", DataType::Boolean);
        map.insert("is_remote", DataType::Boolean);
        map.insert("commit_count", DataType::Integer);
        map.insert("repo", DataType::Text);
        map
    };
}

lazy_static! {
    pub static ref TABLES_FIELDS_NAMES: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("refs", vec!["name", "full_name", "type", "repo"]);
        map.insert(
            "commits",
            vec![
                "commit_id",
                "title",
                "message",
                "name",
                "email",
                "datetime",
                "repo",
            ],
        );
        map.insert(
            "branches",
            vec!["name", "commit_count", "is_head", "is_remote", "repo"],
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
                "repo",
            ],
        );
        map.insert("tags", vec!["name", "repo"]);
        map
    };
}
