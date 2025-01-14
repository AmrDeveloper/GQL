use std::any::Any;

use super::base::DataType;

#[derive(Clone)]
pub struct IntervalType;

impl DataType for IntervalType {
    fn literal(&self) -> String {
        "Interval".to_string()
    }

    fn equals(&self, other: &Box<dyn DataType>) -> bool {
        other.is_any() || other.is_interval() || other.is_variant_with(|t| t.is_interval())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn can_perform_add_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(IntervalType)]
    }

    fn add_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(IntervalType)
    }
}
