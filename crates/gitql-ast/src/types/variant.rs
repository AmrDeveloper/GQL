use std::any::Any;

use super::base::DataType;

#[derive(Clone)]
pub struct VariantType {
    pub variants: Vec<Box<dyn DataType>>,
}

impl DataType for VariantType {
    fn literal(&self) -> String {
        let mut str = String::new();
        let last_position = self.variants.len() - 1;
        str += "[";
        for (pos, data_type) in self.variants.iter().enumerate() {
            str += &data_type.literal();
            if pos != last_position {
                str += " | ";
            }
        }
        str += "]";
        str
    }

    fn equals(&self, other: &Box<dyn DataType>) -> bool {
        if other.is_any() {
            return true;
        }

        for variant in self.variants.iter() {
            if variant.equals(other) {
                return true;
            }
        }

        false
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
