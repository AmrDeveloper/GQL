use std::any::Any;
use std::cmp::Ordering;

use gitql_ast::types::base::DataType;
use gitql_ast::types::text::TextType;

use super::base::Value;
use super::boolean::BoolValue;

#[derive(Clone)]
pub struct TextValue {
    pub value: String,
}

impl Value for TextValue {
    fn literal(&self) -> String {
        self.value.to_string()
    }

    fn equals(&self, other: &Box<dyn Value>) -> bool {
        if let Some(other_text) = other.as_any().downcast_ref::<TextValue>() {
            return self.value == other_text.value;
        }
        false
    }

    fn compare(&self, other: &Box<dyn Value>) -> Option<Ordering> {
        if let Some(other_text) = other.as_any().downcast_ref::<TextValue>() {
            return self.value.partial_cmp(&other_text.value);
        }
        None
    }

    fn data_type(&self) -> Box<dyn DataType> {
        Box::new(TextType)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn perform_eq_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_text) = other.as_any().downcast_ref::<TextValue>() {
            let are_equals = self.value == other_text.value;
            return Ok(Box::new(BoolValue { value: are_equals }));
        }
        Err("Unexpected type to perform `=` with".to_string())
    }

    fn perform_cast_op(&self, target_type: &Box<dyn DataType>) -> Result<Box<dyn Value>, String> {
        if target_type.is_bool() {
            if matches!(self.value.as_str(), "t" | "true" | "y" | "yes" | "1") {
                return Ok(Box::new(BoolValue { value: true }));
            }

            if matches!(self.value.as_str(), "f" | "false" | "n" | "no" | "0") {
                return Ok(Box::new(BoolValue { value: false }));
            }
        }

        Err("Unexpected value to perform `CAST` with".to_string())
    }
}
