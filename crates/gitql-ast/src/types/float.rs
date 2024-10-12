use std::any::Any;

use super::base::DataType;

#[derive(Clone)]
pub struct FloatType;

impl DataType for FloatType {
    fn literal(&self) -> String {
        "Float".to_string()
    }

    fn equals(&self, other: &Box<dyn DataType>) -> bool {
        other.is_any() || other.is_float()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn can_perform_add_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(FloatType)]
    }

    fn add_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(FloatType)
    }

    fn can_perform_sub_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(FloatType)]
    }

    fn sub_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(FloatType)
    }

    fn can_perform_mul_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(FloatType)]
    }

    fn mul_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(FloatType)
    }

    fn can_perform_div_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(FloatType)]
    }

    fn div_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(FloatType)
    }

    fn can_perform_eq_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(FloatType)]
    }

    fn can_perform_bang_eq_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(FloatType)]
    }

    fn can_perform_gt_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(FloatType)]
    }

    fn can_perform_gte_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(FloatType)]
    }

    fn can_perform_lt_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(FloatType)]
    }

    fn can_perform_lte_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(FloatType)]
    }
}
