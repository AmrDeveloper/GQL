use std::any::Any;

use super::base::DataType;
use super::boolean::BoolType;

#[derive(Clone)]
pub struct RangeType {
    pub base: Box<dyn DataType>,
}

impl RangeType {
    pub fn new(base: Box<dyn DataType>) -> Self {
        RangeType { base }
    }
}

impl DataType for RangeType {
    fn literal(&self) -> String {
        format!("Range({})", self.base.literal())
    }

    fn equals(&self, other: &Box<dyn DataType>) -> bool {
        let range_type: Box<dyn DataType> = Box::new(self.clone());
        if other.is_any() || other.is_variant_contains(&range_type) {
            return true;
        }

        if let Some(other_range) = other.as_any().downcast_ref::<RangeType>() {
            return self.base.equals(&other_range.base);
        }
        false
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn can_perform_contains_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(self.clone()), self.base.clone()]
    }

    fn can_perform_logical_and_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(self.clone())]
    }

    fn logical_and_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(BoolType)
    }
}
