use gitql_ast::types::boolean::BoolType;
use gitql_ast::types::integer::IntType;
use gitql_ast::types::text::TextType;
use gitql_core::signature::Signature;
use gitql_core::signature::StandardFunction;
use gitql_core::values::boolean::BoolValue;
use gitql_core::values::integer::IntValue;
use gitql_core::values::text::TextValue;
use gitql_core::values::Value;

use std::collections::HashMap;

use regex::Regex;

#[inline(always)]
pub fn register_std_regex_functions(map: &mut HashMap<&'static str, StandardFunction>) {
    map.insert("regexp_instr", regexp_instr);
    map.insert("regexp_like", regexp_like);
    map.insert("regexp_replace", regexp_replace);
    map.insert("regexp_substr", regexp_substr);
}

#[inline(always)]
pub fn register_std_regex_function_signatures(map: &mut HashMap<&'static str, Signature>) {
    map.insert(
        "regexp_instr",
        Signature {
            parameters: vec![Box::new(TextType), Box::new(TextType)],
            return_type: Box::new(IntType),
        },
    );
    map.insert(
        "regexp_like",
        Signature {
            parameters: vec![Box::new(TextType), Box::new(TextType)],
            return_type: Box::new(BoolType),
        },
    );
    map.insert(
        "regexp_replace",
        Signature {
            parameters: vec![Box::new(TextType), Box::new(TextType), Box::new(TextType)],
            return_type: Box::new(TextType),
        },
    );
    map.insert(
        "regexp_substr",
        Signature {
            parameters: vec![Box::new(TextType), Box::new(TextType)],
            return_type: Box::new(TextType),
        },
    );
}

/// Return the position of the pattern in the input
/// If the pattern compilation fails, it returns -1
/// If a match is found returns the position of the match's start offset (adjusted by 1)
pub fn regexp_instr(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let input = inputs[0].as_text().unwrap();
    let pattern = inputs[1].as_text().unwrap();
    if let Ok(regex) = Regex::new(&pattern) {
        if let Some(match_result) = regex.find(&input) {
            let value = (match_result.start() + 1) as i64;
            return Box::new(IntValue { value });
        }
    }
    Box::new(IntValue { value: -1 })
}

/// Return true if a match is found, overwise return false
pub fn regexp_like(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let input = inputs[0].as_text().unwrap();
    let pattern = inputs[1].as_text().unwrap();
    if let Ok(regex) = Regex::new(&pattern) {
        return Box::new(BoolValue {
            value: regex.is_match(&input),
        });
    }
    Box::new(BoolValue { value: false })
}

/// Return the input after replacing pattern with new content
/// Or return the same input if the pattern is invalid
pub fn regexp_replace(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let input = inputs[0].as_text().unwrap();
    let pattern = inputs[1].as_text().unwrap();
    let replacement = inputs[2].as_text().unwrap();
    if let Ok(regex) = Regex::new(&pattern) {
        let value = regex.replace_all(&input, replacement).to_string();
        return Box::new(TextValue { value });
    }
    Box::new(TextValue { value: input })
}

/// Return substring matching regular expression or empty string if no match found
pub fn regexp_substr(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let input = inputs[0].as_text().unwrap();
    let pattern = inputs[1].as_text().unwrap();
    if let Ok(regex) = Regex::new(&pattern) {
        if let Some(mat) = regex.find(&input) {
            return Box::new(TextValue {
                value: mat.as_str().to_string(),
            });
        }
    }
    Box::new(TextValue {
        value: "".to_string(),
    })
}
