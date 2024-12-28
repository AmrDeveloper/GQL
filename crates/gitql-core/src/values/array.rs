use std::any::Any;
use std::cmp::Ordering;

use gitql_ast::types::array::ArrayType;
use gitql_ast::types::base::DataType;

use super::base::Value;
use super::boolean::BoolValue;
use super::integer::IntValue;

#[derive(Clone)]
pub struct ArrayValue {
    pub values: Vec<Box<dyn Value>>,
    pub base_type: Box<dyn DataType>,
}

impl ArrayValue {
    pub fn new(values: Vec<Box<dyn Value>>, base_type: Box<dyn DataType>) -> Self {
        ArrayValue { values, base_type }
    }

    pub fn empty(base_type: Box<dyn DataType>) -> Self {
        ArrayValue {
            values: Vec::default(),
            base_type,
        }
    }

    pub fn add_element(mut self, element: Box<dyn Value>) -> Self {
        self.values.push(element);
        self
    }
}

impl Value for ArrayValue {
    fn literal(&self) -> String {
        let mut str = String::new();
        let last_position = self.values.len() - 1;
        str += "[";
        for (pos, element) in self.values.iter().enumerate() {
            str += &element.literal();
            if pos != last_position {
                str += ", ";
            }
        }
        str += "]";
        str
    }

    fn equals(&self, other: &Box<dyn Value>) -> bool {
        if let Some(other_array) = other.as_any().downcast_ref::<ArrayValue>() {
            if !self.base_type.equals(&other_array.base_type) {
                return false;
            }

            let self_values = &self.values;
            let other_values = &other_array.values;
            if self.values.len() != other_values.len() {
                return false;
            }

            for i in 0..self.values.len() {
                if !self_values[i].equals(&other_values[i]) {
                    return false;
                }
            }
        }
        false
    }

    fn compare(&self, _other: &Box<dyn Value>) -> Option<Ordering> {
        None
    }

    fn data_type(&self) -> Box<dyn DataType> {
        Box::new(ArrayType {
            base: self.base_type.clone(),
        })
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn index_op(&self, index: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(index) = index.as_any().downcast_ref::<IntValue>() {
            if (index.value < 1) || (index.value as usize > self.values.len()) {
                return Err("Array Index must be between 1 and length of Array".to_string());
            }

            let array_index = (index.value - 1) as usize;
            return Ok(self.values[array_index].clone());
        }
        Err("Unexpected Array Index type".to_string())
    }

    fn slice_op(
        &self,
        start: &Option<Box<dyn Value>>,
        end: &Option<Box<dyn Value>>,
    ) -> Result<Box<dyn Value>, String> {
        if start.is_none() && end.is_none() {
            return Ok(Box::new(self.clone()));
        }

        let mut start_index: usize = 0;

        if start.is_some() {
            if let Some(start_value) = start.clone().unwrap().as_any().downcast_ref::<IntValue>() {
                if start_value.value < 1 || start_value.value >= self.values.len() as i64 {
                    return Err("Slice start must be between 1 and length of Array".to_string());
                }
                start_index = start_value.value as usize;
            }
        }

        let mut end_index: usize = self.values.len();
        if end.is_some() {
            if let Some(end_value) = end.clone().unwrap().as_any().downcast_ref::<IntValue>() {
                if end_value.value < start_index as i64
                    || end_value.value > self.values.len() as i64
                {
                    return Err("Slice end must be between start and length of Array".to_string());
                }
                end_index = end_value.value as usize;
            }
        }

        let slice = self.values[start_index..end_index].to_vec();
        Ok(Box::new(ArrayValue {
            values: slice,
            base_type: self.base_type.clone(),
        }))
    }

    fn logical_or_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_array) = other.as_any().downcast_ref::<ArrayValue>() {
            for value in self.values.iter() {
                for other_value in other_array.values.iter() {
                    if value.equals(other_value) {
                        return Ok(Box::new(BoolValue { value: true }));
                    }
                }
            }
            return Ok(Box::new(BoolValue::new_false()));
        }
        Err("Unexpected Array overlap type".to_string())
    }

    fn contains_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        for value in self.values.iter() {
            if value.equals(other) {
                return Ok(Box::new(BoolValue { value: true }));
            }
        }

        Ok(Box::new(BoolValue::new_false()))
    }
}
