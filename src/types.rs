use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(PartialEq, Clone)]
pub enum DataType {
    Text,
    Number,
    Boolean,
    Date,
}

impl DataType {
    pub fn literal(&self) -> &'static str {
        return match self {
            DataType::Text => "Text",
            DataType::Number => "Number",
            DataType::Boolean => "Boolean",
            DataType::Date => "Date",
        };
    }
}

lazy_static! {
    pub static ref TABLES_FIELDS_TYPES: HashMap<&'static str, DataType> = {
        let mut map = HashMap::new();
        map.insert("title", DataType::Text);
        map.insert("message", DataType::Text);
        map.insert("name", DataType::Text);
        map.insert("full_name", DataType::Text);
        map.insert("email", DataType::Text);
        map.insert("type", DataType::Text);
        map.insert("time", DataType::Date);
        map.insert("is_head", DataType::Boolean);
        map.insert("is_remote", DataType::Boolean);
        map
    };
}
