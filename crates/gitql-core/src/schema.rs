use std::collections::HashMap;

use gitql_ast::types::DataType;

/// A Representation of the Schema of the data including columns, tables and types
pub struct Schema {
    pub tables_fields_names: HashMap<&'static str, Vec<&'static str>>,
    pub tables_fields_types: HashMap<&'static str, Box<dyn DataType>>,
}
