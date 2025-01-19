use std::any::Any;

use crate::types::integer::IntType;

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

    fn can_perform_sub_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(IntervalType)]
    }

    fn sub_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(IntervalType)
    }

    fn can_perform_mul_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(IntType)]
    }

    fn mul_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(IntervalType)
    }

    fn can_perform_div_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(IntType)]
    }

    fn div_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(IntervalType)
    }

    fn can_perform_eq_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(IntervalType)]
    }

    fn can_perform_bang_eq_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(IntervalType)]
    }

    fn can_perform_gt_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(IntervalType)]
    }

    fn can_perform_gte_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(IntervalType)]
    }

    fn can_perform_lt_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(IntervalType)]
    }

    fn can_perform_lte_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(IntervalType)]
    }
}
