use gitql_core::value::Value;
use regex::Regex;

/// Return the position of the pattern in the input
/// If the pattern compilation fails, it returns -1
/// If a match is found returns the position of the match's start offset (adjusted by 1)
pub fn regexp_instr(inputs: &[Value]) -> Value {
    let input = inputs[0].as_text();
    let pattern = inputs[1].as_text();
    if let Ok(regex) = Regex::new(&pattern) {
        if let Some(match_result) = regex.find(&input) {
            return Value::Integer((match_result.start() + 1) as i64);
        }
    }
    Value::Integer(-1)
}

/// Return true if a match is found, overwise return false
pub fn regexp_like(inputs: &[Value]) -> Value {
    let input = inputs[0].as_text();
    let pattern = inputs[1].as_text();
    if let Ok(regex) = Regex::new(&pattern) {
        return Value::Boolean(regex.is_match(&input));
    }
    Value::Boolean(false)
}

/// Return the input after replacing pattern with new content
/// Or return the same input if the pattern is invalid
pub fn regexp_replace(inputs: &[Value]) -> Value {
    let input = inputs[0].as_text();
    let pattern = inputs[1].as_text();
    let replacement = inputs[2].as_text();
    if let Ok(regex) = Regex::new(&pattern) {
        return Value::Text(regex.replace_all(&input, replacement).to_string());
    }
    Value::Text(input)
}

/// Return substring matching regular expression or empty string if no match found
pub fn regexp_substr(inputs: &[Value]) -> Value {
    let input = inputs[0].as_text();
    let pattern = inputs[1].as_text();
    if let Ok(regex) = Regex::new(&pattern) {
        if let Some(mat) = regex.find(&input) {
            return Value::Text(mat.as_str().to_string());
        }
    }
    Value::Text("".to_string())
}
