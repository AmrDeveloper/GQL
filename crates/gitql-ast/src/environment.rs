use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::types::DataType;
use crate::value::Value;

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

#[derive(Default)]
pub struct Environment {
    /// All Global Variables values that can life for this program session
    pub globals: HashMap<String, Value>,
    /// All Global Variables Types that can life for this program session
    pub globals_types: HashMap<String, DataType>,
    /// Local variables types in the current scope, later will be multi layer scopes
    pub scopes: HashMap<String, DataType>,
}

impl Environment {
    /// Define in the current scope
    pub fn define(&mut self, str: String, data_type: DataType) {
        self.scopes.insert(str, data_type);
    }

    /// Define in the global scope
    pub fn define_global(&mut self, str: String, data_type: DataType) {
        self.globals_types.insert(str, data_type);
    }

    /// Returns true if local or global scopes has contains field
    pub fn contains(&self, str: &String) -> bool {
        self.scopes.contains_key(str) || self.globals_types.contains_key(str)
    }

    /// Resolve Global or Local type using symbol name
    pub fn resolve_type(&self, str: &String) -> Option<&DataType> {
        if str.starts_with('@') {
            return self.globals_types.get(str);
        }
        return self.scopes.get(str);
    }

    /// Clear all locals scopes and only save globals
    pub fn clear_session(&mut self) {
        self.scopes.clear()
    }
}
