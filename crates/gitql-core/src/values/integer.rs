use std::any::Any;
use std::cmp::Ordering;

use gitql_ast::types::base::DataType;
use gitql_ast::types::integer::IntType;

use super::base::Value;
use super::boolean::BoolValue;

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

    fn perform_add_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_int) = other.as_any().downcast_ref::<IntValue>() {
            let value = self.value + other_int.value;
            return Ok(Box::new(IntValue { value }));
        }
        Err("Unexpected type to perform `+` with".to_string())
    }

    fn perform_sub_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_int) = other.as_any().downcast_ref::<IntValue>() {
            let value = self.value - other_int.value;
            return Ok(Box::new(IntValue { value }));
        }
        Err("Unexpected type to perform `-` with".to_string())
    }

    fn perform_mul_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_int) = other.as_any().downcast_ref::<IntValue>() {
            let value = self.value * other_int.value;
            return Ok(Box::new(IntValue { value }));
        }
        Err("Unexpected type to perform `*` with".to_string())
    }

    fn perform_div_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_int) = other.as_any().downcast_ref::<IntValue>() {
            if other_int.value == 0 {
                return Err("Can't perform `/` operator with 0 value".to_string());
            }
            let value = self.value / other_int.value;
            return Ok(Box::new(IntValue { value }));
        }
        Err("Unexpected type to perform `/` with".to_string())
    }

    fn perform_rem_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_int) = other.as_any().downcast_ref::<IntValue>() {
            let value = self.value % other_int.value;
            return Ok(Box::new(IntValue { value }));
        }
        Err("Unexpected type to perform `%` with".to_string())
    }

    fn perform_caret_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_int) = other.as_any().downcast_ref::<IntValue>() {
            if other_int.value < 0 {
                return Err("Caret right side hand can't be negative value".to_string());
            }
            let value = self.value.pow(other_int.value as u32);
            return Ok(Box::new(IntValue { value }));
        }
        Err("Unexpected type to perform `^` with".to_string())
    }

    fn perform_or_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_int) = other.as_any().downcast_ref::<IntValue>() {
            let value = self.value | other_int.value;
            return Ok(Box::new(IntValue { value }));
        }
        Err("Unexpected type to perform `|` with".to_string())
    }

    fn perform_and_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_int) = other.as_any().downcast_ref::<IntValue>() {
            let value = self.value & other_int.value;
            return Ok(Box::new(IntValue { value }));
        }
        Err("Unexpected type to perform `&` with".to_string())
    }

    fn perform_xor_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_int) = other.as_any().downcast_ref::<IntValue>() {
            let value = self.value ^ other_int.value;
            return Ok(Box::new(IntValue { value }));
        }
        Err("Unexpected type to perform `^` with".to_string())
    }

    fn perform_shl_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_int) = other.as_any().downcast_ref::<IntValue>() {
            let value = self.value << other_int.value;
            return Ok(Box::new(IntValue { value }));
        }
        Err("Unexpected type to perform `<<` with".to_string())
    }

    fn perform_shr_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_int) = other.as_any().downcast_ref::<IntValue>() {
            let value = self.value >> other_int.value;
            return Ok(Box::new(IntValue { value }));
        }
        Err("Unexpected type to perform `>>` with".to_string())
    }

    fn perform_eq_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_bool) = other.as_any().downcast_ref::<IntValue>() {
            let value = self.value == other_bool.value;
            return Ok(Box::new(BoolValue { value }));
        }
        Err("Unexpected type to perform `^` with".to_string())
    }

    fn perform_bang_eq_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_bool) = other.as_any().downcast_ref::<IntValue>() {
            let value = self.value != other_bool.value;
            return Ok(Box::new(BoolValue { value }));
        }
        Err("Unexpected type to perform `!=` with".to_string())
    }

    fn perform_gt_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_bool) = other.as_any().downcast_ref::<IntValue>() {
            let value = self.value > other_bool.value;
            return Ok(Box::new(BoolValue { value }));
        }
        Err("Unexpected type to perform `>` with".to_string())
    }

    fn perform_gte_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_bool) = other.as_any().downcast_ref::<IntValue>() {
            let value = self.value >= other_bool.value;
            return Ok(Box::new(BoolValue { value }));
        }
        Err("Unexpected type to perform `>` with".to_string())
    }

    fn perform_lt_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_bool) = other.as_any().downcast_ref::<IntValue>() {
            let value = self.value < other_bool.value;
            return Ok(Box::new(BoolValue { value }));
        }
        Err("Unexpected type to perform `<` with".to_string())
    }

    fn perform_lte_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_bool) = other.as_any().downcast_ref::<IntValue>() {
            let value = self.value <= other_bool.value;
            return Ok(Box::new(BoolValue { value }));
        }
        Err("Unexpected type to perform `<=` with".to_string())
    }

    fn perform_neg_op(&self) -> Result<Box<dyn Value>, String> {
        Ok(Box::new(IntValue { value: -self.value }))
    }
}
