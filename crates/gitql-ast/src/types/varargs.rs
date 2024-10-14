use std::any::Any;

use super::base::DataType;

#[derive(Clone)]
pub struct VarargsType {
    pub base: Box<dyn DataType>,
}

impl DataType for VarargsType {
    fn literal(&self) -> String {
        format!("...{}", self.base.literal())
    }

    fn equals(&self, other: &Box<dyn DataType>) -> bool {
        other.is_any() || self.base.equals(other)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
