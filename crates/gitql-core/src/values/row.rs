use std::any::Any;
use std::cmp::Ordering;

use gitql_ast::types::row::RowType;
use gitql_ast::types::DataType;

use super::base::Value;

#[derive(Clone)]
pub struct RowValue {
    pub columns: Vec<Box<dyn Value>>,
    pub row_type: RowType,
}

impl RowValue {
    pub fn new(columns: Vec<Box<dyn Value>>, row_type: RowType) -> Self {
        RowValue { columns, row_type }
    }
}

impl Value for RowValue {
    fn literal(&self) -> String {
        let mut str = String::new();
        str += "(";
        for (pos, element) in self.columns.iter().enumerate() {
            str += &element.literal();
            if pos + 1 != self.columns.len() {
                str += ", ";
            }
        }
        str += ")";
        str
    }

    fn equals(&self, other: &Box<dyn Value>) -> bool {
        if let Some(other_row) = other.as_any().downcast_ref::<RowValue>() {
            let data_type: Box<dyn DataType> = Box::new(other_row.row_type.clone());
            if !self.row_type.equals(&data_type) {
                return false;
            }

            let len = self.row_type.tuple.len();
            for i in 0..len {
                if !self.columns[i].eq(&other_row.columns[i]) {
                    return false;
                }
            }

            return true;
        }
        false
    }

    fn compare(&self, _other: &Box<dyn Value>) -> Option<Ordering> {
        None
    }

    fn data_type(&self) -> Box<dyn DataType> {
        Box::new(self.row_type.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
