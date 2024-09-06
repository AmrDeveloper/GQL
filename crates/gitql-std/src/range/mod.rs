use gitql_core::signature::Function;
use gitql_core::signature::Signature;
use gitql_core::types::DataType;
use gitql_core::value::Value;

use std::collections::HashMap;

#[inline(always)]
pub fn register_std_range_functions(map: &mut HashMap<&'static str, Function>) {
    map.insert("int4range", int4range);
}

#[inline(always)]
pub fn register_std_range_function_signatures(map: &mut HashMap<&'static str, Signature>) {
    map.insert(
        "int4range",
        Signature {
            parameters: vec![DataType::Integer, DataType::Integer],
            return_type: DataType::Range(Box::new(DataType::Integer)),
        },
    );
}

pub fn int4range(inputs: &[Value]) -> Value {
    Value::Range(
        DataType::Integer,
        Box::new(inputs[0].clone()),
        Box::new(inputs[1].clone()),
    )
}
