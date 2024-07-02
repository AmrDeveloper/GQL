use gitql_core::types::DataType;
use std::collections::HashMap;
use std::sync::OnceLock;

pub fn tables_fields_types() -> &'static HashMap<&'static str, DataType> {
    static HASHMAP: OnceLock<HashMap<&'static str, DataType>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut map = HashMap::new();
        map.insert("commit_id", DataType::Text);
        map.insert("title", DataType::Text);
        map.insert("message", DataType::Text);
        map.insert("name", DataType::Text);
        map.insert("author_name", DataType::Text);
        map.insert("author_email", DataType::Text);
        map.insert("committer_name", DataType::Text);
        map.insert("committer_email", DataType::Text);
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
        map.insert("parents_count", DataType::Integer);
        map.insert("updated", DataType::DateTime);
        map.insert("repo", DataType::Text);
        map
    })
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
