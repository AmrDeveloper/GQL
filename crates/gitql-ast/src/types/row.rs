use std::any::Any;

use super::base::DataType;

#[derive(Clone)]
pub struct RowType {
    pub tuple: Vec<Box<dyn DataType>>,
}

impl RowType {
    pub fn new(tuple: Vec<Box<dyn DataType>>) -> Self {
        RowType { tuple }
    }
}

impl DataType for RowType {
    fn literal(&self) -> String {
        let mut str = String::new();
        let last_position = self.tuple.len() - 1;
        str += "Row(";
        for (pos, data_type) in self.tuple.iter().enumerate() {
            str += &data_type.literal();
            if pos != last_position {
                str += ", ";
            }
        }
        str += ")";
        str
    }

    fn equals(&self, other: &Box<dyn DataType>) -> bool {
        let row_type: Box<dyn DataType> = Box::new(self.clone());
        if other.is_any() || other.is_variant_contains(&row_type) {
            return true;
        }

        if let Some(other_row) = other.as_any().downcast_ref::<RowType>() {
            if self.tuple.len() != other_row.tuple.len() {
                return false;
            }

            let len = self.tuple.len();
            for i in 0..len {
                if !self.tuple[i].eq(&other_row.tuple[i]) {
                    return false;
                }
            }

            return true;
        }

        false
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
