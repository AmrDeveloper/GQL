use std::collections::HashMap;
use std::sync::OnceLock;

use gitql_ast::types::any::AnyType;
use gitql_ast::types::dynamic::DynamicType;
use gitql_core::signature::Signature;
use gitql_core::signature::WindowFunction;
use gitql_core::values::base::Value;
use gitql_core::values::null::NullValue;

use crate::meta_types::first_element_type;

pub fn window_functions() -> &'static HashMap<&'static str, WindowFunction> {
    static HASHMAP: OnceLock<HashMap<&'static str, WindowFunction>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut map: HashMap<&'static str, WindowFunction> = HashMap::new();
        map.insert("first_value", window_first_value);
        map.insert("last_value", window_last_value);
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
        "last_value",
        Signature {
            parameters: vec![Box::new(AnyType)],
            return_type: Box::new(DynamicType {
                function: first_element_type,
            }),
        },
    );
    map
}

pub fn window_first_value(group_values: Vec<Vec<Box<dyn Value>>>) -> Box<dyn Value> {
    if group_values.is_empty() || group_values[0].is_empty() {
        return Box::new(NullValue);
    }

    let first_value = &group_values[0][0];
    first_value.clone()
}

pub fn window_last_value(group_values: Vec<Vec<Box<dyn Value>>>) -> Box<dyn Value> {
    if group_values.is_empty() || group_values[0].is_empty() {
        return Box::new(NullValue);
    }
    group_values[0].last().unwrap().clone()
}
