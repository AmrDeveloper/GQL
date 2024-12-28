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
    pub value: i64,
}

impl DateValue {
    pub fn new(timestamp: i64) -> Self {
        DateValue { value: timestamp }
    }
}

impl Value for DateValue {
    fn literal(&self) -> String {
        let datetime = DateTime::from_timestamp(self.value, 0).unwrap();
        format!("{}", datetime.format(VALUE_DATE_FORMAT))
    }

    fn equals(&self, other: &Box<dyn Value>) -> bool {
        if let Some(other_date) = other.as_any().downcast_ref::<DateValue>() {
            return self.value == other_date.value;
        }
        false
    }

    fn compare(&self, other: &Box<dyn Value>) -> Option<Ordering> {
        if let Some(other_date) = other.as_any().downcast_ref::<DateValue>() {
            return self.value.partial_cmp(&other_date.value);
        }
        None
    }

    fn data_type(&self) -> Box<dyn DataType> {
        Box::new(DateType)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn eq_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_text) = other.as_any().downcast_ref::<DateValue>() {
            let are_equals = self.value == other_text.value;
            return Ok(Box::new(BoolValue { value: are_equals }));
        }
        Err("Unexpected type to perform `=` with".to_string())
    }

    fn bang_eq_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_text) = other.as_any().downcast_ref::<DateValue>() {
            let are_equals = self.value != other_text.value;
            return Ok(Box::new(BoolValue { value: are_equals }));
        }
        Err("Unexpected type to perform `!=` with".to_string())
    }

    fn gt_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_text) = other.as_any().downcast_ref::<DateValue>() {
            let are_equals = self.value > other_text.value;
            return Ok(Box::new(BoolValue { value: are_equals }));
        }
        Err("Unexpected type to perform `>` with".to_string())
    }

    fn gte_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_text) = other.as_any().downcast_ref::<DateValue>() {
            let are_equals = self.value >= other_text.value;
            return Ok(Box::new(BoolValue { value: are_equals }));
        }
        Err("Unexpected type to perform `>=` with".to_string())
    }

    fn lt_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_text) = other.as_any().downcast_ref::<DateValue>() {
            let are_equals = self.value < other_text.value;
            return Ok(Box::new(BoolValue { value: are_equals }));
        }
        Err("Unexpected type to perform `<` with".to_string())
    }

    fn lte_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_text) = other.as_any().downcast_ref::<DateValue>() {
            let are_equals = self.value <= other_text.value;
            return Ok(Box::new(BoolValue { value: are_equals }));
        }
        Err("Unexpected type to perform `<=` with".to_string())
    }
}
