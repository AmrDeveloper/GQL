use std::any::Any;

use crate::expression::Expr;
use crate::expression::StringExpr;
use crate::types::array::ArrayType;
use crate::types::integer::IntType;

use super::base::DataType;

#[derive(Clone)]
pub struct BoolType;

impl DataType for BoolType {
    fn literal(&self) -> String {
        "Boolean".to_string()
    }

    fn equals(&self, other: &Box<dyn DataType>) -> bool {
        other.is_any() || other.is_bool() || other.is_variant_with(|t| t.is_bool())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn can_perform_bang_op(&self) -> bool {
        true
    }

    fn bang_op_result_type(&self) -> Box<dyn DataType> {
        Box::new(BoolType)
    }

    fn can_perform_logical_or_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(BoolType)]
    }

    fn logical_or_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(self.clone())
    }

    fn can_perform_logical_and_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(BoolType)]
    }

    fn logical_and_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(self.clone())
    }

    fn can_perform_logical_xor_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(BoolType)]
    }

    fn logical_xor_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(self.clone())
    }

    fn can_perform_eq_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(BoolType)]
    }

    fn can_perform_group_eq_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(ArrayType::new(Box::new(BoolType)))]
    }

    fn can_perform_bang_eq_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(BoolType)]
    }

    fn can_perform_group_bang_eq_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(ArrayType::new(Box::new(BoolType)))]
    }

    fn can_perform_gt_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(BoolType)]
    }

    fn can_perform_group_gt_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(ArrayType::new(Box::new(BoolType)))]
    }

    fn can_perform_gte_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(BoolType)]
    }

    fn can_perform_group_gte_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(ArrayType::new(Box::new(BoolType)))]
    }

    fn can_perform_lt_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(BoolType)]
    }

    fn can_perform_group_lt_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(ArrayType::new(Box::new(BoolType)))]
    }

    fn can_perform_lte_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(BoolType)]
    }

    fn can_perform_group_lte_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(ArrayType::new(Box::new(BoolType)))]
    }

    fn not_op_result_type(&self) -> Box<dyn DataType> {
        Box::new(self.clone())
    }

    fn has_implicit_cast_from(&self, expr: &Box<dyn Expr>) -> bool {
        if let Some(string_expr) = expr.as_any().downcast_ref::<StringExpr>() {
            const BOOLEANS_VALUES_LITERAL: [&str; 10] =
                ["t", "true", "y", "yes", "1", "f", "false", "n", "no", "0"];
            return BOOLEANS_VALUES_LITERAL.contains(&string_expr.value.as_str());
        }
        false
    }

    fn can_perform_explicit_cast_op_to(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(IntType)]
    }
}
