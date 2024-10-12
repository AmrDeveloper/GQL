use std::any::Any;

use super::base::DataType;

#[derive(Clone)]
pub struct DynamicType {
    pub function: fn(&[Box<dyn DataType>]) -> Box<dyn DataType>,
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
