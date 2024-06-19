use gitql_core::signature::Function;
use gitql_core::signature::Signature;
use gitql_core::types::DataType;
use gitql_core::value::Value;
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

pub fn gitql_std_signatures() -> &'static HashMap<&'static str, Signature> {
    static HASHMAP: OnceLock<HashMap<&'static str, Signature>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut map = standard_function_signatures().to_owned();
        map.insert(
            "commit_conventional",
            Signature {
                parameters: vec![DataType::Text],
                return_type: DataType::Text,
            },
        );
        map
    })
}

fn commit_conventional(values: &[Value]) -> Value {
    let text = values[0].as_text();
    let split: Vec<&str> = text.split(':').collect();
    let conventional = if split.len() == 1 { "" } else { split[0] };
    Value::Text(conventional.to_string())
}
