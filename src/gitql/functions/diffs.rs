use std::collections::HashMap;
use std::collections::HashSet;

use gitql_ast::types::integer::IntType;
use gitql_core::signature::Signature;
use gitql_core::signature::StandardFunction;
use gitql_core::values::base::Value;
use gitql_core::values::integer::IntValue;

use crate::gitql::types::diff_changes::DiffChangesType;
use crate::gitql::values::diff_changes::DiffChangesValue;

#[inline(always)]
pub(crate) fn register_diffs_functions(map: &mut HashMap<&'static str, StandardFunction>) {
    map.insert("diff_changes_files_count", diff_changes_files_count);
}

#[inline(always)]
pub(crate) fn register_diffs_function_signatures(map: &mut HashMap<&'static str, Signature>) {
    map.insert(
        "diff_changes_files_count",
        Signature {
            parameters: vec![Box::new(DiffChangesType)],
            return_type: Box::new(IntType),
        },
    );
}

fn diff_changes_files_count(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    if let Some(changes) = values[0].as_any().downcast_ref::<DiffChangesValue>() {
        let mut unique_files: HashSet<&String> = HashSet::new();
        for change in changes.changes.iter() {
            unique_files.insert(&change.location);
        }
        let value = unique_files.len() as i64;
        return Box::new(IntValue { value });
    }
    Box::new(IntValue { value: 0 })
}
