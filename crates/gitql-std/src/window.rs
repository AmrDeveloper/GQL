use std::collections::HashMap;
use std::sync::OnceLock;

use gitql_ast::types::any::AnyType;
use gitql_ast::types::dynamic::DynamicType;
use gitql_ast::types::integer::IntType;
use gitql_core::signature::Signature;
use gitql_core::signature::WindowFunction;
use gitql_core::values::integer::IntValue;
use gitql_core::values::null::NullValue;
use gitql_core::values::Value;

use crate::meta_types::first_element_type;

pub fn window_functions() -> &'static HashMap<&'static str, WindowFunction> {
    static HASHMAP: OnceLock<HashMap<&'static str, WindowFunction>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut map: HashMap<&'static str, WindowFunction> = HashMap::new();
        map.insert("first_value", window_first_value);
        map.insert("nth_value", window_nth_value);
        map.insert("last_value", window_last_value);
        map.insert("row_number", window_row_number);
        map
    })
}

pub fn window_function_signatures() -> HashMap<&'static str, Signature> {
    let mut map: HashMap<&'static str, Signature> = HashMap::new();
    map.insert(
        "first_value",
        Signature {
            parameters: vec![Box::new(AnyType)],
            return_type: Box::new(DynamicType {
                function: first_element_type,
            }),
        },
    );

    map.insert(
        "nth_value",
        Signature {
            parameters: vec![Box::new(AnyType), Box::new(IntType)],
            return_type: Box::new(DynamicType {
                function: first_element_type,
            }),
        },
    );

    map.insert(
        "last_value",
        Signature {
            parameters: vec![Box::new(AnyType)],
            return_type: Box::new(DynamicType {
                function: first_element_type,
            }),
        },
    );

    map.insert(
        "row_number",
        Signature {
            parameters: vec![],
            return_type: Box::new(IntType),
        },
    );
    map
}

pub fn window_first_value(frame: &[Vec<Box<dyn Value>>]) -> Vec<Box<dyn Value>> {
    let frame_len = frame.len();
    let first_value = &frame[0][0];
    let mut values = Vec::with_capacity(frame_len);
    for _ in 0..frame_len {
        values.push(first_value.clone());
    }
    values
}

pub fn window_nth_value(frame: &[Vec<Box<dyn Value>>]) -> Vec<Box<dyn Value>> {
    let frame_len = frame.len();
    let index = frame[0][1].as_int().unwrap();

    let mut values: Vec<Box<dyn Value>> = Vec::with_capacity(frame_len);
    for _ in 0..frame_len {
        if index < 0 || index as usize >= frame_len {
            values.push(Box::new(NullValue));
        } else {
            values.push(frame[index as usize][0].clone());
        };
    }

    values
}

pub fn window_last_value(frame: &[Vec<Box<dyn Value>>]) -> Vec<Box<dyn Value>> {
    let frame_len = frame.len();
    let last_value = &frame[frame_len - 1][0];
    let mut values = Vec::with_capacity(frame_len);
    for _ in 0..frame_len {
        values.push(last_value.clone());
    }
    values
}

pub fn window_row_number(frame: &[Vec<Box<dyn Value>>]) -> Vec<Box<dyn Value>> {
    let frame_len = frame.len();
    let mut values: Vec<Box<dyn Value>> = Vec::with_capacity(frame_len);
    for i in 0..frame_len {
        let num = i as i64 + 1;
        values.push(Box::new(IntValue { value: num }));
    }
    values
}
