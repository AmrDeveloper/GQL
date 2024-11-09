use std::any::Any;
use std::cmp::Ordering;

use gitql_ast::types::base::DataType;
use gitql_ast::types::integer::IntType;

use super::base::Value;
use super::boolean::BoolValue;
use super::float::FloatValue;

#[derive(Clone)]
pub struct IntValue {
    pub value: i64,
}

impl Value for IntValue {
    fn literal(&self) -> String {
        self.value.to_string()
    }

    fn equals(&self, other: &Box<dyn Value>) -> bool {
        if let Some(other_int) = other.as_any().downcast_ref::<IntValue>() {
            return self.value == other_int.value;
        }
        false
    }

    fn compare(&self, other: &Box<dyn Value>) -> Option<Ordering> {
        if let Some(other_int) = other.as_any().downcast_ref::<IntValue>() {
            return self.value.partial_cmp(&other_int.value);
        }
        None
    }

    fn data_type(&self) -> Box<dyn DataType> {
        Box::new(IntType)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn add_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_int) = other.as_any().downcast_ref::<IntValue>() {
            let value = self.value + other_int.value;
            return Ok(Box::new(IntValue { value }));
        }
        Err("Unexpected type to perform `+` with".to_string())
    }

    fn sub_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_int) = other.as_any().downcast_ref::<IntValue>() {
            let value = self.value - other_int.value;
            return Ok(Box::new(IntValue { value }));
        }
        Err("Unexpected type to perform `-` with".to_string())
    }

    fn mul_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_int) = other.as_any().downcast_ref::<IntValue>() {
            let value = self.value * other_int.value;
            return Ok(Box::new(IntValue { value }));
        }
        Err("Unexpected type to perform `*` with".to_string())
    }

    fn div_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_int) = other.as_any().downcast_ref::<IntValue>() {
            if other_int.value == 0 {
                return Err("Can't perform `/` operator with 0 value".to_string());
            }
            let value = self.value / other_int.value;
            return Ok(Box::new(IntValue { value }));
        }
        Err("Unexpected type to perform `/` with".to_string())
    }

    fn rem_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_int) = other.as_any().downcast_ref::<IntValue>() {
            let value = self.value % other_int.value;
            return Ok(Box::new(IntValue { value }));
        }
        Err("Unexpected type to perform `%` with".to_string())
    }

    fn caret_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_int) = other.as_any().downcast_ref::<IntValue>() {
            if other_int.value < 0 {
                return Err("Caret right side hand can't be negative value".to_string());
            }
            let value = self.value.pow(other_int.value as u32);
            return Ok(Box::new(IntValue { value }));
        }
        Err("Unexpected type to perform `^` with".to_string())
    }

    fn or_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_int) = other.as_any().downcast_ref::<IntValue>() {
            let value = self.value | other_int.value;
            return Ok(Box::new(IntValue { value }));
        }
        Err("Unexpected type to perform `|` with".to_string())
    }

    fn and_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_int) = other.as_any().downcast_ref::<IntValue>() {
            let value = self.value & other_int.value;
            return Ok(Box::new(IntValue { value }));
        }
        Err("Unexpected type to perform `&` with".to_string())
    }

    fn xor_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_int) = other.as_any().downcast_ref::<IntValue>() {
            let value = self.value ^ other_int.value;
            return Ok(Box::new(IntValue { value }));
        }
        Err("Unexpected type to perform `^` with".to_string())
    }

    fn shl_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_int) = other.as_any().downcast_ref::<IntValue>() {
            let value = self.value << other_int.value;
            return Ok(Box::new(IntValue { value }));
        }
        Err("Unexpected type to perform `<<` with".to_string())
    }

    fn shr_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_int) = other.as_any().downcast_ref::<IntValue>() {
            let value = self.value >> other_int.value;
            return Ok(Box::new(IntValue { value }));
        }
        Err("Unexpected type to perform `>>` with".to_string())
    }

    fn eq_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_bool) = other.as_any().downcast_ref::<IntValue>() {
            let value = self.value == other_bool.value;
            return Ok(Box::new(BoolValue { value }));
        }
        Err("Unexpected type to perform `^` with".to_string())
    }

    fn bang_eq_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_bool) = other.as_any().downcast_ref::<IntValue>() {
            let value = self.value != other_bool.value;
            return Ok(Box::new(BoolValue { value }));
        }
        Err("Unexpected type to perform `!=` with".to_string())
    }

    fn gt_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_bool) = other.as_any().downcast_ref::<IntValue>() {
            let value = self.value > other_bool.value;
            return Ok(Box::new(BoolValue { value }));
        }
        Err("Unexpected type to perform `>` with".to_string())
    }

    fn gte_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_bool) = other.as_any().downcast_ref::<IntValue>() {
            let value = self.value >= other_bool.value;
            return Ok(Box::new(BoolValue { value }));
        }
        Err("Unexpected type to perform `>` with".to_string())
    }

    fn lt_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_bool) = other.as_any().downcast_ref::<IntValue>() {
            let value = self.value < other_bool.value;
            return Ok(Box::new(BoolValue { value }));
        }
        Err("Unexpected type to perform `<` with".to_string())
    }

    fn lte_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_bool) = other.as_any().downcast_ref::<IntValue>() {
            let value = self.value <= other_bool.value;
            return Ok(Box::new(BoolValue { value }));
        }
        Err("Unexpected type to perform `<=` with".to_string())
    }

    fn neg_op(&self) -> Result<Box<dyn Value>, String> {
        Ok(Box::new(IntValue { value: -self.value }))
    }

    fn cast_op(&self, target_type: &Box<dyn DataType>) -> Result<Box<dyn Value>, String> {
        // Cast to Boolean
        if target_type.is_bool() {
            let value = self.value != 0;
            return Ok(Box::new(BoolValue { value }));
        }

        // Cast to Float
        if target_type.is_float() {
            let value = self.value as f64;
            return Ok(Box::new(FloatValue { value }));
        }

        Err("Unexpected value to perform `CAST` with".to_string())
    }
}
