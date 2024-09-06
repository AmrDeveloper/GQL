use crate::array::*;
use crate::datetime::*;
use crate::general::*;
use crate::number::*;
use crate::range::*;
use crate::regex::*;
use crate::text::*;

use gitql_core::signature::Function;
use gitql_core::signature::Signature;
use std::collections::HashMap;
use std::sync::OnceLock;

pub fn standard_functions() -> &'static HashMap<&'static str, Function> {
    static HASHMAP: OnceLock<HashMap<&'static str, Function>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut map: HashMap<&'static str, Function> = HashMap::new();
        register_std_text_functions(&mut map);
        register_std_datetime_functions(&mut map);
        register_std_number_functions(&mut map);
        register_std_general_functions(&mut map);
        register_std_regex_functions(&mut map);
        register_std_array_functions(&mut map);
        register_std_range_functions(&mut map);
        map
    })
}

pub fn standard_function_signatures() -> &'static HashMap<&'static str, Signature> {
    static HASHMAP: OnceLock<HashMap<&'static str, Signature>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut map: HashMap<&'static str, Signature> = HashMap::new();
        register_std_text_function_signatures(&mut map);
        register_std_datetime_function_signatures(&mut map);
        register_std_number_function_signatures(&mut map);
        register_std_general_function_signatures(&mut map);
        register_std_regex_function_signatures(&mut map);
        register_std_array_function_signatures(&mut map);
        register_std_range_function_signatures(&mut map);
        map
    })
}
