use crate::types::DataType;
use crate::value::Value;

use lazy_static::lazy_static;
use std::collections::HashMap;

type Transformation = fn(Value) -> Value;

pub struct TransformationPrototype {
    pub parameters: Vec<DataType>,
    pub result: DataType,
}

lazy_static! {
    pub static ref TRANSFORMATIONS: HashMap<&'static str, Transformation> = {
        let mut map: HashMap<&'static str, Transformation> = HashMap::new();
        map.insert("lower", text_lowercase);
        map.insert("upper", text_uppercase);
        map.insert("trim", text_trim);
        map.insert("len", text_len);
        map
    };
}

lazy_static! {
    pub static ref TRANSFORMATIONS_PROTOS: HashMap<&'static str, TransformationPrototype> = {
        let mut map: HashMap<&'static str, TransformationPrototype> = HashMap::new();
        map.insert(
            "lower",
            TransformationPrototype {
                parameters: vec![DataType::Text],
                result: DataType::Text,
            },
        );

        map.insert(
            "upper",
            TransformationPrototype {
                parameters: vec![DataType::Text],
                result: DataType::Text,
            },
        );

        map.insert(
            "trim",
            TransformationPrototype {
                parameters: vec![DataType::Text],
                result: DataType::Text,
            },
        );

        map.insert(
            "len",
            TransformationPrototype {
                parameters: vec![DataType::Text],
                result: DataType::Text,
            },
        );
        map
    };
}

fn text_lowercase(input: Value) -> Value {
    return Value::Text(input.as_text().to_lowercase());
}

fn text_uppercase(input: Value) -> Value {
    return Value::Text(input.as_text().to_uppercase());
}

fn text_trim(input: Value) -> Value {
    return Value::Text(input.as_text().trim().to_string());
}

fn text_len(input: Value) -> Value {
    return Value::Number(input.as_text().len() as i64);
}
