use std::collections::HashMap;
use std::sync::OnceLock;

use commits::register_commits_function_signatures;
use commits::register_commits_functions;
use diffs::register_diffs_function_signatures;
use diffs::register_diffs_functions;
use gitql_core::signature::Signature;
use gitql_core::signature::StandardFunction;
use gitql_std::standard::standard_function_signatures;
use gitql_std::standard::standard_functions;

mod commits;
mod diffs;

pub fn gitql_std_functions() -> &'static HashMap<&'static str, StandardFunction> {
    static HASHMAP: OnceLock<HashMap<&'static str, StandardFunction>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut map = standard_functions().to_owned();
        register_commits_functions(&mut map);
        register_diffs_functions(&mut map);
        map
    })
}

pub fn gitql_std_signatures() -> HashMap<&'static str, Signature> {
    let mut map = standard_function_signatures().to_owned();
    register_commits_function_signatures(&mut map);
    register_diffs_function_signatures(&mut map);
    map
}
