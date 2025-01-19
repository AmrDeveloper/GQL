use std::any::Any;
use std::cmp::Ordering;

use super::base::Value;
use super::boolean::BoolValue;
use super::date::DateValue;

use chrono::DateTime;
use gitql_ast::types::base::DataType;
use gitql_ast::types::datetime::DateTimeType;

const VALUE_DATE_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S%.3f";

#[derive(Clone)]
pub struct DateTimeValue {
    pub value: i64,
}

impl DateTimeValue {
    pub fn new(timestamp: i64) -> Self {
        DateTimeValue { value: timestamp }
    }
}

impl Value for DateTimeValue {
    fn literal(&self) -> String {
        let datetime = DateTime::from_timestamp(self.value, 0).unwrap();
        format!("{}", datetime.format(VALUE_DATE_TIME_FORMAT))
    }

    fn equals(&self, other: &Box<dyn Value>) -> bool {
        if let Some(other_datetime) = other.as_any().downcast_ref::<DateTimeValue>() {
            return self.value == other_datetime.value;
        }
        false
    }

    fn compare(&self, other: &Box<dyn Value>) -> Option<Ordering> {
        if let Some(other_datetime) = other.as_any().downcast_ref::<DateTimeValue>() {
            return self.value.partial_cmp(&other_datetime.value);
        }
        None
    }

    fn data_type(&self) -> Box<dyn DataType> {
        Box::new(DateTimeType)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn eq_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_text) = other.as_any().downcast_ref::<DateTimeValue>() {
            let are_equals = self.value == other_text.value;
            return Ok(Box::new(BoolValue { value: are_equals }));
        }
        Err("Unexpected type to perform `=` with".to_string())
    }

    fn bang_eq_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_text) = other.as_any().downcast_ref::<DateTimeValue>() {
            let are_equals = self.value != other_text.value;
            return Ok(Box::new(BoolValue { value: are_equals }));
        }
        Err("Unexpected type to perform `!=` with".to_string())
    }

    fn gt_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_text) = other.as_any().downcast_ref::<DateTimeValue>() {
            let are_equals = self.value > other_text.value;
            return Ok(Box::new(BoolValue { value: are_equals }));
        }
        Err("Unexpected type to perform `>` with".to_string())
    }

    fn gte_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_text) = other.as_any().downcast_ref::<DateTimeValue>() {
            let are_equals = self.value >= other_text.value;
            return Ok(Box::new(BoolValue { value: are_equals }));
        }
        Err("Unexpected type to perform `>=` with".to_string())
    }

    fn lt_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_text) = other.as_any().downcast_ref::<DateTimeValue>() {
            let are_equals = self.value < other_text.value;
            return Ok(Box::new(BoolValue { value: are_equals }));
        }
        Err("Unexpected type to perform `<` with".to_string())
    }

    fn lte_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_text) = other.as_any().downcast_ref::<DateTimeValue>() {
            let are_equals = self.value <= other_text.value;
            return Ok(Box::new(BoolValue { value: are_equals }));
        }
        Err("Unexpected type to perform `<=` with".to_string())
    }

    fn cast_op(&self, target_type: &Box<dyn DataType>) -> Result<Box<dyn Value>, String> {
        if target_type.is_date() {
            return Ok(Box::new(DateValue {
                timestamp: self.value,
            }));
        }
        Err("Unexpected type to perform `Cast` with".to_string())
    }
}
