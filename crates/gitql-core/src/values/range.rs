use std::any::Any;
use std::cmp::Ordering;

use gitql_ast::types::range::RangeType;
use gitql_ast::types::DataType;

use super::base::Value;
use super::boolean::BoolValue;

#[derive(Clone)]
pub struct RangeValue {
    pub lower: Box<dyn Value>,
    pub upper: Box<dyn Value>,
    pub base_type: Box<dyn DataType>,
    pub is_lower_bound_inclusive: bool,
    pub is_upper_bound_inclusive: bool,
}

impl RangeValue {
    pub fn new(lower: Box<dyn Value>, upper: Box<dyn Value>, base_type: Box<dyn DataType>) -> Self {
        RangeValue {
            lower,
            upper,
            base_type,
            is_lower_bound_inclusive: true,
            is_upper_bound_inclusive: false,
        }
    }
}

impl Value for RangeValue {
    fn literal(&self) -> String {
        let lower_bound = if self.is_lower_bound_inclusive {
            '['
        } else {
            '('
        };

        let upper_bound = if self.is_upper_bound_inclusive {
            ']'
        } else {
            ')'
        };

        format!(
            "{lower_bound}{}..{}{upper_bound}",
            self.lower.literal(),
            self.upper.literal()
        )
    }

    fn equals(&self, other: &Box<dyn Value>) -> bool {
        if let Some(other_range) = other.as_any().downcast_ref::<RangeValue>() {
            return self.base_type.equals(&other_range.base_type)
                && self.lower.equals(&other_range.lower)
                && self.upper.equals(&other_range.upper)
                && self.is_lower_bound_inclusive == other_range.is_lower_bound_inclusive
                && self.is_upper_bound_inclusive == other_range.is_upper_bound_inclusive;
        }
        false
    }

    fn compare(&self, _other: &Box<dyn Value>) -> Option<Ordering> {
        None
    }

    fn data_type(&self) -> Box<dyn DataType> {
        Box::new(RangeType::new(self.base_type.clone()))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn logical_and_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_range) = other.as_any().downcast_ref::<RangeValue>() {
            if !self.data_type().equals(&other.data_type()) {
                return Err("Overlap operator expect both Ranges to have the same type".to_string());
            }

            let compare_lowers = self.lower.compare(&other_range.lower).unwrap();
            let max_lower = if (self.is_lower_bound_inclusive
                == other_range.is_lower_bound_inclusive
                && compare_lowers.is_ge())
                || compare_lowers.is_gt()
            {
                &self.lower
            } else {
                &other_range.lower
            };

            let compare_uppers = self.upper.compare(&other_range.upper).unwrap();
            let max_upper = if (self.is_lower_bound_inclusive
                == other_range.is_lower_bound_inclusive
                && compare_uppers.is_le())
                || compare_uppers.is_lt()
            {
                &self.upper
            } else {
                &other_range.upper
            };

            let is_overlap = max_upper.compare(max_lower).unwrap().is_gt();
            return Ok(Box::new(BoolValue::new(is_overlap)));
        }
        Err("Unexpected type to perform `Range Overlap &&` with".to_string())
    }

    fn contains_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_range) = other.as_any().downcast_ref::<RangeValue>() {
            let is_lower_in_range =
            // Current range has no lower boundraies
            if self.lower.is_null() {
                true
            }
            // Current range has lower boundraies, but the other range hasn't
            else if other_range.lower.is_null() {
                false
            } else {
                let compare_lowers = other_range.lower.compare(&self.lower).unwrap();
                if self.is_lower_bound_inclusive || (self.is_upper_bound_inclusive == other_range.is_upper_bound_inclusive)
                    { compare_lowers.is_ge() } else  { compare_lowers.is_gt() }
            };

            let is_range_contains = is_lower_in_range &&
            // Current range has no upper boundraies
            if self.upper.is_null() {
                true
            }
            // Current range has upper boundraies, but the other range hasn't
            else if other_range.upper.is_null() {
                false
            } else {
                let compare_uppers = other_range.upper.compare(&self.upper).unwrap();
                if self.is_upper_bound_inclusive || (self.is_upper_bound_inclusive == other_range.is_upper_bound_inclusive)
                    { compare_uppers.is_le() } else  { compare_uppers.is_lt() }
            };

            return Ok(Box::new(BoolValue::new(is_range_contains)));
        }

        if self.base_type.equals(&other.data_type()) {
            let is_lower_in = self.lower.is_null()
                || if self.is_lower_bound_inclusive {
                    other.compare(&self.lower).unwrap().is_ge()
                } else {
                    other.compare(&self.lower).unwrap().is_gt()
                };
            let is_upper_in = self.upper.is_null()
                || if self.is_upper_bound_inclusive {
                    other.compare(&self.upper).unwrap().is_le()
                } else {
                    other.compare(&self.upper).unwrap().is_lt()
                };
            return Ok(Box::new(BoolValue::new(is_lower_in && is_upper_in)));
        }

        Err("Unexpected type to perform `Range contains @>` with".to_string())
    }
}
