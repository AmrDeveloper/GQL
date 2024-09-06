use gitql_core::dynamic_types::first_element_type;
use gitql_core::signature::Function;
use gitql_core::signature::Signature;
use gitql_core::types::DataType;
use gitql_core::value::Value;

use std::collections::HashMap;
use std::ops::Rem;

use rand::distributions::Uniform;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

#[inline(always)]
pub fn register_std_number_functions(map: &mut HashMap<&'static str, Function>) {
    map.insert("abs", numeric_abs);
    map.insert("pi", numeric_pi);
    map.insert("floor", numeric_floor);
    map.insert("round", numeric_round);
    map.insert("square", numeric_square);
    map.insert("sin", numeric_sin);
    map.insert("asin", numeric_asin);
    map.insert("cos", numeric_cos);
    map.insert("acos", numeric_acos);
    map.insert("tan", numeric_tan);
    map.insert("atan", numeric_atan);
    map.insert("atn2", numeric_atn2);
    map.insert("sign", numeric_sign);
    map.insert("mod", numeric_mod);
    map.insert("rand", numeric_rand);
}

#[inline(always)]
pub fn register_std_number_function_signatures(map: &mut HashMap<&'static str, Signature>) {
    map.insert(
        "abs",
        Signature {
            parameters: vec![DataType::Variant(vec![DataType::Integer, DataType::Float])],
            return_type: DataType::Dynamic(first_element_type),
        },
    );
    map.insert(
        "pi",
        Signature {
            parameters: vec![],
            return_type: DataType::Float,
        },
    );
    map.insert(
        "floor",
        Signature {
            parameters: vec![DataType::Float],
            return_type: DataType::Integer,
        },
    );
    map.insert(
        "round",
        Signature {
            parameters: vec![
                DataType::Float,
                DataType::Optional(Box::new(DataType::Integer)),
            ],
            return_type: DataType::Float,
        },
    );
    map.insert(
        "square",
        Signature {
            parameters: vec![DataType::Integer],
            return_type: DataType::Integer,
        },
    );
    map.insert(
        "sin",
        Signature {
            parameters: vec![DataType::Float],
            return_type: DataType::Float,
        },
    );
    map.insert(
        "asin",
        Signature {
            parameters: vec![DataType::Float],
            return_type: DataType::Float,
        },
    );
    map.insert(
        "cos",
        Signature {
            parameters: vec![DataType::Float],
            return_type: DataType::Float,
        },
    );
    map.insert(
        "acos",
        Signature {
            parameters: vec![DataType::Float],
            return_type: DataType::Float,
        },
    );
    map.insert(
        "tan",
        Signature {
            parameters: vec![DataType::Float],
            return_type: DataType::Float,
        },
    );
    map.insert(
        "atan",
        Signature {
            parameters: vec![DataType::Float],
            return_type: DataType::Float,
        },
    );
    map.insert(
        "atn2",
        Signature {
            parameters: vec![DataType::Float, DataType::Float],
            return_type: DataType::Float,
        },
    );
    map.insert(
        "sign",
        Signature {
            parameters: vec![DataType::Variant(vec![DataType::Integer, DataType::Float])],
            return_type: DataType::Integer,
        },
    );
    map.insert(
        "mod",
        Signature {
            parameters: vec![DataType::Integer, DataType::Integer],
            return_type: DataType::Integer,
        },
    );
    map.insert(
        "rand",
        Signature {
            parameters: vec![DataType::Optional(Box::new(DataType::Float))],
            return_type: DataType::Float,
        },
    );
}

pub fn numeric_abs(inputs: &[Value]) -> Value {
    let input_type = inputs[0].data_type();
    if input_type.is_float() {
        return Value::Float(inputs[0].as_float().abs());
    }
    Value::Integer(inputs[0].as_int().abs())
}

pub fn numeric_pi(_inputs: &[Value]) -> Value {
    let pi = std::f64::consts::PI;
    Value::Float(pi)
}

pub fn numeric_floor(inputs: &[Value]) -> Value {
    let float_value = inputs[0].as_float();
    Value::Integer(float_value.floor() as i64)
}

pub fn numeric_round(inputs: &[Value]) -> Value {
    let number = inputs[0].as_float();
    let decimal_places = if inputs.len() == 2 {
        inputs[1].as_int()
    } else {
        0
    };
    let multiplier = 10_f64.powi(decimal_places.try_into().unwrap());
    let result = (number * multiplier).round() / multiplier;
    Value::Float(result)
}

pub fn numeric_square(inputs: &[Value]) -> Value {
    let int_value = inputs[0].as_int();
    Value::Integer(int_value * int_value)
}

pub fn numeric_sin(inputs: &[Value]) -> Value {
    let float_value = inputs[0].as_float();
    Value::Float(f64::sin(float_value))
}

pub fn numeric_asin(inputs: &[Value]) -> Value {
    let float_value = inputs[0].as_float();
    Value::Float(f64::asin(float_value))
}

pub fn numeric_cos(inputs: &[Value]) -> Value {
    let float_value = inputs[0].as_float();
    Value::Float(f64::cos(float_value))
}

pub fn numeric_acos(inputs: &[Value]) -> Value {
    let float_value = inputs[0].as_float();
    Value::Float(f64::acos(float_value))
}

pub fn numeric_tan(inputs: &[Value]) -> Value {
    let float_value = inputs[0].as_float();
    Value::Float(f64::tan(float_value))
}

pub fn numeric_atan(inputs: &[Value]) -> Value {
    let float_value = inputs[0].as_float();
    Value::Float(f64::atan(float_value))
}

pub fn numeric_atn2(inputs: &[Value]) -> Value {
    let first = inputs[0].as_float();
    let other = inputs[1].as_float();
    Value::Float(f64::atan2(first, other))
}

pub fn numeric_sign(inputs: &[Value]) -> Value {
    let value = &inputs[0];
    if value.data_type().is_int() {
        let int_value = value.as_int();
        return Value::Integer(int_value.signum());
    }

    let float_value = value.as_float();
    if float_value == 0.0 {
        Value::Integer(0)
    } else if float_value > 0.0 {
        Value::Integer(1)
    } else {
        Value::Integer(-1)
    }
}

pub fn numeric_mod(inputs: &[Value]) -> Value {
    let other = &inputs[1];
    if other.as_int() == 0 {
        return Value::Null;
    }

    let first = &inputs[0];
    Value::Integer(first.as_int().rem(other.as_int()))
}

pub fn numeric_rand(inputs: &[Value]) -> Value {
    let mut rng: StdRng = match inputs.first() {
        Some(s) => SeedableRng::seed_from_u64(s.as_int().try_into().unwrap()),
        None => SeedableRng::from_entropy(),
    };
    Value::Float(rng.sample(Uniform::from(0.0..1.0)))
}
