use std::any::Any;
use std::cmp::Ordering;

use super::base::Value;

use chrono::DateTime;
use gitql_ast::types::base::DataType;
use gitql_ast::types::date::DateType;

const VALUE_DATE_FORMAT: &str = "%Y-%m-%d";

#[derive(Clone)]
pub struct DateValue {
    pub value: i64,
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
}
