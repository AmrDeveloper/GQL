use std::collections::HashMap;

use gitql_ast::statement::Join;
use gitql_ast::statement::JoinKind;
use gitql_ast::statement::TableSelection;
use gitql_core::environment::Environment;
use gitql_core::object::Row;
use gitql_core::value::Value;

use crate::engine_evaluator::evaluate_expression;

#[inline(always)]
pub(crate) fn apply_join_operation(
    env: &mut Environment,
    all_rows: &mut Vec<Row>,
    joins: &Vec<Join>,
    tables_selections: &Vec<TableSelection>,
    selected_rows_per_table: &mut HashMap<String, Vec<Row>>,
    hidden_selection_per_table: &HashMap<String, usize>,
    titles: &[String],
) -> Result<(), String> {
    // If no join, just merge them, can be optimized to append only the first value in the map
    if joins.is_empty() {
        for table_selection in tables_selections {
            let table_rows = selected_rows_per_table
                .get_mut(&table_selection.table_name)
                .unwrap();
            all_rows.append(table_rows);
        }
        return Ok(());
    }

    // Apply join operator depend on the join type
    for join in joins {
        let left_rows = selected_rows_per_table.get(&join.left).unwrap();
        let left_hidden_count = if let Some(count) = hidden_selection_per_table.get(&join.left) {
            *count
        } else {
            0
        };

        let right_rows = selected_rows_per_table.get(&join.right).unwrap();
        let right_hidden_count = if let Some(count) = hidden_selection_per_table.get(&join.right) {
            *count
        } else {
            0
        };

        // Perform nested loops straight forward join algorithm
        for outer in left_rows {
            for inner in right_rows {
                let row_len = outer.values.len() + inner.values.len();
                let mut joined_row: Vec<Value> = Vec::with_capacity(row_len);
                joined_row.append(&mut outer.values.clone());

                let inner_rows = inner.values.clone();
                let inner_hidden_values = &inner_rows[0..left_hidden_count];
                joined_row.splice(
                    right_hidden_count..right_hidden_count,
                    inner_hidden_values.to_vec(),
                );

                let inner_other_values = &inner_rows[left_hidden_count..];
                joined_row.extend_from_slice(inner_other_values);

                // If join has predicate, insert the joined row only if the predicate value is true
                if let Some(predicate) = &join.predicate {
                    let predicate_value = evaluate_expression(env, predicate, titles, &joined_row)?;
                    if predicate_value.as_bool() {
                        all_rows.push(Row { values: joined_row });
                        continue;
                    }

                    // For LEFT and RIGHT Join only if the predicate is false we need to create new joined row
                    // The new joined row will have nulls as LEFT table row values if the join type is `RIGHT OUTER` or
                    // Nulls as RGIHT table row values if the join type is `LEFT OUTER`
                    match join.kind {
                        JoinKind::Left => {
                            let mut left_joined_row: Vec<Value> = Vec::with_capacity(row_len);
                            // Push the LEFT values row
                            left_joined_row.append(&mut outer.values.clone());
                            // Push (N * NULL) values as RIGHT values row
                            for _ in 0..inner.values.len() {
                                left_joined_row.push(Value::Null);
                            }
                        }
                        JoinKind::Right => {
                            let mut right_joined_row: Vec<Value> = Vec::with_capacity(row_len);
                            // Push (N * NULL) values as LEFT values row
                            for _ in 0..outer.values.len() {
                                right_joined_row.push(Value::Null);
                            }
                            // Push the RIGHT values row
                            right_joined_row.append(&mut inner.values.clone());
                        }
                        _ => {}
                    }
                    continue;
                }

                // If the condition has no predicate, just insert it
                all_rows.push(Row { values: joined_row });
            }
        }
    }

    Ok(())
}
