use std::any::Any;

use super::base::DataType;

#[derive(Clone)]
pub struct OptionType {
    pub base: Option<Box<dyn DataType>>,
}

impl OptionType {
    pub fn new(base: Option<Box<dyn DataType>>) -> Self {
        OptionType { base }
    }
}

impl DataType for OptionType {
    fn literal(&self) -> String {
        if let Some(base) = &self.base {
            return format!("{}?", base.literal());
        }
        "None".to_string()
    }

    fn equals(&self, other: &Box<dyn DataType>) -> bool {
        if other.is_any() {
            return true;
        }
        if let Some(base) = &self.base {
            return base.equals(other);
        }
        true
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
