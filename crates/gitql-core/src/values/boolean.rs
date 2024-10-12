use std::any::Any;
use std::cmp::Ordering;

use gitql_ast::types::base::DataType;
use gitql_ast::types::boolean::BoolType;

use super::base::Value;

#[derive(Clone)]
pub struct BoolValue {
    pub value: bool,
}

impl Value for BoolValue {
    fn literal(&self) -> String {
        self.value.to_string()
    }

    fn equals(&self, other: &Box<dyn Value>) -> bool {
        if let Some(other_bool) = other.as_any().downcast_ref::<BoolValue>() {
            return self.value == other_bool.value;
        }
        false
    }

    fn compare(&self, other: &Box<dyn Value>) -> Option<Ordering> {
        if let Some(other_bool) = other.as_any().downcast_ref::<BoolValue>() {
            return self.value.partial_cmp(&other_bool.value);
        }
        None
    }

    fn data_type(&self) -> Box<dyn DataType> {
        Box::new(BoolType)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn perform_logical_or_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_bool) = other.as_any().downcast_ref::<BoolValue>() {
            let value = self.value || other_bool.value;
            return Ok(Box::new(BoolValue { value }));
        }
        Err("Unexpected type to perform `||` with".to_string())
    }

    fn perform_logical_and_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_bool) = other.as_any().downcast_ref::<BoolValue>() {
            let value = self.value && other_bool.value;
            return Ok(Box::new(BoolValue { value }));
        }
        Err("Unexpected type to perform `&&` with".to_string())
    }

    fn perform_logical_xor_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_bool) = other.as_any().downcast_ref::<BoolValue>() {
            let value = self.value ^ other_bool.value;
            return Ok(Box::new(BoolValue { value }));
        }
        Err("Unexpected type to perform `^` with".to_string())
    }

    fn perform_eq_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_bool) = other.as_any().downcast_ref::<BoolValue>() {
            let value = self.value == other_bool.value;
            return Ok(Box::new(BoolValue { value }));
        }
        Err("Unexpected type to perform `==` with".to_string())
    }

    fn perform_bang_eq_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_bool) = other.as_any().downcast_ref::<BoolValue>() {
            let value = self.value != other_bool.value;
            return Ok(Box::new(BoolValue { value }));
        }
        Err("Unexpected type to perform `!=` with".to_string())
    }

    fn perform_gt_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_bool) = other.as_any().downcast_ref::<BoolValue>() {
            let value = self.value & !other_bool.value;
            return Ok(Box::new(BoolValue { value }));
        }
        Err("Unexpected type to perform `>` with".to_string())
    }

    fn perform_gte_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_bool) = other.as_any().downcast_ref::<BoolValue>() {
            let value = self.value >= other_bool.value;
            return Ok(Box::new(BoolValue { value }));
        }
        Err("Unexpected type to perform `>=` with".to_string())
    }

    fn perform_lt_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_bool) = other.as_any().downcast_ref::<BoolValue>() {
            let value = !self.value & other_bool.value;
            return Ok(Box::new(BoolValue { value }));
        }
        Err("Unexpected type to perform `<` with".to_string())
    }

    fn perform_lte_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_bool) = other.as_any().downcast_ref::<BoolValue>() {
            let value = self.value <= other_bool.value;
            return Ok(Box::new(BoolValue { value }));
        }
        Err("Unexpected type to perform `<=` with".to_string())
    }

    fn perform_not_op(&self) -> Result<Box<dyn Value>, String> {
        Ok(Box::new(BoolValue { value: !self.value }))
    }
}
