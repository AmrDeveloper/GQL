use std::any::Any;

use crate::types::{boolean::BoolType, float::FloatType};

use super::base::DataType;

#[derive(Clone)]
pub struct IntType;

impl DataType for IntType {
    fn literal(&self) -> String {
        "Int".to_string()
    }

    fn equals(&self, other: &Box<dyn DataType>) -> bool {
        other.is_any() || other.is_int() || other.is_variant_with(|t| t.is_int())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn can_perform_add_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(IntType)]
    }

    fn add_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(IntType)
    }

    fn can_perform_sub_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(IntType)]
    }

    fn sub_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(IntType)
    }

    fn can_perform_mul_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(IntType)]
    }

    fn mul_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(IntType)
    }

    fn can_perform_div_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(IntType)]
    }

    fn div_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(IntType)
    }

    fn can_perform_rem_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(IntType)]
    }

    fn rem_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(IntType)
    }

    fn can_perform_caret_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(IntType)]
    }

    fn caret_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(IntType)
    }

    fn can_perform_or_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(IntType)]
    }

    fn or_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(self.clone())
    }

    fn can_perform_and_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(IntType)]
    }

    fn and_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(self.clone())
    }

    fn can_perform_xor_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(IntType)]
    }

    fn xor_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(self.clone())
    }

    fn can_perform_shl_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(IntType)]
    }

    fn shl_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(self.clone())
    }

    fn can_perform_shr_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(IntType)]
    }

    fn shr_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(self.clone())
    }

    fn can_perform_eq_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(IntType)]
    }

    fn can_perform_bang_eq_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(IntType)]
    }

    fn can_perform_gt_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(IntType)]
    }

    fn can_perform_gte_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(IntType)]
    }

    fn can_perform_lt_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(IntType)]
    }

    fn can_perform_lte_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(IntType)]
    }

    fn can_perform_neg_op(&self) -> bool {
        true
    }

    fn neg_op_result_type(&self) -> Box<dyn DataType> {
        Box::new(self.clone())
    }

    fn can_perform_explicit_cast_op_to(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(FloatType), Box::new(BoolType)]
    }
}
