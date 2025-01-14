use std::any::Any;
use std::cmp::Ordering;

use gitql_ast::types::base::DataType;
use gitql_ast::types::interval::IntervalType;
use gitql_ast::Interval;

use super::base::Value;
use super::boolean::BoolValue;

#[derive(Clone)]
pub struct IntervalValue {
    pub interval: Interval,
}

impl IntervalValue {
    pub fn new(interval: Interval) -> Self {
        IntervalValue { interval }
    }
}

impl Value for IntervalValue {
    fn literal(&self) -> String {
        self.interval.to_string()
    }

    fn equals(&self, other: &Box<dyn Value>) -> bool {
        if let Some(other_date) = other.as_any().downcast_ref::<IntervalValue>() {
            return self.interval == other_date.interval;
        }
        false
    }

    fn compare(&self, _other: &Box<dyn Value>) -> Option<Ordering> {
        None
    }

    fn data_type(&self) -> Box<dyn DataType> {
        Box::new(IntervalType)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn eq_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_interval) = other.as_any().downcast_ref::<IntervalValue>() {
            let is_equals = self.interval == other_interval.interval;
            return Ok(Box::new(BoolValue::new(is_equals)));
        }
        Err("Unexpected type to perform `=` with".to_string())
    }

    fn bang_eq_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_interval) = other.as_any().downcast_ref::<IntervalValue>() {
            let is_not_equals = self.interval != other_interval.interval;
            return Ok(Box::new(BoolValue::new(is_not_equals)));
        }
        Err("Unexpected type to perform `!=` with".to_string())
    }

    fn add_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_interval) = other.as_any().downcast_ref::<IntervalValue>() {
            let result = self.interval.add(&other_interval.interval);
            return Ok(Box::new(IntervalValue::new(result)));
        }
        Err("Unexpected type to perform `+` with".to_string())
    }

    fn sub_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_interval) = other.as_any().downcast_ref::<IntervalValue>() {
            let result = self.interval.sub(&other_interval.interval);
            return Ok(Box::new(IntervalValue::new(result)));
        }
        Err("Unexpected type to perform `-` with".to_string())
    }
}
