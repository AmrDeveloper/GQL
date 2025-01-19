use std::any::Any;
use std::cmp::Ordering;

use super::base::Value;
use super::boolean::BoolValue;

use chrono::DateTime;
use gitql_ast::types::base::DataType;
use gitql_ast::types::date::DateType;

const VALUE_DATE_FORMAT: &str = "%Y-%m-%d";

#[derive(Clone)]
pub struct DateValue {
    pub timestamp: i64,
}

impl DateValue {
    pub fn new(timestamp: i64) -> Self {
        DateValue { timestamp }
    }
}

impl Value for DateValue {
    fn literal(&self) -> String {
        let datetime = DateTime::from_timestamp(self.timestamp, 0).unwrap();
        format!("{}", datetime.format(VALUE_DATE_FORMAT))
    }

    fn equals(&self, other: &Box<dyn Value>) -> bool {
        if let Some(other_date) = other.as_any().downcast_ref::<DateValue>() {
            return self.timestamp == other_date.timestamp;
        }
        false
    }

    fn compare(&self, other: &Box<dyn Value>) -> Option<Ordering> {
        if let Some(other_date) = other.as_any().downcast_ref::<DateValue>() {
            return self.timestamp.partial_cmp(&other_date.timestamp);
        }
        None
    }

    fn data_type(&self) -> Box<dyn DataType> {
        Box::new(DateType)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn add_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(days) = other.as_int() {
            let days_to_timestamp = days * 24 * 60 * 60;
            let timestamp = self.timestamp + days_to_timestamp;
            return Ok(Box::new(DateValue::new(timestamp)));
        }
        Err("Unexpected type to perform `+` with".to_string())
    }

    fn sub_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(days) = other.as_int() {
            let days_to_timestamp = days * 24 * 60 * 60;
            let timestamp = self.timestamp - days_to_timestamp;
            return Ok(Box::new(DateValue::new(timestamp)));
        }
        Err("Unexpected type to perform `-` with".to_string())
    }

    fn eq_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_text) = other.as_any().downcast_ref::<DateValue>() {
            let are_equals = self.timestamp == other_text.timestamp;
            return Ok(Box::new(BoolValue { value: are_equals }));
        }
        Err("Unexpected type to perform `=` with".to_string())
    }

    fn bang_eq_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_text) = other.as_any().downcast_ref::<DateValue>() {
            let are_equals = self.timestamp != other_text.timestamp;
            return Ok(Box::new(BoolValue { value: are_equals }));
        }
        Err("Unexpected type to perform `!=` with".to_string())
    }

    fn gt_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_text) = other.as_any().downcast_ref::<DateValue>() {
            let are_equals = self.timestamp > other_text.timestamp;
            return Ok(Box::new(BoolValue { value: are_equals }));
        }
        Err("Unexpected type to perform `>` with".to_string())
    }

    fn gte_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_text) = other.as_any().downcast_ref::<DateValue>() {
            let are_equals = self.timestamp >= other_text.timestamp;
            return Ok(Box::new(BoolValue { value: are_equals }));
        }
        Err("Unexpected type to perform `>=` with".to_string())
    }

    fn lt_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_text) = other.as_any().downcast_ref::<DateValue>() {
            let are_equals = self.timestamp < other_text.timestamp;
            return Ok(Box::new(BoolValue { value: are_equals }));
        }
        Err("Unexpected type to perform `<` with".to_string())
    }

    fn lte_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_text) = other.as_any().downcast_ref::<DateValue>() {
            let are_equals = self.timestamp <= other_text.timestamp;
            return Ok(Box::new(BoolValue { value: are_equals }));
        }
        Err("Unexpected type to perform `<=` with".to_string())
    }
}
