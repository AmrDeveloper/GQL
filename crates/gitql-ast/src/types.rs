use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(PartialEq, Clone)]
pub enum DataType {
    Any,
    Text,
    Integer,
    Float,
    Boolean,
    Date,
    Time,
    DateTime,
    Undefined,
    Null,
}

impl DataType {
    pub fn is_type(&self, data_type: DataType) -> bool {
        return *self == data_type;
    }

    pub fn is_int(&self) -> bool {
        return self.is_type(DataType::Integer);
    }

    pub fn is_float(&self) -> bool {
        return self.is_type(DataType::Float);
    }

    pub fn is_number(&self) -> bool {
        return self.is_int() || self.is_float();
    }

    pub fn literal(&self) -> &'static str {
        return match self {
            DataType::Any => "Any",
            DataType::Text => "Text",
            DataType::Integer => "Integer",
            DataType::Float => "Float",
            DataType::Boolean => "Boolean",
            DataType::Date => "Date",
            DataType::Time => "Time",
            DataType::DateTime => "DateTime",
            DataType::Undefined => "Undefined",
            DataType::Null => "Null",
        };
    }
}

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
        map.insert("time", DataType::DateTime);
        map.insert("is_head", DataType::Boolean);
        map.insert("is_remote", DataType::Boolean);
        map.insert("commit_count", DataType::Integer);
        map.insert("repo", DataType::Text);
        map
    };
}
