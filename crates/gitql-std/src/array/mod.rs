use std::collections::HashMap;

use gitql_ast::types::any::AnyType;
use gitql_ast::types::array::ArrayType;
use gitql_ast::types::dynamic::DynamicType;
use gitql_ast::types::integer::IntType;
use gitql_ast::types::text::TextType;
use gitql_core::signature::Signature;
use gitql_core::signature::StandardFunction;
use gitql_core::values::array::ArrayValue;
use gitql_core::values::integer::IntValue;
use gitql_core::values::null::NullValue;
use gitql_core::values::text::TextValue;
use gitql_core::values::Value;

use crate::meta_types::array_element_type;
use crate::meta_types::array_of_type;
use crate::meta_types::first_element_type;
use crate::meta_types::second_element_type;

use rand::seq::SliceRandom;

#[inline(always)]
pub fn register_std_array_functions(map: &mut HashMap<&'static str, StandardFunction>) {
    map.insert("array_append", array_append);
    map.insert("array_prepend", array_prepend);
    map.insert("array_remove", array_remove);
    map.insert("array_cat", array_cat);
    map.insert("array_length", array_length);
    map.insert("array_shuffle", array_shuffle);
    map.insert("array_position", array_position);
    map.insert("array_positions", array_positions);
    map.insert("array_dims", array_dims);
    map.insert("array_replace", array_replace);
    map.insert("trim_array", array_trim);
    map.insert("cardinality", array_cardinality);
}

#[inline(always)]
pub fn register_std_array_function_signatures(map: &mut HashMap<&'static str, Signature>) {
    map.insert(
        "array_append",
        Signature {
            parameters: vec![
                Box::new(ArrayType::new(Box::new(AnyType))),
                Box::new(DynamicType {
                    function: |elements| array_element_type(first_element_type(elements)),
                }),
            ],
            return_type: Box::new(DynamicType::new(first_element_type)),
        },
    );
    map.insert(
        "array_prepend",
        Signature {
            parameters: vec![
                Box::new(AnyType),
                Box::new(DynamicType {
                    function: |elements| array_of_type(first_element_type(elements)),
                }),
            ],
            return_type: Box::new(DynamicType::new(second_element_type)),
        },
    );
    map.insert(
        "array_remove",
        Signature {
            parameters: vec![
                Box::new(ArrayType::new(Box::new(AnyType))),
                Box::new(DynamicType {
                    function: |elements| array_element_type(first_element_type(elements)),
                }),
            ],
            return_type: Box::new(DynamicType::new(first_element_type)),
        },
    );
    map.insert(
        "array_cat",
        Signature {
            parameters: vec![
                Box::new(ArrayType::new(Box::new(AnyType))),
                Box::new(DynamicType::new(first_element_type)),
            ],
            return_type: Box::new(DynamicType::new(first_element_type)),
        },
    );
    map.insert(
        "array_length",
        Signature {
            parameters: vec![Box::new(ArrayType {
                base: Box::new(AnyType),
            })],
            return_type: Box::new(IntType),
        },
    );
    map.insert(
        "array_shuffle",
        Signature {
            parameters: vec![Box::new(ArrayType {
                base: Box::new(AnyType),
            })],
            return_type: Box::new(DynamicType {
                function: first_element_type,
            }),
        },
    );
    map.insert(
        "array_position",
        Signature {
            parameters: vec![
                Box::new(ArrayType {
                    base: Box::new(AnyType),
                }),
                Box::new(DynamicType {
                    function: |elements| array_element_type(first_element_type(elements)),
                }),
            ],
            return_type: Box::new(IntType),
        },
    );
    map.insert(
        "array_dims",
        Signature {
            parameters: vec![Box::new(ArrayType {
                base: Box::new(AnyType),
            })],
            return_type: Box::new(TextType),
        },
    );
    map.insert(
        "array_replace",
        Signature {
            parameters: vec![
                Box::new(ArrayType {
                    base: Box::new(AnyType),
                }),
                Box::new(DynamicType {
                    function: |elements| array_element_type(first_element_type(elements)),
                }),
                Box::new(DynamicType {
                    function: |elements| array_element_type(first_element_type(elements)),
                }),
            ],
            return_type: Box::new(DynamicType {
                function: first_element_type,
            }),
        },
    );
    map.insert(
        "trim_array",
        Signature {
            parameters: vec![
                Box::new(ArrayType {
                    base: Box::new(AnyType),
                }),
                Box::new(IntType),
            ],
            return_type: Box::new(DynamicType {
                function: first_element_type,
            }),
        },
    );
    map.insert(
        "cardinality",
        Signature {
            parameters: vec![Box::new(ArrayType::new(Box::new(AnyType)))],
            return_type: Box::new(IntType),
        },
    );
}

