use std::collections::HashMap;

use crate::types::DataType;

pub struct Schema {
    pub tables_fields_names: HashMap<&'static str, Vec<&'static str>>,
    pub tables_fields_types: HashMap<&'static str, DataType>,
}

