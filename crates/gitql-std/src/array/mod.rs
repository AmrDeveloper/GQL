use gitql_core::types::DataType;
use gitql_core::value::Value;
use rand::seq::SliceRandom;

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
