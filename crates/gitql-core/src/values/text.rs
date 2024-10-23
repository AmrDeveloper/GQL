use std::any::Any;
use std::cmp::Ordering;

use gitql_ast::types::base::DataType;
use gitql_ast::types::text::TextType;

use super::base::Value;
use super::boolean::BoolValue;
use super::converters::string_literal_to_boolean;
use super::converters::string_literal_to_date;
use super::converters::string_literal_to_date_time;
use super::converters::string_literal_to_time;

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

    fn eq_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_text) = other.as_any().downcast_ref::<TextValue>() {
            let are_equals = self.value == other_text.value;
            return Ok(Box::new(BoolValue { value: are_equals }));
        }
        Err("Unexpected type to perform `=` with".to_string())
    }

    fn cast_op(&self, target_type: &Box<dyn DataType>) -> Result<Box<dyn Value>, String> {
        if target_type.is_bool() {
            return Ok(string_literal_to_boolean(&self.value));
        }

        if target_type.is_time() {
            return Ok(string_literal_to_time(&self.value));
        }

        if target_type.is_date() {
            return Ok(string_literal_to_date(&self.value));
        }

        if target_type.is_date_time() {
            return Ok(string_literal_to_date_time(&self.value));
        }

        Err("Unexpected value to perform `CAST` with".to_string())
    }
}
