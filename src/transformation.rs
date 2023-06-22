use crate::types::DataType;
use lazy_static::lazy_static;
use std::collections::HashMap;

type Transformation = fn(String) -> String;

pub struct Prototype {
    pub ret_type: DataType,
    pub parameters: Vec<DataType>,
}

lazy_static! {
    pub static ref TRANSFORMATIONS: HashMap<&'static str, Transformation> = {
        let mut map: HashMap<&'static str, Transformation> = HashMap::new();
        map.insert("lower", lower);
        map.insert("upper", upper);
        map.insert("trim", trim);
        map.insert("length", length);
        map
    };
}

lazy_static! {
    pub static ref TRANSFORMATIONS_PROTOS: HashMap<&'static str, Prototype> = {
        let mut map: HashMap<&'static str, Prototype> = HashMap::new();
        map.insert(
            "lower",
            Prototype {
                ret_type: DataType::Text,
                parameters: vec![DataType::Text],
            },
        );

        map.insert(
            "upper",
            Prototype {
                ret_type: DataType::Text,
                parameters: vec![DataType::Text],
            },
        );

        map.insert(
            "trim",
            Prototype {
                ret_type: DataType::Text,
                parameters: vec![DataType::Text],
            },
        );

        map.insert(
            "length",
            Prototype {
                ret_type: DataType::Text,
                parameters: vec![DataType::Text],
            },
        );
        map
    };
}

fn lower(input: String) -> String {
    return input.to_lowercase();
}

fn upper(input: String) -> String {
    return input.to_uppercase();
}

fn trim(input: String) -> String {
    return input.trim().to_string();
}

fn length(input: String) -> String {
    return input.len().to_string();
}
