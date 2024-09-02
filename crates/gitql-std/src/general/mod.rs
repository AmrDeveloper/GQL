use gitql_core::signature::Function;
use gitql_core::signature::Signature;
use gitql_core::types::DataType;
use gitql_core::value::Value;

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
}

#[inline(always)]
pub fn register_std_general_function_signatures(map: &mut HashMap<&'static str, Signature>) {
    map.insert(
        "isnull",
        Signature {
            parameters: vec![DataType::Any],
            return_type: DataType::Boolean,
        },
    );
    map.insert(
        "isnumeric",
        Signature {
            parameters: vec![DataType::Any],
            return_type: DataType::Boolean,
        },
    );
    map.insert(
        "typeof",
        Signature {
            parameters: vec![DataType::Any],
            return_type: DataType::Text,
        },
    );
    map.insert(
        "greatest",
        Signature {
            parameters: vec![
                DataType::Any,
                DataType::Any,
                DataType::Varargs(Box::new(DataType::Any)),
            ],
            return_type: DataType::Any,
        },
    );
    map.insert(
        "least",
        Signature {
            parameters: vec![
                DataType::Any,
                DataType::Any,
                DataType::Varargs(Box::new(DataType::Any)),
            ],
            return_type: DataType::Any,
        },
    );
    map.insert(
        "uuid",
        Signature {
            parameters: vec![],
            return_type: DataType::Text,
        },
    );
}

pub fn general_is_null(inputs: &[Value]) -> Value {
    Value::Boolean(inputs[0].data_type() == DataType::Null)
}

pub fn general_is_numeric(inputs: &[Value]) -> Value {
    let input_type = inputs[0].data_type();
    Value::Boolean(input_type.is_number())
}

pub fn general_type_of(inputs: &[Value]) -> Value {
    let input_type = inputs[0].data_type();
    Value::Text(input_type.to_string())
}

pub fn general_greatest(inputs: &[Value]) -> Value {
    let mut max = &inputs[0];

    for value in inputs.iter().skip(1) {
        if max.compare(value) == Ordering::Greater {
            max = value;
        }
    }

    max.to_owned()
}

pub fn general_least(inputs: &[Value]) -> Value {
    let mut least = &inputs[0];

    for value in inputs.iter().skip(1) {
        if least.compare(value) == Ordering::Less {
            least = value;
        }
    }

    least.to_owned()
}

pub fn general_uuid(_inputs: &[Value]) -> Value {
    let uuid = Uuid::new_v4();
    Value::Text(uuid.to_string())
}
