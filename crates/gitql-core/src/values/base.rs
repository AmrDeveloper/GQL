use std::any::Any;
use std::cmp::Ordering;
use std::fmt;

use dyn_clone::DynClone;
use gitql_ast::types::base::DataType;

use super::array::ArrayValue;
use super::boolean::BoolValue;
use super::composite::CompositeValue;
use super::date::DateValue;
use super::datetime::DateTimeValue;
use super::float::FloatValue;
use super::integer::IntValue;
use super::null::NullValue;
use super::range::RangeValue;
use super::text::TextValue;
use super::time::TimeValue;

dyn_clone::clone_trait_object!(Value);

/// The in memory representation of the Values in the GitQL query engine
pub trait Value: DynClone {
    /// Return the literal representation for this [`Value`]
    fn literal(&self) -> String;

    /// Return if other [`Value`] is equal or not to current value
    #[allow(clippy::borrowed_box)]
    fn equals(&self, other: &Box<dyn Value>) -> bool;

    /// Return the order between [`Value`] and the current value,
    /// or None if they can't be ordered
    #[allow(clippy::borrowed_box)]
    fn compare(&self, other: &Box<dyn Value>) -> Option<Ordering>;

    /// Return the [`DataType`] for the current [`Value`]
    fn data_type(&self) -> Box<dyn DataType>;

    /// Return the current value as dynamic [`Any`]
    fn as_any(&self) -> &dyn Any;

    /// Perform unary `=` operator and return new [`Value`] represent the old value after casted
    /// or Exception message as [`String`]
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn add_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    /// Perform unary `-` operator and return new [`Value`] represent the old value after casted
    /// or Exception message as [`String`]
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn sub_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    /// Perform unary `*` operator and return new [`Value`] represent the old value after casted
    /// or Exception message as [`String`]
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn mul_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    /// Perform unary `/` operator and return new [`Value`] represent the old value after casted
    /// or Exception message as [`String`]
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn div_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    /// Perform unary `%` operator and return new [`Value`] represent the old value after casted
    /// or Exception message as [`String`]
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn rem_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    /// Perform unary `^` operator and return new [`Value`] represent the old value after casted
    /// or Exception message as [`String`]
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn caret_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    /// Perform unary `|` operator and return new [`Value`] represent the old value after casted
    /// or Exception message as [`String`]
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn or_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    /// Perform unary `&` operator and return new [`Value`] represent the old value after casted
    /// or Exception message as [`String`]
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn and_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    /// Perform unary `#` operator and return new [`Value`] represent the old value after casted
    /// or Exception message as [`String`]
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn xor_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    /// Perform unary `||` or `OR` operator and return new [`Value`] represent the old value after casted
    /// or Exception message as [`String`]
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn logical_or_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    /// Perform unary `&&` or `AND` operator and return new [`Value`] represent the old value after casted
    /// or Exception message as [`String`]
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn logical_and_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    /// Perform unary `XOR` operator and return new [`Value`] represent the old value after casted
    /// or Exception message as [`String`]
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn logical_xor_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    /// Perform unary `<<` operator and return new [`Value`] represent the old value after casted
    /// or Exception message as [`String`]
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn shl_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    /// Perform unary `>>` operator and return new [`Value`] represent the old value after casted
    /// or Exception message as [`String`]
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn shr_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    /// Perform unary `[I]` operator and return new [`Value`] represent the old value after casted
    /// or Exception message as [`String`]
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn index_op(&self, index: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    /// Perform unary `[S:E]` operator and return new [`Value`] represent the old value after casted
    /// or Exception message as [`String`]
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn slice_op(
        &self,
        start: &Option<Box<dyn Value>>,
        end: &Option<Box<dyn Value>>,
    ) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    /// Perform unary `=` operator and return new [`Value`] represent the old value after casted
    /// or Exception message as [`String`]
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn eq_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    /// Perform unary `!=` or `<>` operator and return new [`Value`] represent the old value after casted
    /// or Exception message as [`String`]
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn bang_eq_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    /// Perform unary `<=>` operator and return new [`Value`] represent the old value after casted
    /// or Exception message as [`String`]
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn null_safe_eq_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    /// Perform unary `>` operator and return new [`Value`] represent the old value after casted
    /// or Exception message as [`String`]
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn gt_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    /// Perform unary `>=` operator and return new [`Value`] represent the old value after casted
    /// or Exception message as [`String`]
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn gte_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    /// Perform unary `<` operator and return new [`Value`] represent the old value after casted
    /// or Exception message as [`String`]
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn lt_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    /// Perform unary `<=` operator and return new [`Value`] represent the old value after casted
    /// or Exception message as [`String`]
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn lte_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    /// Perform unary `NOT` operator and return new [`Value`] represent the old value after casted
    /// or Exception message as [`String`]
    fn not_op(&self) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    /// Perform unary `-` operator and return new [`Value`] represent the old value after casted
    /// or Exception message as [`String`]
    fn neg_op(&self) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    /// Perform unary `!` operator and return new [`Value`] represent the old value after casted
    /// or Exception message as [`String`]
    fn bang_op(&self) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    /// Perform `@>` operator and return new [`Value`] represent the old value after casted
    /// or Exception message as [`String`]
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn contains_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    /// Perform Cast operator and return new [`Value`] represent the old value after casted
    /// or Exception message as [`String`]
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn cast_op(&self, target_type: &Box<dyn DataType>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }
}

impl dyn Value {
    /// Return true if this value is [`TextValue`]
    pub fn is_text(&self) -> bool {
        self.as_any().downcast_ref::<TextValue>().is_some()
    }

