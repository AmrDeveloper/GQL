use std::any::Any;

use super::base::DataType;

#[derive(Clone)]
pub struct TextType;

impl DataType for TextType {
    fn literal(&self) -> String {
        "Text".to_string()
    }

    fn equals(&self, other: &Box<dyn DataType>) -> bool {
        other.is_any() || other.is_text() || other.is_variant_with(|t| t.is_text())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn can_perform_eq_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(TextType)]
    }

    fn can_perform_bang_eq_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(TextType)]
    }

    fn can_perform_gt_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(TextType)]
    }

    fn can_perform_gte_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(TextType)]
    }

    fn can_perform_lt_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(TextType)]
    }

    fn can_perform_lte_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(TextType)]
    }

    fn can_perform_like_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(TextType)]
    }

    fn can_perform_glob_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(TextType)]
    }

    fn can_perform_regexp_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(TextType)]
    }
}
