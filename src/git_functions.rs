use gitql_ast::types::text::TextType;
use gitql_core::signature::Function;
use gitql_core::signature::Signature;
use gitql_core::values::base::Value;
use gitql_core::values::text::TextValue;
use gitql_std::function::standard_function_signatures;
use gitql_std::function::standard_functions;
use std::collections::HashMap;
use std::sync::OnceLock;

pub fn gitql_std_functions() -> &'static HashMap<&'static str, Function> {
    static HASHMAP: OnceLock<HashMap<&'static str, Function>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut map = standard_functions().to_owned();
        map.insert("commit_conventional", commit_conventional);
        map
    })
}

pub fn gitql_std_signatures() -> HashMap<&'static str, Signature> {
    let mut map = standard_function_signatures().to_owned();
    map.insert(
        "commit_conventional",
        Signature {
            parameters: vec![Box::new(TextType)],
            return_type: Box::new(TextType),
        },
    );
    map
}

fn commit_conventional(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let text = values[0].as_text().unwrap();
    let split: Vec<&str> = text.split(':').collect();
    let conventional = if split.len() == 1 { "" } else { split[0] };
    Box::new(TextValue {
        value: conventional.to_string(),
    })
}