    /// Return List of [`String`] represent the inner value of [`IntValue`]
    /// or None if this type it's called from wrong [`Value`]
    pub fn as_text(&self) -> Option<String> {
        if let Some(text_value) = self.as_any().downcast_ref::<TextValue>() {
            return Some(text_value.value.to_string());
        }
        None
    }

    /// Return true if this value is [`IntValue`]
    pub fn is_int(&self) -> bool {
        self.as_any().downcast_ref::<IntValue>().is_some()
    }

    /// Return List of [`i64`] represent the inner value of [`IntValue`]
    /// or None if this type it's called from wrong [`Value`]
    pub fn as_int(&self) -> Option<i64> {
        if let Some(int_value) = self.as_any().downcast_ref::<IntValue>() {
            return Some(int_value.value);
        }
        None
    }

    /// Return true if this value is [`FloatValue`]
    pub fn is_float(&self) -> bool {
        self.as_any().downcast_ref::<FloatValue>().is_some()
    }

    /// Return List of [`f64`] represent the inner value of [`FloatValue`]
    /// or None if this type it's called from wrong [`Value`]
    pub fn as_float(&self) -> Option<f64> {
        if let Some(float_value) = self.as_any().downcast_ref::<FloatValue>() {
            return Some(float_value.value);
        }
        None
    }

    /// Return true if this value is [`IntValue`] or [`FloatValue`]
    pub fn is_number(&self) -> bool {
        self.is_int() || self.is_float()
    }

    /// Return true if this value is [`BoolValue`]
    pub fn is_bool(&self) -> bool {
        self.as_any().downcast_ref::<BoolValue>().is_some()
    }

    /// Return List of [`bool`] represent the inner value of [`BoolValue`]
    /// or None if this type it's called from wrong [`Value`]
    pub fn as_bool(&self) -> Option<bool> {
        if let Some(bool_value) = self.as_any().downcast_ref::<BoolValue>() {
            return Some(bool_value.value);
        }
        None
    }

    /// Return true if this value is [`DateValue`]
    pub fn is_date(&self) -> bool {
        self.as_any().downcast_ref::<DateValue>().is_some()
    }

    /// Return List of [`i64`] represent the inner value of [`DateValue`]
    /// or None if this type it's called from wrong [`Value`]
    pub fn as_date(&self) -> Option<i64> {
        if let Some(date_value) = self.as_any().downcast_ref::<DateValue>() {
            return Some(date_value.value);
        }
        None
    }

    /// Return true if this value is [`TimeValue`]
    pub fn is_time(&self) -> bool {
        self.as_any().downcast_ref::<TimeValue>().is_some()
    }

    /// Return List of [`String`] represent the inner value of [`DateValue`]
    /// or None if this type it's called from wrong [`Value`]
    pub fn as_time(&self) -> Option<String> {
        if let Some(time_value) = self.as_any().downcast_ref::<DateValue>() {
            return Some(time_value.value.to_string());
        }
        None
    }

    /// Return true if this value is [`DateTimeValue`]
    pub fn is_date_time(&self) -> bool {
        self.as_any().downcast_ref::<DateTimeValue>().is_some()
    }

    /// Return List of [`i64`] represent the value of [`DateTimeValue`]
    /// or None if this type it's called from wrong [`Value`]
    pub fn as_date_time(&self) -> Option<i64> {
        if let Some(date_time_value) = self.as_any().downcast_ref::<DateTimeValue>() {
            return Some(date_time_value.value);
        }
        None
    }

    /// Return true if this value is [`ArrayValue`]
    pub fn is_array(&self) -> bool {
        self.as_any().downcast_ref::<ArrayValue>().is_some()
    }

    /// Return List of [`Value`] represent the inner values of [`ArrayValue`]
    /// or None if this type it's called from wrong [`Value`]
    pub fn as_array(&self) -> Option<Vec<Box<dyn Value>>> {
        if let Some(array_value) = self.as_any().downcast_ref::<ArrayValue>() {
            return Some(array_value.values.clone());
        }
        None
    }

    /// Return true if this value is [`RangeValue`]
    pub fn is_range(&self) -> bool {
        self.as_any().downcast_ref::<RangeValue>().is_some()
    }

    /// Return Tuple of two [`Value`] represent the start and the end of [`RangeValue`]
    /// or None if this type it's called from wrong [`Value`]
    pub fn as_range(&self) -> Option<(Box<dyn Value>, Box<dyn Value>)> {
        if let Some(range_value) = self.as_any().downcast_ref::<RangeValue>() {
            return Some((range_value.start.clone(), range_value.end.clone()));
        }
        None
    }

    /// Return true if this value is [`NullValue`]
    pub fn is_null(&self) -> bool {
        self.as_any().downcast_ref::<NullValue>().is_some()
    }

    /// Return true if this value is [`CompositeValue`]
    pub fn is_composite(&self) -> bool {
        self.as_any().downcast_ref::<CompositeValue>().is_some()
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
