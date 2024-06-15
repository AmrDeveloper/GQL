use std::collections::HashSet;
use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

use gitql_ast::statement::Distinct;
use gitql_core::object::GitQLObject;
use gitql_core::object::Group;
use gitql_core::object::Row;

/// Apply the distinct operator depending on the type of distinct
pub(crate) fn apply_distinct_operator(
    distinct: &Distinct,
    object: &mut GitQLObject,
    hidden_selections: &[String],
) {
    if !object.is_empty() {
        return;
    }

    match distinct {
        Distinct::DistinctAll => apply_distinct_all_operation(object, hidden_selections),
        Distinct::DistinctOn(fields) => apply_distinct_on_operation(object, fields),
        _ => {}
    }
}

/// Apply Distinct all operator that depend on all selected fields in the object
fn apply_distinct_all_operation(object: &mut GitQLObject, hidden_selections: &[String]) {
    let titles: Vec<&String> = object
        .titles
        .iter()
        .filter(|s| !hidden_selections.contains(s))
        .collect();

    let titles_count = titles.len();
    let hidden_selection_count = hidden_selections.len();

    let objects = &object.groups[0].rows;
    let mut new_objects = Group { rows: vec![] };
    let mut values_set: HashSet<u64> = HashSet::new();

    for object in objects {
        // Build row of the selected only values
        let mut row_values: Vec<String> = Vec::with_capacity(titles_count);
        for i in 0..titles.len() {
            if let Some(value) = object.values.get(i + hidden_selection_count) {
                row_values.push(value.to_string());
            }
        }

        // Compute the hash for row of values
        let mut hash = DefaultHasher::new();
        row_values.hash(&mut hash);
        let values_hash = hash.finish();

        // If this hash is unique, insert the row
        if values_set.insert(values_hash) {
            new_objects.rows.push(Row {
                values: object.values.clone(),
            });
        }
    }

    // If number of total rows is changed, update the main group rows
    if objects.len() != new_objects.len() {
        object.groups[0].rows.clear();
        object.groups[0].rows.append(&mut new_objects.rows);
    }
}

/// Apply Distinct on one or more valid fields from the object
fn apply_distinct_on_operation(object: &mut GitQLObject, distinct_fields: &[String]) {
    let objects = &object.groups[0].rows;
    let mut new_objects: Group = Group { rows: vec![] };
    let mut values_set: HashSet<u64> = HashSet::new();
    let titles = &object.titles;

    for object in objects {
        // Build row of the selected only values
        let mut row_values: Vec<String> = Vec::with_capacity(distinct_fields.len());
        for field in distinct_fields {
            if let Some(index) = titles.iter().position(|r| r.eq(field)) {
                row_values.push(object.values.get(index).unwrap().to_string());
            }
        }

        // Compute the hash for row of values
        let mut hash = DefaultHasher::new();
        row_values.hash(&mut hash);
        let values_hash = hash.finish();

        // If this hash is unique, insert the row
        if values_set.insert(values_hash) {
            new_objects.rows.push(Row {
                values: object.values.clone(),
            });
        }
    }

    // If number of total rows is changed, update the main group rows
    if objects.len() != new_objects.len() {
        object.groups[0].rows.clear();
        object.groups[0].rows.append(&mut new_objects.rows);
    }
}
