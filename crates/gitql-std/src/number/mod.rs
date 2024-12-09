use std::collections::HashMap;
use std::ops::Rem;

use gitql_ast::types::dynamic::DynamicType;
use gitql_ast::types::float::FloatType;
use gitql_ast::types::integer::IntType;
use gitql_ast::types::optional::OptionType;
use gitql_ast::types::variant::VariantType;
use gitql_core::signature::Signature;
use gitql_core::signature::StandardFunction;
use gitql_core::values::base::Value;
use gitql_core::values::float::FloatValue;
use gitql_core::values::integer::IntValue;
use gitql_core::values::null::NullValue;

use crate::meta_types::first_element_type;

use rand::distributions::Uniform;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

#[inline(always)]
pub fn register_std_number_functions(map: &mut HashMap<&'static str, StandardFunction>) {
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
            parameters: vec![Box::new(VariantType {
                variants: vec![Box::new(IntType), Box::new(FloatType)],
            })],
            return_type: Box::new(DynamicType {
                function: first_element_type,
            }),
        },
    );
    map.insert(
        "pi",
        Signature {
            parameters: vec![],
            return_type: Box::new(FloatType),
        },
    );
    map.insert(
        "floor",
        Signature {
            parameters: vec![Box::new(FloatType)],
            return_type: Box::new(IntType),
        },
    );
    map.insert(
        "round",
        Signature {
            parameters: vec![
                Box::new(FloatType),
                Box::new(OptionType {
                    base: Some(Box::new(IntType)),
                }),
            ],
            return_type: Box::new(FloatType),
        },
    );
    map.insert(
        "square",
        Signature {
            parameters: vec![Box::new(IntType)],
            return_type: Box::new(IntType),
        },
    );
    map.insert(
        "sin",
        Signature {
            parameters: vec![Box::new(FloatType)],
            return_type: Box::new(FloatType),
        },
    );
    map.insert(
        "asin",
        Signature {
            parameters: vec![Box::new(FloatType)],
            return_type: Box::new(FloatType),
        },
    );
    map.insert(
        "cos",
        Signature {
            parameters: vec![Box::new(FloatType)],
            return_type: Box::new(FloatType),
        },
    );
    map.insert(
        "acos",
        Signature {
            parameters: vec![Box::new(FloatType)],
            return_type: Box::new(FloatType),
        },
    );
    map.insert(
        "tan",
        Signature {
            parameters: vec![Box::new(FloatType)],
            return_type: Box::new(FloatType),
        },
    );
    map.insert(
        "atan",
        Signature {
            parameters: vec![Box::new(FloatType)],
            return_type: Box::new(FloatType),
        },
    );
    map.insert(
        "atn2",
        Signature {
            parameters: vec![Box::new(FloatType), Box::new(FloatType)],
            return_type: Box::new(FloatType),
        },
    );
    map.insert(
        "sign",
        Signature {
            parameters: vec![Box::new(VariantType {
                variants: vec![Box::new(IntType), Box::new(FloatType)],
            })],
            return_type: Box::new(IntType),
        },
    );
    map.insert(
        "mod",
        Signature {
            parameters: vec![Box::new(IntType), Box::new(IntType)],
            return_type: Box::new(IntType),
        },
    );
    map.insert(
        "rand",
        Signature {
            parameters: vec![Box::new(OptionType {
                base: Some(Box::new(FloatType)),
            })],
            return_type: Box::new(FloatType),
        },
    );
}

pub fn numeric_abs(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let input_type = inputs[0].data_type();
    if input_type.is_float() {
        return Box::new(FloatValue {
            value: inputs[0].as_float().unwrap().abs(),
        });
    }

    Box::new(IntValue {
        value: inputs[0].as_int().unwrap().abs(),
    })
}

pub fn numeric_pi(_inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let pi = std::f64::consts::PI;
    Box::new(FloatValue { value: pi })
}

pub fn numeric_floor(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let float_value = inputs[0].as_float().unwrap();
    Box::new(IntValue {
        value: float_value.floor() as i64,
    })
}

pub fn numeric_round(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let number = inputs[0].as_float().unwrap();
    let decimal_places = if inputs.len() == 2 {
        inputs[1].as_int().unwrap()
    } else {
        0
    };
    let multiplier = 10_f64.powi(decimal_places.try_into().unwrap());
    let result = (number * multiplier).round() / multiplier;
    Box::new(FloatValue { value: result })
}

pub fn numeric_square(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let int_value = inputs[0].as_int().unwrap();
    Box::new(IntValue {
        value: int_value * int_value,
    })
}

pub fn numeric_sin(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let float_value = inputs[0].as_float().unwrap();
    Box::new(FloatValue {
        value: f64::sin(float_value),
    })
}

pub fn numeric_asin(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let float_value = inputs[0].as_float().unwrap();
    Box::new(FloatValue {
        value: f64::asin(float_value),
    })
}

pub fn numeric_cos(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let float_value = inputs[0].as_float().unwrap();
    Box::new(FloatValue {
        value: f64::cos(float_value),
    })
}

pub fn numeric_acos(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let float_value = inputs[0].as_float().unwrap();
    Box::new(FloatValue {
        value: f64::acos(float_value),
    })
}

pub fn numeric_tan(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let float_value = inputs[0].as_float().unwrap();
    Box::new(FloatValue {
        value: f64::tan(float_value),
    })
}

pub fn numeric_atan(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let float_value = inputs[0].as_float().unwrap();
    Box::new(FloatValue {
        value: f64::atan(float_value),
    })
}

pub fn numeric_atn2(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let first = inputs[0].as_float().unwrap();
    let other = inputs[1].as_float().unwrap();
    Box::new(FloatValue {
        value: f64::atan2(first, other),
    })
}

pub fn numeric_sign(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let value = &inputs[0];
    if value.data_type().is_int() {
        let value = value.as_int().unwrap().signum();
        return Box::new(IntValue { value });
    }

    let float_value = value.as_float().unwrap();
    let value = if float_value == 0.0 {
        0
    } else if float_value > 0.0 {
        1
    } else {
        -1
    };
    Box::new(IntValue { value })
}

pub fn numeric_mod(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let other = inputs[1].as_int().unwrap();
    if other == 0 {
        return Box::new(NullValue);
    }

    let first = inputs[0].as_int().unwrap();
    let value = first.rem(other);
    Box::new(IntValue { value })
}

pub fn numeric_rand(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let mut rng: StdRng = match inputs.first() {
        Some(s) => SeedableRng::seed_from_u64(s.as_int().unwrap().try_into().unwrap()),
        None => SeedableRng::from_entropy(),
    };

    Box::new(FloatValue {
        value: rng.sample(Uniform::from(0.0..1.0)),
    })
}
