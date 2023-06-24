use crate::types::DataType;

use lazy_static::lazy_static;
use std::collections::HashMap;

type Transformation = fn(String) -> String;

pub struct TransformationPrototype {
    pub parameters: Vec<DataType>,
    pub result: DataType,
}

lazy_static! {
    pub static ref TRANSFORMATIONS: HashMap<&'static str, Transformation> = {
        let mut map: HashMap<&'static str, Transformation> = HashMap::new();
        map.insert("lower", transformation_lower);
        map.insert("upper", transformation_upper);
        map.insert("trim", transformtion_trim);
        map.insert("length", transformation_length);
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
            "length",
            TransformationPrototype {
                parameters: vec![DataType::Text],
                result: DataType::Text,
            },
        );
        map
    };
}

fn transformation_lower(input: String) -> String {
    return input.to_lowercase();
}

fn transformation_upper(input: String) -> String {
    return input.to_uppercase();
}

fn transformtion_trim(input: String) -> String {
    return input.trim().to_string();
}

fn transformation_length(input: String) -> String {
    return input.len().to_string();
}
