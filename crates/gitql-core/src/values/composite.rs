use std::any::Any;
use std::cmp::Ordering;
use std::collections::HashMap;

use gitql_ast::types::base::DataType;
use gitql_ast::types::composite::CompositeType;

use indexmap::IndexMap;

use super::base::Value;

#[derive(Clone)]
pub struct CompositeValue {
    pub name: String,
    pub members: IndexMap<String, Box<dyn Value>>,
}

impl CompositeValue {
    pub fn new(name: String, members: IndexMap<String, Box<dyn Value>>) -> Self {
        CompositeValue { name, members }
    }

    pub fn empty(name: String) -> Self {
        CompositeValue {
            name,
            members: IndexMap::default(),
        }
    }

    pub fn add_member(mut self, name: String, value: Box<dyn Value>) -> Self {
        self.members.insert(name, value);
        self
    }
}

impl Value for CompositeValue {
    fn literal(&self) -> String {
        let mut str = String::new();
        let last_position = self.members.len() - 1;
        str += "(";
        for (pos, member) in self.members.iter().enumerate() {
            str += &member.1.literal();
            if pos != last_position {
                str += ", ";
            }
        }
        str += ")";
        str
    }

    fn equals(&self, other: &Box<dyn Value>) -> bool {
        if let Some(other_composite) = other.as_any().downcast_ref::<CompositeValue>() {
            return self.name.eq(&other_composite.name)
                && self.members.eq(&other_composite.members);
        }
        false
    }

    fn compare(&self, _other: &Box<dyn Value>) -> Option<Ordering> {
        None
    }

    fn data_type(&self) -> Box<dyn DataType> {
        let name = self.name.to_string();
        let mut members: HashMap<String, Box<dyn DataType>> = HashMap::new();
        for member in self.members.iter() {
            members.insert(member.0.to_string(), member.1.data_type().clone());
        }
        Box::new(CompositeType { name, members })
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