pub fn array_append(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let mut array = inputs[0].as_array().unwrap();
    let element = &inputs[1];
    array.push(element.to_owned());

    let element_type = inputs[0].data_type().clone();
    Box::new(ArrayValue::new(array, element_type))
}

pub fn array_prepend(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let element = &inputs[0];
    let mut array = inputs[1].as_array().unwrap();
    array.insert(0, element.clone());

    let element_type = inputs[0].data_type().clone();
    Box::new(ArrayValue::new(array, element_type))
}

pub fn array_remove(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let array = inputs[0].as_array().unwrap();
    let element_to_remove = &inputs[1];
    let array_after_remove = array
        .into_iter()
        .filter(|element| !element_to_remove.equals(element))
        .collect();

    let element_type = inputs[0].data_type().clone();
    Box::new(ArrayValue::new(array_after_remove, element_type))
}

pub fn array_cat(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let mut first = inputs[0].as_array().unwrap();
    let mut other = inputs[1].as_array().unwrap();
    let mut result = Vec::with_capacity(first.len() + other.len());
    result.append(&mut first);
    result.append(&mut other);

    let element_type = inputs[0].data_type().clone();
    Box::new(ArrayValue::new(result, element_type))
}

pub fn array_length(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let array = inputs[0].as_array().unwrap();
    let value = array.len() as i64;
    Box::new(IntValue::new(value))
}

pub fn array_shuffle(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let array_type = &inputs[0].data_type();
    let element_type = &array_type
        .as_any()
        .downcast_ref::<ArrayType>()
        .unwrap()
        .base;
    let mut array = inputs[0].as_array().unwrap();
    array.shuffle(&mut rand::rng());
    Box::new(ArrayValue::new(array, element_type.clone()))
}

pub fn array_position(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let array = inputs[0].as_array().unwrap();
    let elemnet = &inputs[1];
    if let Some(index) = array.iter().position(|r| r.equals(elemnet)) {
        return Box::new(IntValue::new((index + 1) as i64));
    }
    Box::new(NullValue)
}

pub fn array_positions(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let array = inputs[0].as_array().unwrap();
    let target = &inputs[1];
    let mut positions: Vec<Box<dyn Value>> = vec![];
    for (index, element) in array.into_iter().enumerate() {
        if element.equals(target) {
            positions.push(Box::new(IntValue::new((index + 1) as i64)));
        }
    }
    Box::new(ArrayValue::new(positions, Box::new(IntType)))
}

pub fn array_dims(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let array_type = inputs[0].data_type();
    Box::new(TextValue::new(array_type.to_string()))
}

pub fn array_replace(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let array_type = inputs[0].data_type();
    let mut array_values = inputs[0].as_array().unwrap();
    let from = &inputs[1];
    let to = &inputs[2];
    for element in &mut array_values {
        if element.equals(from) {
            *element = to.clone();
        }
    }
    Box::new(ArrayValue::new(array_values, array_type))
}

pub fn array_trim(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let mut array = inputs[0].as_array().unwrap();
    let array_type = inputs[0].data_type();
    let array_len = array.len();
    let n = i64::min(array.len().try_into().unwrap(), inputs[1].as_int().unwrap());
    array.truncate(array_len - n as usize);
    Box::new(ArrayValue::new(array, array_type))
}

#[allow(clippy::borrowed_box)]
pub fn array_cardinality(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    fn calculate_array_cardinality(value: &Box<dyn Value>) -> usize {
        if let Some(array) = value.as_array() {
            if array.is_empty() {
                return 0;
            }

            return array.len() * calculate_array_cardinality(&array[0]);
        }
        1
    }

    let cardinality: usize = calculate_array_cardinality(&inputs[0]);
    Box::new(IntValue::new(cardinality as i64))
}
