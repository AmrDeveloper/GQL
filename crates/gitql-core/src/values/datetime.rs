use std::any::Any;
use std::cmp::Ordering;

use super::base::Value;

use chrono::DateTime;
use gitql_ast::types::base::DataType;
use gitql_ast::types::datetime::DateTimeType;

const VALUE_DATE_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S%.3f";

#[derive(Clone)]
pub struct DateTimeValue {
    pub value: i64,
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
}
