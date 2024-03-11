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
