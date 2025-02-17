use std::any::Any;
use std::cmp::Ordering;

use gitql_ast::operator::GroupComparisonOperator;
use regex::Regex;
use regex::RegexBuilder;

use gitql_ast::types::text::TextType;
use gitql_ast::types::DataType;

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

impl TextValue {
    pub fn new(value: String) -> Self {
        TextValue { value }
    }

    pub fn empty() -> Self {
        TextValue {
            value: String::default(),
        }
    }
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
            return Ok(Box::new(BoolValue::new(self.value == other_text.value)));
        }
        Err("Unexpected type to perform `=` with".to_string())
    }

    fn group_eq_op(
        &self,
        other: &Box<dyn Value>,
        group_op: &GroupComparisonOperator,
    ) -> Result<Box<dyn Value>, String> {
        if other.is_array_of(|element_type| element_type.is_text()) {
            let elements = &other.as_array().unwrap();
            let mut matches_count = 0;
            for element in elements.iter() {
                if self.value == element.as_text().unwrap() {
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
        if let Some(other_text) = other.as_any().downcast_ref::<TextValue>() {
            return Ok(Box::new(BoolValue::new(self.value != other_text.value)));
        }
        Err("Unexpected type to perform `!=` with".to_string())
    }

    fn group_bang_eq_op(
        &self,
        other: &Box<dyn Value>,
        group_op: &GroupComparisonOperator,
    ) -> Result<Box<dyn Value>, String> {
        if other.is_array_of(|element_type| element_type.is_text()) {
            let elements = &other.as_array().unwrap();
            let mut matches_count = 0;
            for element in elements.iter() {
                if self.value != element.as_text().unwrap() {
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
        if let Some(other_text) = other.as_any().downcast_ref::<TextValue>() {
            return Ok(Box::new(BoolValue::new(self.value > other_text.value)));
        }
        Err("Unexpected type to perform `>` with".to_string())
    }

    fn group_gt_op(
        &self,
        other: &Box<dyn Value>,
        group_op: &GroupComparisonOperator,
    ) -> Result<Box<dyn Value>, String> {
        if other.is_array_of(|element_type| element_type.is_text()) {
            let elements = &other.as_array().unwrap();
            let mut matches_count = 0;
            for element in elements.iter() {
                if self.value > element.as_text().unwrap() {
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
        if let Some(other_text) = other.as_any().downcast_ref::<TextValue>() {
            return Ok(Box::new(BoolValue::new(self.value >= other_text.value)));
        }
        Err("Unexpected type to perform `>=` with".to_string())
    }

    fn group_gte_op(
        &self,
        other: &Box<dyn Value>,
        group_op: &GroupComparisonOperator,
    ) -> Result<Box<dyn Value>, String> {
        if other.is_array_of(|element_type| element_type.is_text()) {
            let elements = &other.as_array().unwrap();
            let mut matches_count = 0;
            for element in elements.iter() {
                if self.value >= element.as_text().unwrap() {
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
        if let Some(other_text) = other.as_any().downcast_ref::<TextValue>() {
            return Ok(Box::new(BoolValue::new(self.value < other_text.value)));
        }
        Err("Unexpected type to perform `<` with".to_string())
    }

    fn group_lt_op(
        &self,
        other: &Box<dyn Value>,
        group_op: &GroupComparisonOperator,
    ) -> Result<Box<dyn Value>, String> {
        if other.is_array_of(|element_type| element_type.is_text()) {
            let elements = &other.as_array().unwrap();
            let mut matches_count = 0;
            for element in elements.iter() {
                if self.value < element.as_text().unwrap() {
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
        if let Some(other_text) = other.as_any().downcast_ref::<TextValue>() {
            return Ok(Box::new(BoolValue::new(self.value <= other_text.value)));
        }
        Err("Unexpected type to perform `<=` with".to_string())
    }

    fn group_lte_op(
        &self,
        other: &Box<dyn Value>,
        group_op: &GroupComparisonOperator,
    ) -> Result<Box<dyn Value>, String> {
        if other.is_array_of(|element_type| element_type.is_text()) {
            let elements = &other.as_array().unwrap();
            let mut matches_count = 0;
            for element in elements.iter() {
                if self.value <= element.as_text().unwrap() {
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

    fn like_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        let pattern_text = other.as_text().unwrap();
        let pattern = &format!(
            "^{}$",
            pattern_text
                .to_lowercase()
                .replace('%', ".*")
                .replace('_', ".")
        );

        let regex_builder = RegexBuilder::new(pattern)
            .multi_line(true)
            .unicode(true)
            .build();

        match regex_builder {
            Ok(regex) => {
                let is_match = regex.is_match(&self.value.to_lowercase());
                Ok(Box::new(BoolValue { value: is_match }))
            }
            Err(error_message) => Err(error_message.to_string()),
        }
    }

    fn glob_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        let pattern_text = other.as_text().unwrap();
        let pattern = &format!(
            "^{}$",
            pattern_text
                .replace('.', "\\.")
                .replace('*', ".*")
                .replace('?', ".")
        );

        match Regex::new(pattern) {
            Ok(regex) => {
                let is_match = regex.is_match(&self.value);
                Ok(Box::new(BoolValue { value: is_match }))
            }
            Err(error_message) => Err(error_message.to_string()),
        }
    }

    fn regexp_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        let pattern_text = other.as_text().unwrap();
        let pattern = &format!(
            "^{}$",
            pattern_text
                .to_lowercase()
                .replace('%', ".*")
                .replace('_', ".")
        );

        let regex_builder = RegexBuilder::new(pattern)
            .multi_line(true)
            .unicode(true)
            .build();

        match regex_builder {
            Ok(regex) => {
                let is_match = regex.is_match(&self.value.to_lowercase());
                Ok(Box::new(BoolValue { value: is_match }))
            }
            Err(error_message) => Err(error_message.to_string()),
        }
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
