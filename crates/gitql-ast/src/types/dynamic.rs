use std::any::Any;

use super::base::DataType;

/// A function that resolve a dynamic type depending on a list of already resolved types
pub type ResolveFunction = fn(&[Box<dyn DataType>]) -> Box<dyn DataType>;

#[derive(Clone)]
#[allow(clippy::borrowed_box)]
pub struct DynamicType {
    pub function: ResolveFunction,
}

impl DynamicType {
    #[allow(clippy::type_complexity)]
    pub fn new(function: ResolveFunction) -> Self {
        DynamicType { function }
    }
}

impl DataType for DynamicType {
    fn literal(&self) -> String {
        "Dynamic".to_string()
    }

    fn equals(&self, _other: &Box<dyn DataType>) -> bool {
        false
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
