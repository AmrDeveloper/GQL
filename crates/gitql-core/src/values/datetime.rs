use std::any::Any;
use std::cmp::Ordering;

use super::base::Value;
use super::boolean::BoolValue;
use super::date::DateValue;

use chrono::DateTime;
use gitql_ast::operator::GroupComparisonOperator;
use gitql_ast::types::datetime::DateTimeType;
use gitql_ast::types::DataType;

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

    fn group_eq_op(
        &self,
        other: &Box<dyn Value>,
        group_op: &GroupComparisonOperator,
    ) -> Result<Box<dyn Value>, String> {
        if other.is_array_of(|element_type| element_type.is_date_time()) {
            let elements = &other.as_array().unwrap();
            let mut matches_count = 0;
            for element in elements.iter() {
                if self.value == element.as_date_time().unwrap() {
                    matches_count += 1;
                    if GroupComparisonOperator::Any.eq(group_op) {
                        break;
                    }
                }
            }

            let result = match group_op {
                GroupComparisonOperator::All => matches_count == elements.len(),
                GroupComparisonOperator::Any => matches_count > 0,
            };

            return Ok(Box::new(BoolValue::new(result)));
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

    fn group_bang_eq_op(
        &self,
        other: &Box<dyn Value>,
        group_op: &GroupComparisonOperator,
    ) -> Result<Box<dyn Value>, String> {
        if other.is_array_of(|element_type| element_type.is_date_time()) {
            let elements = &other.as_array().unwrap();
            let mut matches_count = 0;
            for element in elements.iter() {
                if self.value != element.as_date_time().unwrap() {
                    matches_count += 1;
                    if GroupComparisonOperator::Any.eq(group_op) {
                        break;
                    }
                }
            }

            let result = match group_op {
                GroupComparisonOperator::All => matches_count == elements.len(),
                GroupComparisonOperator::Any => matches_count > 0,
            };

            return Ok(Box::new(BoolValue::new(result)));
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

    fn group_gt_op(
        &self,
        other: &Box<dyn Value>,
        group_op: &GroupComparisonOperator,
    ) -> Result<Box<dyn Value>, String> {
        if other.is_array_of(|element_type| element_type.is_date_time()) {
            let elements = &other.as_array().unwrap();
            let mut matches_count = 0;
            for element in elements.iter() {
                if self.value > element.as_date_time().unwrap() {
                    matches_count += 1;
                    if GroupComparisonOperator::Any.eq(group_op) {
                        break;
                    }
                }
            }

            let result = match group_op {
                GroupComparisonOperator::All => matches_count == elements.len(),
                GroupComparisonOperator::Any => matches_count > 0,
            };

            return Ok(Box::new(BoolValue::new(result)));
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

    fn group_gte_op(
        &self,
        other: &Box<dyn Value>,
        group_op: &GroupComparisonOperator,
    ) -> Result<Box<dyn Value>, String> {
        if other.is_array_of(|element_type| element_type.is_date_time()) {
            let elements = &other.as_array().unwrap();
            let mut matches_count = 0;
            for element in elements.iter() {
                if self.value >= element.as_date_time().unwrap() {
                    matches_count += 1;
                    if GroupComparisonOperator::Any.eq(group_op) {
                        break;
                    }
                }
            }

            let result = match group_op {
                GroupComparisonOperator::All => matches_count == elements.len(),
                GroupComparisonOperator::Any => matches_count > 0,
            };

            return Ok(Box::new(BoolValue::new(result)));
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

    fn group_lt_op(
        &self,
        other: &Box<dyn Value>,
        group_op: &GroupComparisonOperator,
    ) -> Result<Box<dyn Value>, String> {
        if other.is_array_of(|element_type| element_type.is_date_time()) {
            let elements = &other.as_array().unwrap();
            let mut matches_count = 0;
            for element in elements.iter() {
                if self.value < element.as_date_time().unwrap() {
                    matches_count += 1;
                    if GroupComparisonOperator::Any.eq(group_op) {
                        break;
                    }
                }
            }

            let result = match group_op {
                GroupComparisonOperator::All => matches_count == elements.len(),
                GroupComparisonOperator::Any => matches_count > 0,
            };

            return Ok(Box::new(BoolValue::new(result)));
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

    fn group_lte_op(
        &self,
        other: &Box<dyn Value>,
        group_op: &GroupComparisonOperator,
    ) -> Result<Box<dyn Value>, String> {
        if other.is_array_of(|element_type| element_type.is_date_time()) {
            let elements = &other.as_array().unwrap();
            let mut matches_count = 0;
            for element in elements.iter() {
                if self.value <= element.as_date_time().unwrap() {
                    matches_count += 1;
                    if GroupComparisonOperator::Any.eq(group_op) {
                        break;
                    }
                }
            }

            let result = match group_op {
                GroupComparisonOperator::All => matches_count == elements.len(),
                GroupComparisonOperator::Any => matches_count > 0,
            };

            return Ok(Box::new(BoolValue::new(result)));
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
