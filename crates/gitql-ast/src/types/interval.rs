use std::any::Any;

use super::base::DataType;

#[derive(Clone)]
pub struct IntervalType;

impl DataType for IntervalType {
    fn literal(&self) -> String {
        "Interval".to_string()
    }

    fn equals(&self, other: &Box<dyn DataType>) -> bool {
        other.is_any() || other.is_int() || other.is_variant_with(|t| t.is_interval())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
