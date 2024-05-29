use gitql_core::value::Value;
use rand::distributions::Uniform;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use std::ops::Rem;

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
