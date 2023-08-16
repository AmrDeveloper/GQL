use std::collections::HashMap;

#[derive(Clone)]
pub struct GQLObject {
    pub attributes: HashMap<String, String>,
}
