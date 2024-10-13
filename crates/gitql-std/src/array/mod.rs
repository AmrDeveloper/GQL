use gitql_ast::types::any::AnyType;
use gitql_ast::types::array::ArrayType;
use gitql_ast::types::dynamic::DynamicType;
use gitql_ast::types::integer::IntType;
use gitql_ast::types::text::TextType;
use gitql_core::dynamic_types::array_element_type;
use gitql_core::dynamic_types::array_of_type;
use gitql_core::dynamic_types::first_element_type;
use gitql_core::dynamic_types::second_element_type;
use gitql_core::signature::Function;
use gitql_core::signature::Signature;
use gitql_core::values::array::ArrayValue;
use gitql_core::values::base::Value;
use gitql_core::values::integer::IntValue;
use gitql_core::values::null::NullValue;
use gitql_core::values::text::TextValue;

use rand::seq::SliceRandom;

use std::collections::HashMap;

#[inline(always)]
pub fn register_std_array_functions(map: &mut HashMap<&'static str, Function>) {
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
}

#[inline(always)]
pub fn register_std_array_function_signatures(map: &mut HashMap<&'static str, Signature>) {
    map.insert(
        "array_append",
        Signature {
            parameters: vec![
                Box::new(ArrayType {
                    base: Box::new(AnyType),
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
        "array_prepend",
        Signature {
            parameters: vec![
                Box::new(AnyType),
                Box::new(DynamicType {
                    function: |elements| array_of_type(first_element_type(elements)),
                }),
            ],
            return_type: Box::new(DynamicType {
                function: second_element_type,
            }),
        },
    );
    map.insert(
        "array_remove",
        Signature {
            parameters: vec![
                Box::new(ArrayType {
                    base: Box::new(AnyType),
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
        "array_cat",
        Signature {
            parameters: vec![
                Box::new(ArrayType {
                    base: Box::new(AnyType),
                }),
                Box::new(DynamicType {
                    function: first_element_type,
                }),
            ],
            return_type: Box::new(DynamicType {
                function: first_element_type,
            }),
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
}

pub fn array_append(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let mut array = inputs[0].as_array().unwrap();
    let element = &inputs[1];
    array.push(element.to_owned());
    Box::new(ArrayValue {
        values: array,
        base_type: inputs[0].data_type().clone(),
    })
}

pub fn array_prepend(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let element = &inputs[0];
    let mut array = inputs[1].as_array().unwrap();
    array.insert(0, element.clone());
    Box::new(ArrayValue {
        values: array,
        base_type: inputs[1].data_type().clone(),
    })
}

pub fn array_remove(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let array = inputs[0].as_array().unwrap();
    let element_to_remove = &inputs[1];
    let array_after_remove = array
        .into_iter()
        .filter(|element| !element_to_remove.equals(element))
        .collect();
    Box::new(ArrayValue {
        values: array_after_remove,
        base_type: inputs[0].data_type().clone(),
    })
}

pub fn array_cat(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let mut first = inputs[0].as_array().unwrap();
    let mut other = inputs[1].as_array().unwrap();
    let mut result = Vec::with_capacity(first.len() + other.len());
    result.append(&mut first);
    result.append(&mut other);
    Box::new(ArrayValue {
        values: result,
        base_type: inputs[0].data_type().clone(),
    })
}

pub fn array_length(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let array = inputs[0].as_array().unwrap();
    let value = array.len() as i64;
    Box::new(IntValue { value })
}

pub fn array_shuffle(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let array_type = &inputs[0].data_type();
    let element_type = &array_type
        .as_any()
        .downcast_ref::<ArrayType>()
        .unwrap()
        .base;

    let mut array = inputs[0].as_array().unwrap();
    array.shuffle(&mut rand::thread_rng());
    Box::new(ArrayValue {
        values: array,
        base_type: element_type.clone(),
    })
}

pub fn array_position(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let array = inputs[0].as_array().unwrap();
    let elemnet = &inputs[1];
    if let Some(index) = array.iter().position(|r| r.equals(elemnet)) {
        return Box::new(IntValue {
            value: (index + 1) as i64,
        });
    }

    Box::new(NullValue)
}

pub fn array_positions(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let array = inputs[0].as_array().unwrap();
    let target = &inputs[1];
    let mut positions: Vec<Box<dyn Value>> = vec![];
    for (index, element) in array.into_iter().enumerate() {
        if element.equals(target) {
            positions.push(Box::new(IntValue {
                value: (index + 1) as i64,
            }));
        }
    }
    Box::new(ArrayValue {
        values: positions,
        base_type: Box::new(IntType),
    })
}

pub fn array_dims(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let array_type = inputs[0].data_type();
    Box::new(TextValue {
        value: array_type.to_string(),
    })
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

    Box::new(ArrayValue {
        values: array_values,
        base_type: array_type,
    })
}

pub fn array_trim(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let mut array = inputs[0].as_array().unwrap();
    let array_type = inputs[0].data_type();
    let array_len = array.len();
    let n = i64::min(array.len().try_into().unwrap(), inputs[1].as_int().unwrap());
    array.truncate(array_len - n as usize);
    Box::new(ArrayValue {
        values: array,
        base_type: array_type,
    })
}
