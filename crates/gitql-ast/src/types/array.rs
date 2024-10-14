use std::any::Any;

use super::base::DataType;
use super::boolean::BoolType;
use super::integer::IntType;

#[derive(Clone)]
pub struct ArrayType {
    pub base: Box<dyn DataType>,
}

impl DataType for ArrayType {
    fn literal(&self) -> String {
        format!("Array({})", self.base.literal())
    }

    fn equals(&self, other: &Box<dyn DataType>) -> bool {
        let array_type: Box<dyn DataType> = Box::new(self.clone());
        if other.is_any() || other.is_variant_contains(&array_type) {
            return true;
        }

        if let Some(other_array) = other.as_any().downcast_ref::<ArrayType>() {
            return self.equals(&other_array.base);
        }
        false
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn can_perform_index_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(IntType)]
    }

    fn index_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        self.base.clone()
    }

    fn can_perform_slice_op(&self) -> bool {
        true
    }

    fn can_perform_slice_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(IntType)]
    }

    fn slice_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(self.clone())
    }

    fn can_perform_contains_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(self.clone()), self.base.clone()]
    }

    fn contains_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(BoolType)
    }

    fn can_perform_logical_or_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(self.clone())]
    }

    fn logical_or_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(BoolType)
    }
}
