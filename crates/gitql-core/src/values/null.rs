use std::any::Any;
use std::cmp::Ordering;

use gitql_ast::types::null::NullType;
use gitql_ast::types::DataType;

use super::base::Value;

#[derive(Clone)]
pub struct NullValue;

impl Value for NullValue {
    fn literal(&self) -> String {
        "Null".to_string()
    }

    fn equals(&self, other: &Box<dyn Value>) -> bool {
        other.as_any().downcast_ref::<NullValue>().is_some()
    }

    fn compare(&self, _other: &Box<dyn Value>) -> Option<Ordering> {
        None
    }

    fn data_type(&self) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
