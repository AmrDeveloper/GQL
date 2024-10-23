use std::any::Any;
use std::cmp::Ordering;

use gitql_ast::types::base::DataType;
use gitql_ast::types::float::FloatType;

use super::base::Value;

#[derive(Clone)]
pub struct FloatValue {
    pub value: f64,
}

impl Value for FloatValue {
    fn literal(&self) -> String {
        self.value.to_string()
    }

    fn equals(&self, other: &Box<dyn Value>) -> bool {
        if let Some(other_float) = other.as_any().downcast_ref::<FloatValue>() {
            return self.value == other_float.value;
        }
        false
    }

    fn compare(&self, other: &Box<dyn Value>) -> Option<Ordering> {
        if let Some(other_float) = other.as_any().downcast_ref::<FloatValue>() {
            return self.value.partial_cmp(&other_float.value);
        }
        None
    }

    fn data_type(&self) -> Box<dyn DataType> {
        Box::new(FloatType)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn add_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_int) = other.as_any().downcast_ref::<FloatValue>() {
            let value = self.value + other_int.value;
            return Ok(Box::new(FloatValue { value }));
        }
        Err("Unexpected value to perform `+` with".to_string())
    }

    fn sub_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_int) = other.as_any().downcast_ref::<FloatValue>() {
            let value = self.value - other_int.value;
            return Ok(Box::new(FloatValue { value }));
        }
        Err("Unexpected value to perform `-` with".to_string())
    }

    fn mul_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_int) = other.as_any().downcast_ref::<FloatValue>() {
            let value = self.value * other_int.value;
            return Ok(Box::new(FloatValue { value }));
        }
        Err("Unexpected value to perform `*` with".to_string())
    }

    fn div_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_int) = other.as_any().downcast_ref::<FloatValue>() {
            let value = self.value / other_int.value;
            return Ok(Box::new(FloatValue { value }));
        }
        Err("Unexpected value to perform `/` with".to_string())
    }

    fn neg_op(&self) -> Result<Box<dyn Value>, String> {
        Ok(Box::new(FloatValue { value: -self.value }))
    }
}
