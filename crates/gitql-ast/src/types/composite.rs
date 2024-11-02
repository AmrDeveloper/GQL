use std::any::Any;
use std::collections::HashMap;

use super::base::DataType;

#[derive(Clone)]
pub struct CompositeType {
    pub name: String,
    pub members: HashMap<String, Box<dyn DataType>>,
}

impl DataType for CompositeType {
    fn literal(&self) -> String {
        self.name.to_string()
    }

    fn equals(&self, other: &Box<dyn DataType>) -> bool {
        if other.is_any() {
            return true;
        }

        let composite_type: Box<dyn DataType> = Box::new(self.clone());
        if other.is_variant_contains(&composite_type) {
            return true;
        }

        if let Some(other_composite) = other.as_any().downcast_ref::<CompositeType>() {
            if self.name.ne(&other_composite.name) {
                return false;
            }

            return self.members.eq(&other_composite.members);
        }

        false
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
