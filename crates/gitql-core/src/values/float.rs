use std::any::Any;
use std::cmp::Ordering;

use gitql_ast::operator::GroupComparisonOperator;
use gitql_ast::types::float::FloatType;
use gitql_ast::types::DataType;

use super::base::Value;
use super::boolean::BoolValue;
use super::integer::IntValue;

#[derive(Clone)]
pub struct FloatValue {
    pub value: f64,
}

impl FloatValue {
    pub fn new(value: f64) -> Self {
        FloatValue { value }
    }
}

impl Value for FloatValue {
    fn literal(&self) -> String {
        self.value.to_string()
    }

    fn equals(&self, other: &Box<dyn Value>) -> bool {
        if let Some(other_float) = other.as_any().downcast_ref::<FloatValue>() {
            return self.value == other_float.value;
        }
        false
    }

    fn compare(&self, other: &Box<dyn Value>) -> Option<Ordering> {
        if let Some(other_float) = other.as_any().downcast_ref::<FloatValue>() {
            return self.value.partial_cmp(&other_float.value);
        }
        None
    }

    fn data_type(&self) -> Box<dyn DataType> {
        Box::new(FloatType)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn add_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_int) = other.as_any().downcast_ref::<FloatValue>() {
            let value = self.value + other_int.value;
            return Ok(Box::new(FloatValue { value }));
        }
        Err("Unexpected value to perform `+` with".to_string())
    }

    fn sub_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_int) = other.as_any().downcast_ref::<FloatValue>() {
            let value = self.value - other_int.value;
            return Ok(Box::new(FloatValue { value }));
        }
        Err("Unexpected value to perform `-` with".to_string())
    }

    fn mul_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_int) = other.as_any().downcast_ref::<FloatValue>() {
            let value = self.value * other_int.value;
            return Ok(Box::new(FloatValue { value }));
        }
        Err("Unexpected value to perform `*` with".to_string())
    }

    fn div_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_int) = other.as_any().downcast_ref::<FloatValue>() {
            let value = self.value / other_int.value;
            return Ok(Box::new(FloatValue { value }));
        }
        Err("Unexpected value to perform `/` with".to_string())
    }

    fn neg_op(&self) -> Result<Box<dyn Value>, String> {
        Ok(Box::new(FloatValue { value: -self.value }))
    }

    fn eq_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_bool) = other.as_any().downcast_ref::<FloatValue>() {
            let value = self.value == other_bool.value;
            return Ok(Box::new(BoolValue::new(value)));
        }
        Err("Unexpected type to perform `=` with".to_string())
    }

    fn group_eq_op(
        &self,
        other: &Box<dyn Value>,
        group_op: &GroupComparisonOperator,
    ) -> Result<Box<dyn Value>, String> {
        if other.is_array_of(|element_type| element_type.is_float()) {
            let elements = &other.as_array().unwrap();
            let mut matches_count = 0;
            for element in elements.iter() {
                if self.value == element.as_float().unwrap() {
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
        if let Some(other_bool) = other.as_any().downcast_ref::<FloatValue>() {
            let value = self.value != other_bool.value;
            return Ok(Box::new(BoolValue::new(value)));
        }
        Err("Unexpected type to perform `!=` with".to_string())
    }

    fn group_bang_eq_op(
        &self,
        other: &Box<dyn Value>,
        group_op: &GroupComparisonOperator,
    ) -> Result<Box<dyn Value>, String> {
        if other.is_array_of(|element_type| element_type.is_float()) {
            let elements = &other.as_array().unwrap();
            let mut matches_count = 0;
            for element in elements.iter() {
                if self.value != element.as_float().unwrap() {
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
        if let Some(other_bool) = other.as_any().downcast_ref::<FloatValue>() {
            let value = self.value > other_bool.value;
            return Ok(Box::new(BoolValue::new(value)));
        }
        Err("Unexpected type to perform `>` with".to_string())
    }

    fn group_gt_op(
        &self,
        other: &Box<dyn Value>,
        group_op: &GroupComparisonOperator,
    ) -> Result<Box<dyn Value>, String> {
        if other.is_array_of(|element_type| element_type.is_float()) {
            let elements = &other.as_array().unwrap();
            let mut matches_count = 0;
            for element in elements.iter() {
                if self.value > element.as_float().unwrap() {
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
        if let Some(other_bool) = other.as_any().downcast_ref::<FloatValue>() {
            let value = self.value >= other_bool.value;
            return Ok(Box::new(BoolValue::new(value)));
        }
        Err("Unexpected type to perform `>` with".to_string())
    }

    fn group_gte_op(
        &self,
        other: &Box<dyn Value>,
        group_op: &GroupComparisonOperator,
    ) -> Result<Box<dyn Value>, String> {
        if other.is_array_of(|element_type| element_type.is_float()) {
            let elements = &other.as_array().unwrap();
            let mut matches_count = 0;
            for element in elements.iter() {
                if self.value >= element.as_float().unwrap() {
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
        if let Some(other_bool) = other.as_any().downcast_ref::<FloatValue>() {
            let value = self.value < other_bool.value;
            return Ok(Box::new(BoolValue::new(value)));
        }
        Err("Unexpected type to perform `<` with".to_string())
    }

    fn group_lt_op(
        &self,
        other: &Box<dyn Value>,
        group_op: &GroupComparisonOperator,
    ) -> Result<Box<dyn Value>, String> {
        if other.is_array_of(|element_type| element_type.is_float()) {
            let elements = &other.as_array().unwrap();
            let mut matches_count = 0;
            for element in elements.iter() {
                if self.value < element.as_float().unwrap() {
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
        if let Some(other_bool) = other.as_any().downcast_ref::<FloatValue>() {
            let value = self.value <= other_bool.value;
            return Ok(Box::new(BoolValue::new(value)));
        }
        Err("Unexpected type to perform `<=` with".to_string())
    }

    fn group_lte_op(
        &self,
        other: &Box<dyn Value>,
        group_op: &GroupComparisonOperator,
    ) -> Result<Box<dyn Value>, String> {
        if other.is_array_of(|element_type| element_type.is_float()) {
            let elements = &other.as_array().unwrap();
            let mut matches_count = 0;
            for element in elements.iter() {
                if self.value <= element.as_float().unwrap() {
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
        // Cast Integer
        if target_type.is_int() {
            let value = self.value as i64;
            return Ok(Box::new(IntValue { value }));
        }

        Err("Unexpected value to perform `CAST` with".to_string())
    }
}
