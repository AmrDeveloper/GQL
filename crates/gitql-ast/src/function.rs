use crate::types::DataType;
use crate::value::Value;

use lazy_static::lazy_static;
use std::collections::HashMap;

type Function = fn(Vec<Value>) -> Value;

pub struct Prototype {
    pub parameters: Vec<DataType>,
    pub result: DataType,
}

lazy_static! {
    pub static ref FUNCTIONS: HashMap<&'static str, Function> = {
        let mut map: HashMap<&'static str, Function> = HashMap::new();
        // String functions
        map.insert("lower", text_lowercase);
        map.insert("upper", text_uppercase);
        map.insert("reverse", text_reverse);
        map.insert("replicate", text_replicate);
        map.insert("trim", text_trim);
        map.insert("len", text_len);
        map
    };
}

lazy_static! {
    pub static ref PROTOTYPES: HashMap<&'static str, Prototype> = {
        let mut map: HashMap<&'static str, Prototype> = HashMap::new();
        map.insert(
            "lower",
            Prototype {
                parameters: vec![DataType::Text],
                result: DataType::Text,
            },
        );
        map.insert(
            "upper",
            Prototype {
                parameters: vec![DataType::Text],
                result: DataType::Text,
            },
        );
        map.insert(
            "reverse",
            Prototype {
                parameters: vec![DataType::Text],
                result: DataType::Text,
            },
        );
        map.insert(
            "replicate",
            Prototype {
                parameters: vec![DataType::Text, DataType::Number],
                result: DataType::Text,
            },
        );
        map.insert(
            "trim",
            Prototype {
                parameters: vec![DataType::Text],
                result: DataType::Text,
            },
        );
        map.insert(
            "len",
            Prototype {
                parameters: vec![DataType::Text],
                result: DataType::Number,
            },
        );
        map
    };
}

fn text_lowercase(inputs: Vec<Value>) -> Value {
    return Value::Text(inputs[0].as_text().to_lowercase());
}

fn text_uppercase(inputs: Vec<Value>) -> Value {
    return Value::Text(inputs[0].as_text().to_uppercase());
}

fn text_reverse(inputs: Vec<Value>) -> Value {
    return Value::Text(inputs[0].as_text().chars().rev().collect::<String>());
}

fn text_replicate(inputs: Vec<Value>) -> Value {
    let str = inputs[0].as_text();
    let count = inputs[1].as_number() as usize;
    return Value::Text(str.repeat(count));
}

fn text_trim(inputs: Vec<Value>) -> Value {
    return Value::Text(inputs[0].as_text().trim().to_string());
}

fn text_len(inputs: Vec<Value>) -> Value {
    return Value::Number(inputs[0].as_text().len() as i64);
}
