use regex::Regex;

/// Return the position of the pattern in the input
/// If the pattern compilation fails, it returns -1
/// If a match is found returns the position of the match's start offset (adjusted by 1)
pub fn regexp_pattern_position(input: &str, pattern: &str) -> i64 {
    if let Ok(regex) = Regex::new(pattern) {
        if let Some(match_result) = regex.find(input) {
            return (match_result.start() + 1) as i64;
        }
    }
    -1
}

/// Return true if a match is found, overwise return false
pub fn regex_is_match(input: &str, pattern: &str) -> bool {
    if let Ok(regex) = Regex::new(pattern) {
        return regex.is_match(input);
    }
    false
}

/// Return the input after replacing pattern with new content
/// Or return the same input if the pattern is invalid
pub fn regex_replace(input: &str, pattern: &str, replacement: &str) -> String {
    if let Ok(regex) = Regex::new(pattern) {
        return regex.replace_all(input, replacement).to_string();
    }
    input.to_string()
}

/// Return substring matching regular expression or empty string if no match found
pub fn regex_substr(input: &str, pattern: &str) -> String {
    if let Ok(regex) = Regex::new(pattern) {
        if let Some(mat) = regex.find(input) {
            return mat.as_str().to_string();
        }
    }
    "".to_string()
}
