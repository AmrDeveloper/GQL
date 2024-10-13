use std::any::Any;
use std::cmp::Ordering;
use std::fmt;

use dyn_clone::DynClone;
use gitql_ast::types::base::DataType;

use super::array::ArrayValue;
use super::boolean::BoolValue;
use super::date::DateValue;
use super::datetime::DateTimeValue;
use super::float::FloatValue;
use super::integer::IntValue;
use super::null::NullValue;
use super::range::RangeValue;
use super::text::TextValue;
use super::time::TimeValue;

dyn_clone::clone_trait_object!(Value);

pub trait Value: DynClone {
    fn literal(&self) -> String;

    fn equals(&self, other: &Box<dyn Value>) -> bool;

    fn compare(&self, other: &Box<dyn Value>) -> Option<Ordering>;

    fn data_type(&self) -> Box<dyn DataType>;

    fn as_any(&self) -> &dyn Any;

    fn perform_add_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_sub_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_mul_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_div_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_rem_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_caret_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_or_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_and_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_xor_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_logical_or_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_logical_and_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_logical_xor_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_shl_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_shr_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_index_op(&self, _index: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_slice_op(
        &self,
        _start: &Option<Box<dyn Value>>,
        _end: &Option<Box<dyn Value>>,
    ) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_eq_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_bang_eq_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_null_safe_eq_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_gt_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_gte_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_lt_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_lte_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_not_op(&self) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_neg_op(&self) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_bang_op(&self) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_contains_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_cast_op(&self, _target_type: &Box<dyn DataType>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }
}

impl dyn Value {
    pub fn is_text(&self) -> bool {
        self.as_any().downcast_ref::<TextValue>().is_some()
    }

    pub fn as_text(&self) -> Option<String> {
        if let Some(text_value) = self.as_any().downcast_ref::<TextValue>() {
            return Some(text_value.value.to_string());
        }
        None
    }

    pub fn is_int(&self) -> bool {
        self.as_any().downcast_ref::<IntValue>().is_some()
    }

    pub fn as_int(&self) -> Option<i64> {
        if let Some(int_value) = self.as_any().downcast_ref::<IntValue>() {
            return Some(int_value.value);
        }
        None
    }

    pub fn is_float(&self) -> bool {
        self.as_any().downcast_ref::<FloatValue>().is_some()
    }

    pub fn as_float(&self) -> Option<f64> {
        if let Some(float_value) = self.as_any().downcast_ref::<FloatValue>() {
            return Some(float_value.value);
        }
        None
    }

    pub fn is_number(&self) -> bool {
        self.is_int() || self.is_float()
    }

    pub fn is_bool(&self) -> bool {
        self.as_any().downcast_ref::<BoolValue>().is_some()
    }

    pub fn as_bool(&self) -> Option<bool> {
        if let Some(bool_value) = self.as_any().downcast_ref::<BoolValue>() {
            return Some(bool_value.value);
        }
        None
    }

    pub fn is_date(&self) -> bool {
        self.as_any().downcast_ref::<DateValue>().is_some()
    }

    pub fn as_date(&self) -> Option<i64> {
        if let Some(date_value) = self.as_any().downcast_ref::<DateValue>() {
            return Some(date_value.value);
        }
        None
    }

    pub fn is_time(&self) -> bool {
        self.as_any().downcast_ref::<TimeValue>().is_some()
    }

    pub fn as_time(&self) -> Option<String> {
        if let Some(time_value) = self.as_any().downcast_ref::<DateValue>() {
            return Some(time_value.value.to_string());
        }
        None
    }

    pub fn is_datetime(&self) -> bool {
        self.as_any().downcast_ref::<DateTimeValue>().is_some()
    }

    pub fn as_datetime(&self) -> Option<i64> {
        if let Some(datetime_value) = self.as_any().downcast_ref::<DateTimeValue>() {
            return Some(datetime_value.value);
        }
        None
    }

    pub fn is_array(&self) -> bool {
        self.as_any().downcast_ref::<ArrayValue>().is_some()
    }

    pub fn as_array(&self) -> Option<Vec<Box<dyn Value>>> {
        if let Some(array_value) = self.as_any().downcast_ref::<ArrayValue>() {
            return Some(array_value.values.clone());
        }
        None
    }

    pub fn is_range(&self) -> bool {
        self.as_any().downcast_ref::<RangeValue>().is_some()
    }

    pub fn as_range(&self) -> Option<(Box<dyn Value>, Box<dyn Value>)> {
        if let Some(range_value) = self.as_any().downcast_ref::<RangeValue>() {
            return Some((range_value.start.clone(), range_value.end.clone()));
        }
        None
    }

    pub fn is_null(&self) -> bool {
        self.as_any().downcast_ref::<NullValue>().is_some()
    }
}

impl fmt::Display for Box<dyn Value> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.literal())
    }
}

impl PartialEq for Box<dyn Value> {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

impl PartialOrd for Box<dyn Value> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.compare(other)
    }
}
