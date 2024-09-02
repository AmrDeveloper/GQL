use gitql_core::dynamic_types::array_element_type_of_first_element;
use gitql_core::dynamic_types::type_of_first_element;
use gitql_core::signature::Function;
use gitql_core::signature::Signature;
use gitql_core::types::DataType;
use gitql_core::value::Value;

use std::collections::HashMap;

use rand::seq::SliceRandom;

#[inline(always)]
pub fn register_std_array_functions(map: &mut HashMap<&'static str, Function>) {
    map.insert("array_append", array_append);
    map.insert("array_cat", array_cat);
    map.insert("array_length", array_length);
    map.insert("array_shuffle", array_shuffle);
    map.insert("array_position", array_position);
    map.insert("array_dims", array_dims);
    map.insert("array_replace", array_replace);
}

#[inline(always)]
pub fn register_std_array_function_signatures(map: &mut HashMap<&'static str, Signature>) {
    map.insert(
        "array_append",
        Signature {
            parameters: vec![
                DataType::Array(Box::new(DataType::Any)),
                DataType::Dynamic(array_element_type_of_first_element),
            ],
            return_type: DataType::Dynamic(type_of_first_element),
        },
    );
    map.insert(
        "array_cat",
        Signature {
            parameters: vec![
                DataType::Array(Box::new(DataType::Any)),
                DataType::Dynamic(type_of_first_element),
            ],
            return_type: DataType::Dynamic(type_of_first_element),
        },
    );
    map.insert(
        "array_length",
        Signature {
            parameters: vec![DataType::Array(Box::new(DataType::Any))],
            return_type: DataType::Integer,
        },
    );
    map.insert(
        "array_shuffle",
        Signature {
            parameters: vec![DataType::Array(Box::new(DataType::Any))],
            return_type: DataType::Dynamic(type_of_first_element),
        },
    );
    map.insert(
        "array_position",
        Signature {
            parameters: vec![DataType::Array(Box::new(DataType::Any)), DataType::Any],
            return_type: DataType::Integer,
        },
    );
    map.insert(
        "array_dims",
        Signature {
            parameters: vec![DataType::Array(Box::new(DataType::Any))],
            return_type: DataType::Text,
        },
    );
    map.insert(
        "array_replace",
        Signature {
            parameters: vec![
                DataType::Array(Box::new(DataType::Any)),
                DataType::Dynamic(array_element_type_of_first_element),
                DataType::Dynamic(array_element_type_of_first_element),
            ],
            return_type: DataType::Dynamic(type_of_first_element),
        },
    );
}

pub fn array_append(inputs: &[Value]) -> Value {
    let mut array = inputs[0].as_array();
    let element = &inputs[1];
    array.push(element.to_owned());
    Value::Array(inputs[0].data_type(), array)
}

pub fn array_cat(inputs: &[Value]) -> Value {
    let mut first = inputs[0].as_array();
    let mut other = inputs[1].as_array();
    let mut result = Vec::with_capacity(first.len() + other.len());
    result.append(&mut first);
    result.append(&mut other);
    Value::Array(inputs[0].data_type(), result)
}

pub fn array_length(inputs: &[Value]) -> Value {
    let array = inputs[0].as_array();
    Value::Integer(array.len() as i64)
}

pub fn array_shuffle(inputs: &[Value]) -> Value {
    let array_type = inputs[0].data_type();
    let element_type = match array_type {
        DataType::Array(element_type) => *element_type,
        _ => DataType::Any,
    };
    let mut array: Vec<Value> = inputs[0].as_array();
    array.shuffle(&mut rand::thread_rng());
    Value::Array(element_type, array)
}

pub fn array_position(inputs: &[Value]) -> Value {
    let array = inputs[0].as_array();
    let elemnet = &inputs[1];
    if let Some(index) = array.iter().position(|r| r.equals(elemnet)) {
        return Value::Integer((index + 1) as i64);
    }
    Value::Null
}

pub fn array_dims(inputs: &[Value]) -> Value {
    let array_type = inputs[0].data_type();
    Value::Text(array_type.to_string())
}

pub fn array_replace(inputs: &[Value]) -> Value {
    let array_type = inputs[0].data_type();
    let mut array_values = inputs[0].as_array();
    let from = &inputs[1];
    let to = &inputs[2];
    for element in &mut array_values {
        if element.equals(from) {
            *element = to.clone();
        }
    }
    Value::Array(array_type, array_values)
}
