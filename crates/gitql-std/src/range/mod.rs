use gitql_ast::types::any::AnyType;
use gitql_ast::types::boolean::BoolType;
use gitql_ast::types::date::DateType;
use gitql_ast::types::datetime::DateTimeType;
use gitql_ast::types::integer::IntType;
use gitql_ast::types::range::RangeType;
use gitql_core::signature::Signature;
use gitql_core::signature::StandardFunction;
use gitql_core::values::boolean::BoolValue;
use gitql_core::values::range::RangeValue;
use gitql_core::values::Value;

use std::collections::HashMap;

#[inline(always)]
pub fn register_std_range_functions(map: &mut HashMap<&'static str, StandardFunction>) {
    map.insert("int4range", int4range);
    map.insert("daterange", daterange);
    map.insert("tsrange", tsrange);
    map.insert("isempty", isempty);
}

#[inline(always)]
pub fn register_std_range_function_signatures(map: &mut HashMap<&'static str, Signature>) {
    map.insert(
        "int4range",
        Signature {
            parameters: vec![Box::new(IntType), Box::new(IntType)],
            return_type: Box::new(RangeType {
                base: Box::new(IntType),
            }),
        },
    );
    map.insert(
        "daterange",
        Signature {
            parameters: vec![Box::new(DateType), Box::new(DateType)],
            return_type: Box::new(RangeType {
                base: Box::new(DateType),
            }),
        },
    );
    map.insert(
        "tsrange",
        Signature {
            parameters: vec![Box::new(DateTimeType), Box::new(DateTimeType)],
            return_type: Box::new(RangeType {
                base: Box::new(DateTimeType),
            }),
        },
    );
    map.insert(
        "isempty",
        Signature {
            parameters: vec![Box::new(RangeType {
                base: Box::new(AnyType),
            })],
            return_type: Box::new(BoolType),
        },
    );
}

pub fn int4range(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    Box::new(RangeValue {
        start: inputs[0].clone(),
        end: inputs[1].clone(),
        base_type: Box::new(IntType),
    })
}

pub fn daterange(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    Box::new(RangeValue {
        start: inputs[0].clone(),
        end: inputs[1].clone(),
        base_type: Box::new(DateType),
    })
}

pub fn tsrange(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    Box::new(RangeValue {
        start: inputs[0].clone(),
        end: inputs[1].clone(),
        base_type: Box::new(DateTimeType),
    })
}

pub fn isempty(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let range = inputs[0].as_range().unwrap();
    Box::new(BoolValue {
        value: range.0.equals(&range.1),
    })
}
