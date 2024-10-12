use std::any::Any;

use super::base::DataType;

#[derive(Clone)]
pub struct AnyType;

impl DataType for AnyType {
    fn literal(&self) -> String {
        "Any".to_string()
    }

    fn equals(&self, _other: &Box<dyn DataType>) -> bool {
        true
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
