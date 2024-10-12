use std::any::Any;
use std::cmp::Ordering;

use gitql_ast::types::base::DataType;
use gitql_ast::types::range::RangeType;

use super::base::Value;
use super::boolean::BoolValue;

#[derive(Clone)]
pub struct RangeValue {
    pub start: Box<dyn Value>,
    pub end: Box<dyn Value>,
    pub base_type: Box<dyn DataType>,
}

impl Value for RangeValue {
    fn literal(&self) -> String {
        format!("{}..{}", self.start.literal(), self.end.literal())
    }

    fn equals(&self, other: &Box<dyn Value>) -> bool {
        if let Some(other_range) = other.as_any().downcast_ref::<RangeValue>() {
            return self.base_type.equals(&other_range.base_type)
                && self.start.equals(&other_range.start)
                && self.end.equals(&other_range.end);
        }
        false
    }

    fn compare(&self, _other: &Box<dyn Value>) -> Option<Ordering> {
        None
    }

    fn data_type(&self) -> Box<dyn DataType> {
        Box::new(RangeType {
            base: self.base_type.clone(),
        })
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn perform_logical_or_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_range) = other.as_any().downcast_ref::<RangeValue>() {
            if !self.equals(other) {
                return Err("Overlap operator expect both Ranges to have same type".to_string());
            }

            let max_start = if self.start.compare(&other_range.start).unwrap().is_le() {
                &self.start
            } else {
                &other_range.start
            };

            let max_end = if self.end.compare(&other_range.end).unwrap().is_gt() {
                &self.end
            } else {
                &other_range.end
            };

            let is_overlap = max_end.compare(max_start).unwrap().is_le();
            return Ok(Box::new(BoolValue { value: is_overlap }));
        }
        Err("Unexpected type to perform `Range Overlap &&` with".to_string())
    }

    fn perform_contains_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_range) = other.as_any().downcast_ref::<RangeValue>() {
            if !self.equals(other) {
                return Err("Contains operator expect both Ranges to have same type".to_string());
            }

            let is_in_range = other_range.start.compare(&self.start).unwrap().is_ge()
                && other_range.end.compare(&self.end).unwrap().is_le();
            return Ok(Box::new(BoolValue { value: is_in_range }));
        }

        if self.base_type.equals(&other.data_type()) {
            let is_in_range = other.compare(&self.start).unwrap().is_ge()
                && other.compare(&self.start).unwrap().is_le();
            return Ok(Box::new(BoolValue { value: is_in_range }));
        }

        Err("Unexpected type to perform `Range contains @>` with".to_string())
    }
}
