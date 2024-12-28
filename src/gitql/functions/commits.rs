use std::collections::HashMap;

use gitql_ast::types::text::TextType;
use gitql_core::signature::Signature;
use gitql_core::signature::StandardFunction;
use gitql_core::values::base::Value;
use gitql_core::values::text::TextValue;

#[inline(always)]
pub(crate) fn register_commits_functions(map: &mut HashMap<&'static str, StandardFunction>) {
    map.insert("commit_conventional", commit_conventional);
}

#[inline(always)]
pub(crate) fn register_commits_function_signatures(map: &mut HashMap<&'static str, Signature>) {
    map.insert(
        "commit_conventional",
        Signature::with_return(Box::new(TextType)).add_parameter(Box::new(TextType)),
    );
}

fn commit_conventional(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let text = values[0].as_text().unwrap();
    let split: Vec<&str> = text.split(':').collect();
    let value = if split.len() == 1 { "" } else { split[0] }.to_string();
    Box::new(TextValue::new(value))
}
