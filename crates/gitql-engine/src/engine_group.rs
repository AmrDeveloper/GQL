use std::collections::hash_map::Entry::Vacant;
use std::collections::HashMap;
use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

use gitql_ast::statement::GroupByStatement;
use gitql_core::combinations_generator::generate_list_of_all_combinations;
use gitql_core::environment::Environment;
use gitql_core::object::GitQLObject;
use gitql_core::object::Group;

use crate::engine_evaluator::evaluate_expression;

pub(crate) fn execute_group_by_statement(
    env: &mut Environment,
    statement: &GroupByStatement,
    gitql_object: &mut GitQLObject,
) -> Result<(), String> {
    if gitql_object.is_empty() {
        return Ok(());
    }

    let main_group = gitql_object.groups.remove(0);
    if main_group.is_empty() {
        return Ok(());
    }

    // Mapping each unique value to it group index
    let mut groups_map: HashMap<u64, usize> = HashMap::new();

    // Track current group index
    let mut next_group_index = 0;
    let values_count = statement.values.len();

    let is_rollup_enabled = statement.has_with_rollup;
    let indexes_combinations = if is_rollup_enabled {
        generate_list_of_all_combinations(values_count)
    } else {
        vec![(0..values_count).collect()]
    };

    // For each row should check the group by values combinations to build multi groups
    for row in main_group.rows.iter() {
        // Create all combination of values for each row
        for indexes in indexes_combinations.iter() {
            let mut row_values: Vec<String> = Vec::with_capacity(indexes.len());
            for index in indexes {
                let value = evaluate_expression(
                    env,
                    &statement.values[*index],
                    &gitql_object.titles,
                    &row.values,
                )?;
                row_values.push(value.literal());
            }

            // Compute the hash for row of values
            let mut hasher = DefaultHasher::new();
            row_values.hash(&mut hasher);
            let values_hash = hasher.finish();

            // Push a new group for this unique value and update the next index
            if let Vacant(e) = groups_map.entry(values_hash) {
                e.insert(next_group_index);
                next_group_index += 1;
                gitql_object.groups.push(Group {
                    rows: vec![row.clone()],
                });
                continue;
            }

            // If there is an existing group for this value, append current object to it
            let index = *groups_map.get(&values_hash).unwrap();
            let target_group = &mut gitql_object.groups[index];
            target_group.rows.push(row.clone());
        }
    }

    // If the group by elements is one and rollup is enabled
    // For example: SELECT ... FROM <TABLE> GROUP BY X WITH ROLLUP
    // Should append the the main group at the end
    if is_rollup_enabled && indexes_combinations.len() == 1 && indexes_combinations[0].len() == 1 {
        gitql_object.groups.push(main_group);
    }

    Ok(())
}
