use std::any::Any;

use crate::expression::Expr;
use crate::expression::StringExpr;
use crate::format_checker::is_valid_datetime_format;

use super::base::DataType;

#[derive(Clone)]
pub struct DateTimeType;

impl DataType for DateTimeType {
    fn literal(&self) -> String {
        "DateTime".to_string()
    }

    fn equals(&self, other: &Box<dyn DataType>) -> bool {
        other.is_any() || other.is_date_time() || other.is_variant_with(|t| t.is_date_time())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn can_perform_eq_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(DateTimeType)]
    }

    fn can_perform_bang_eq_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(DateTimeType)]
    }

    fn can_perform_gt_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(DateTimeType)]
    }

    fn can_perform_gte_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(DateTimeType)]
    }

    fn can_perform_lt_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(DateTimeType)]
    }

    fn can_perform_lte_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(DateTimeType)]
    }

    fn has_implicit_cast_from(&self, expr: &Box<dyn Expr>) -> bool {
        if let Some(string_expr) = expr.as_any().downcast_ref::<StringExpr>() {
            return is_valid_datetime_format(&string_expr.value);
        }
        false
    }
}
