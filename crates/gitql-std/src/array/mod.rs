use gitql_core::dynamic_types::array_element_type;
use gitql_core::dynamic_types::array_of_type;
use gitql_core::dynamic_types::first_element_type;
use gitql_core::dynamic_types::second_element_type;
use gitql_core::signature::Function;
use gitql_core::signature::Signature;
use gitql_core::types::DataType;
use gitql_core::value::Value;

use std::collections::HashMap;

use rand::seq::SliceRandom;

#[inline(always)]
pub fn register_std_array_functions(map: &mut HashMap<&'static str, Function>) {
    map.insert("array_append", array_append);
    map.insert("array_prepend", array_prepend);
    map.insert("array_remove", array_remove);
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
                DataType::Dynamic(|elements| array_element_type(first_element_type(elements))),
            ],
            return_type: DataType::Dynamic(first_element_type),
        },
    );
    map.insert(
        "array_prepend",
        Signature {
            parameters: vec![
                DataType::Any,
                DataType::Dynamic(|elements| array_of_type(first_element_type(elements))),
            ],
            return_type: DataType::Dynamic(second_element_type),
        },
    );
    map.insert(
        "array_remove",
        Signature {
            parameters: vec![
                DataType::Array(Box::new(DataType::Any)),
                DataType::Dynamic(|elements| array_element_type(first_element_type(elements))),
            ],
            return_type: DataType::Dynamic(first_element_type),
        },
    );
    map.insert(
        "array_cat",
        Signature {
            parameters: vec![
                DataType::Array(Box::new(DataType::Any)),
                DataType::Dynamic(first_element_type),
            ],
            return_type: DataType::Dynamic(first_element_type),
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
            return_type: DataType::Dynamic(first_element_type),
        },
    );
    map.insert(
        "array_position",
        Signature {
            parameters: vec![
                DataType::Array(Box::new(DataType::Any)),
                DataType::Dynamic(|elements| array_element_type(first_element_type(elements))),
            ],
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
                DataType::Dynamic(|elements| array_element_type(first_element_type(elements))),
                DataType::Dynamic(|elements| array_element_type(first_element_type(elements))),
            ],
            return_type: DataType::Dynamic(first_element_type),
        },
    );
}

pub fn array_append(inputs: &[Value]) -> Value {
    let mut array = inputs[0].as_array();
    let element = &inputs[1];
    array.push(element.to_owned());
    Value::Array(inputs[0].data_type(), array)
}

pub fn array_prepend(inputs: &[Value]) -> Value {
    let element = &inputs[0];
    let mut array = inputs[1].as_array();
    array.insert(0, element.clone());
    Value::Array(inputs[1].data_type(), array)
}

pub fn array_remove(inputs: &[Value]) -> Value {
    let array = inputs[0].as_array();
    let element_to_remove = &inputs[1];
    let array_after_remove = array
        .into_iter()
        .filter(|element| !element_to_remove.equals(element))
        .collect();
    Value::Array(inputs[0].data_type(), array_after_remove)
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
