use std::collections::HashMap;

use gitql_ast::types::DataType;

/// A Representation of the Data Schema that constructed the following
///
/// [`tables_fields_names`]  is a map of tables and columns names
///
/// # Examples
///
/// ```
///
/// pub static ref TABLES_FIELDS_NAMES: HashMap<&'static str, Vec<&'static str>> = {
///    let mut map = HashMap::new();
///    map.insert("refs", vec!["name", "full_name", "type", "repo"]);
/// }
///
/// ```
///
/// [`tables_fields_types`] is a map of each column name in general with the expected data type
///
/// # Examples
///
/// ```
/// pub static ref TABLES_FIELDS_TYPES: HashMap<&'static str, Box<dyn DataType>> = {
///    let mut map = HashMap::new();
///    map.insert("commit_id", Box::new(TextType));
/// }
/// ```
///
pub struct Schema {
    pub tables_fields_names: HashMap<&'static str, Vec<&'static str>>,
    pub tables_fields_types: HashMap<&'static str, Box<dyn DataType>>,
}
