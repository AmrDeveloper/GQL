use std::any::Any;

use super::base::DataType;

#[derive(Clone)]
pub struct NullType;

impl DataType for NullType {
    fn literal(&self) -> String {
        "Null".to_string()
    }

    fn equals(&self, other: &Box<dyn DataType>) -> bool {
        other.is_any() || other.is_null() || other.is_variant_with(|t| t.is_null())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
