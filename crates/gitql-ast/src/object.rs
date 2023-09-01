use std::collections::HashMap;

use crate::value::Value;

#[derive(Clone)]
pub struct GQLObject {
    pub attributes: HashMap<String, Value>,
}
