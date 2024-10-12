use std::any::Any;

use super::base::DataType;

#[derive(Clone)]
pub struct UndefType;

impl DataType for UndefType {
    fn literal(&self) -> String {
        "Undef".to_string()
    }

    fn equals(&self, other: &Box<dyn DataType>) -> bool {
        other.as_any().downcast_ref::<UndefType>().is_some()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
