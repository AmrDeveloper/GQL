use lazy_static::lazy_static;
use std::collections::HashMap;

type Transformation = fn(String) -> String;

lazy_static! {
    pub static ref TRANSFORMATIONS: HashMap<&'static str, Transformation> = {
        let mut map: HashMap<&'static str, Transformation> = HashMap::new();
        map.insert("lower", lower);
        map.insert("upper", upper);
        map.insert("trim", trim);
        map.insert("length", length);
        map
    };
}

fn lower(input: String) -> String {
    return input.to_lowercase();
}

fn upper(input: String) -> String {
    return input.to_uppercase();
}

fn trim(input: String) -> String {
    return input.trim().to_string();
}

fn length(input: String) -> String {
    return input.len().to_string();
}
