use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::types::DataType;

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
                "time",
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

pub struct Scope {
    pub env: HashMap<String, DataType>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            env: HashMap::new(),
        }
    }

    pub fn define(&mut self, str: String, data_type: DataType) {
        self.env.insert(str, data_type);
    }

    pub fn contains(&self, str: &String) -> bool {
        self.env.contains_key(str)
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}
