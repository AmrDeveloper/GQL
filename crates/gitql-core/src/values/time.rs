use std::any::Any;
use std::cmp::Ordering;

use gitql_ast::types::base::DataType;
use gitql_ast::types::time::TimeType;

use super::base::Value;

#[derive(Clone)]
pub struct TimeValue {
    pub value: String,
}

impl Value for TimeValue {
    fn literal(&self) -> String {
        self.value.to_string()
    }

    fn equals(&self, other: &Box<dyn Value>) -> bool {
        if let Some(other_time) = other.as_any().downcast_ref::<TimeValue>() {
            return self.value == other_time.value;
        }
        false
    }

    fn compare(&self, other: &Box<dyn Value>) -> Option<Ordering> {
        if let Some(other_time) = other.as_any().downcast_ref::<TimeValue>() {
            return self.value.partial_cmp(&other_time.value);
        }
        None
    }

    fn data_type(&self) -> Box<dyn DataType> {
        Box::new(TimeType)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
