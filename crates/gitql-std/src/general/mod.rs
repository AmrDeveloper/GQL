use gitql_ast::types::any::AnyType;
use gitql_ast::types::boolean::BoolType;
use gitql_ast::types::dynamic::DynamicType;
use gitql_ast::types::text::TextType;
use gitql_ast::types::varargs::VarargsType;
use gitql_core::dynamic_types::first_element_type;
use gitql_core::dynamic_types::second_element_type;
use gitql_core::signature::Function;
use gitql_core::signature::Signature;
use gitql_core::values::base::Value;
use gitql_core::values::boolean::BoolValue;
use gitql_core::values::text::TextValue;

use std::cmp::Ordering;
use std::collections::HashMap;

use uuid::Uuid;

#[inline(always)]
pub fn register_std_general_functions(map: &mut HashMap<&'static str, Function>) {
    map.insert("isnull", general_is_null);
    map.insert("isnumeric", general_is_numeric);
    map.insert("typeof", general_type_of);
    map.insert("greatest", general_greatest);
    map.insert("least", general_least);
    map.insert("uuid", general_uuid);
    map.insert("if", general_if);
    map.insert("ifnull", general_ifnull);
}

#[inline(always)]
pub fn register_std_general_function_signatures(map: &mut HashMap<&'static str, Signature>) {
    map.insert(
        "isnull",
        Signature {
            parameters: vec![Box::new(AnyType)],
            return_type: Box::new(BoolType),
        },
    );
    map.insert(
        "isnumeric",
        Signature {
            parameters: vec![Box::new(AnyType)],
            return_type: Box::new(BoolType),
        },
    );
    map.insert(
        "typeof",
        Signature {
            parameters: vec![Box::new(AnyType)],
            return_type: Box::new(TextType),
        },
    );
    map.insert(
        "greatest",
        Signature {
            parameters: vec![
                Box::new(AnyType),
                Box::new(AnyType),
                Box::new(VarargsType {
                    base: Box::new(AnyType),
                }),
            ],
            return_type: Box::new(AnyType),
        },
    );
    map.insert(
        "least",
        Signature {
            parameters: vec![
                Box::new(AnyType),
                Box::new(AnyType),
                Box::new(VarargsType {
                    base: Box::new(AnyType),
                }),
            ],
            return_type: Box::new(AnyType),
        },
    );
    map.insert(
        "uuid",
        Signature {
            parameters: vec![],
            return_type: Box::new(TextType),
        },
    );
    map.insert(
        "if",
        Signature {
            parameters: vec![
                Box::new(BoolType),
                Box::new(AnyType),
                Box::new(DynamicType {
                    function: second_element_type,
                }),
            ],
            return_type: Box::new(DynamicType {
                function: second_element_type,
            }),
        },
    );
    map.insert(
        "ifnull",
        Signature {
            parameters: vec![
                Box::new(AnyType),
                Box::new(DynamicType {
                    function: first_element_type,
                }),
            ],
            return_type: Box::new(DynamicType {
                function: first_element_type,
            }),
        },
    );
}

pub fn general_is_null(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let is_null = inputs[0].data_type().is_null();
    Box::new(BoolValue { value: is_null })
}

pub fn general_is_numeric(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let input_type = inputs[0].data_type();
    Box::new(BoolValue {
        value: input_type.is_number(),
    })
}

pub fn general_type_of(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let input_type = inputs[0].data_type();
    Box::new(TextValue {
        value: input_type.to_string(),
    })
}

pub fn general_greatest(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let mut max = &inputs[0];

    for value in inputs.iter().skip(1) {
        if max.compare(value) == Some(Ordering::Greater) {
            max = value;
        }
    }

    max.to_owned()
}

pub fn general_least(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let mut least = &inputs[0];

    for value in inputs.iter().skip(1) {
        if least.compare(value) == Some(Ordering::Less) {
            least = value;
        }
    }

    least.to_owned()
}

pub fn general_uuid(_inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let uuid = Uuid::new_v4();
    Box::new(TextValue {
        value: uuid.to_string(),
    })
}

pub fn general_if(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let condition = inputs[0].as_bool().unwrap();
    if condition {
        inputs[1].clone()
    } else {
        inputs[2].clone()
    }
}

pub fn general_ifnull(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    if inputs[0].data_type().is_null() {
        return inputs[1].clone();
    }
    inputs[0].clone()
}
