use std::collections::HashMap;

use crate::schema::Schema;
use crate::signature::Aggregation;
use crate::signature::Function;
use crate::signature::Signature;
use crate::types::DataType;
use crate::value::Value;

pub struct Environment {
    /// Data schema information contains table, fields names and types
    pub schema: Schema,

    pub std_signatures: HashMap<&'static str, Signature>,
    pub std_functions: HashMap<&'static str, Function>,

    pub aggregation_signatures: HashMap<&'static str, Signature>,
    pub aggregation_functions: HashMap<&'static str, Aggregation>,

    /// All Global Variables values that can life for this program session
    pub globals: HashMap<String, Value>,

    /// All Global Variables Types that can life for this program session
    pub globals_types: HashMap<String, DataType>,

    /// Local variables types in the current scope, later will be multi layer scopes
    pub scopes: HashMap<String, DataType>,
}

impl Environment {
    /// Create new [`Environment`] instance with Data Schema
    pub fn new(schema: Schema) -> Self {
        Self {
            schema,
            std_signatures: HashMap::default(),
            std_functions: HashMap::default(),
            aggregation_signatures: HashMap::default(),
            aggregation_functions: HashMap::default(),
            globals: HashMap::default(),
            globals_types: HashMap::default(),
            scopes: HashMap::default(),
        }
    }

    pub fn with_standard_functions(
        &mut self,
        signatures: &HashMap<&'static str, Signature>,
        functions: &HashMap<&'static str, Function>,
    ) {
        self.std_signatures.extend(signatures.to_owned());
        self.std_functions.extend(functions.to_owned());
    }

    pub fn with_aggregation_functions(
        &mut self,
        signatures: &HashMap<&'static str, Signature>,
        aggregation: &HashMap<&'static str, Aggregation>,
    ) {
        self.aggregation_signatures.extend(signatures.to_owned());
        self.aggregation_functions.extend(aggregation.to_owned());
    }

    pub fn is_std_function(&self, str: &str) -> bool {
        self.std_functions.contains_key(str)
    }

    pub fn std_signature(&self, str: &str) -> Option<&Signature> {
        self.std_signatures.get(str)
    }

    pub fn std_function(&self, str: &str) -> Option<&Function> {
        self.std_functions.get(str)
    }

    pub fn is_aggregation_function(&self, str: &str) -> bool {
        self.aggregation_signatures.contains_key(str)
    }

    pub fn aggregation_signature(&self, str: &str) -> Option<&Signature> {
        self.aggregation_signatures.get(str)
    }

    pub fn aggregation_function(&self, str: &str) -> Option<&Aggregation> {
        self.aggregation_functions.get(str)
    }

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
