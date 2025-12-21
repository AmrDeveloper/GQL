use std::collections::HashMap;

use gitql_ast::types::boolean::BoolType;
use gitql_ast::types::date::DateType;
use gitql_ast::types::datetime::DateTimeType;
use gitql_ast::types::float::FloatType;
use gitql_ast::types::integer::IntType;
use gitql_ast::types::interval::IntervalType;
use gitql_ast::types::text::TextType;
use gitql_ast::types::time::TimeType;
use gitql_ast::types::DataType;

/// Map of Types and Names to be used in type parser
pub struct TypesTable {
    /// Collection of type names mapped to actual types
    types_map: HashMap<&'static str, Box<dyn DataType>>,
}

impl Default for TypesTable {
    fn default() -> Self {
        Self::new()
    }
}

impl TypesTable {
    /// Create new Instance of [`TypesTable`] with SQL primitives types registered
    pub fn new() -> Self {
        let mut types_table = TypesTable {
            types_map: HashMap::default(),
        };
        register_primitives_types(&mut types_table.types_map);
        types_table
    }

    /// Create new Instance of [`TypesTable`] with empty map
    pub fn empty() -> Self {
        TypesTable {
            types_map: HashMap::default(),
        }
    }

    /// Register DataType to a new name and return optional if it success or not
    pub fn register(
        &mut self,
        name: &'static str,
        data_type: Box<dyn DataType>,
    ) -> Option<Box<dyn DataType>> {
        self.types_map.insert(name, data_type)
    }

    /// Lookup at the types map by name and return DataType if registered or None if not found
    pub fn lookup(&self, name: &str) -> Option<Box<dyn DataType>> {
        self.types_map.get(&name).cloned()
    }

    /// Return Reference to the current type map
    pub fn types_map(&self) -> &HashMap<&'static str, Box<dyn DataType>> {
        &self.types_map
    }

    /// Returns true if the map contains no elements.
    pub fn is_empty(&self) -> bool {
        self.types_map.is_empty()
    }

    /// Return the length of types map
    pub fn len(&self) -> usize {
        self.types_map.len()
    }
}

/// Register the common predefined Types in SQL with their common aliases
fn register_primitives_types(types_map: &mut HashMap<&'static str, Box<dyn DataType>>) {
    // SQL Data Types
    types_map.insert("integer", Box::new(IntType));
    types_map.insert("real", Box::new(FloatType));
    types_map.insert("boolean", Box::new(BoolType));
    types_map.insert("text", Box::new(TextType));
    types_map.insert("date", Box::new(DateType));
    types_map.insert("time", Box::new(TimeType));
    types_map.insert("datetime", Box::new(DateTimeType));
    types_map.insert("interval", Box::new(IntervalType));

    // SQL Type Aliases
    types_map.insert("int", Box::new(IntType));
    types_map.insert("float", Box::new(FloatType));
    types_map.insert("bool", Box::new(BoolType));
}
