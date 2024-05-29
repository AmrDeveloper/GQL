use gitql_core::types::DataType;
use gitql_core::value::Value;
use std::cmp::Ordering;
use uuid::Uuid;

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
