use std::collections::HashMap;

use gitql_ast::statement::Join;
use gitql_ast::statement::TableSelection;
use gitql_core::object::Row;
use gitql_core::value::Value;

#[inline(always)]
pub(crate) fn apply_join_operation(
    all_rows: &mut Vec<Row>,
    joins: &Vec<Join>,
    tables_selections: &Vec<TableSelection>,
    selected_rows_per_table: &mut HashMap<String, Vec<Row>>,
    hidden_selection_per_table: &HashMap<String, usize>,
) {
    // If no join, just merge them, can be optimized to append only the first value in the map
    if joins.is_empty() {
        for table_selection in tables_selections {
            let table_rows = selected_rows_per_table
                .get_mut(&table_selection.table_name)
                .unwrap();
            all_rows.append(table_rows);
        }
        return;
    }

    // Apply join operator depend on the join type
    for join in joins {
        let right_rows = selected_rows_per_table.get(&join.right).unwrap();
        let right_hidden_count = if let Some(count) = hidden_selection_per_table.get(&join.right) {
            *count
        } else {
            0
        };
        let left_rows = selected_rows_per_table.get(&join.left).unwrap();
        let left_hidden_count = if let Some(count) = hidden_selection_per_table.get(&join.left) {
            *count
        } else {
            0
        };

        for right in right_rows {
            for left in left_rows {
                let mut new_row_values: Vec<Value> = vec![];
                new_row_values.append(&mut right.values.clone());

                let left_rows = left.values.clone();
                let left_hidden_values = &left_rows[0..left_hidden_count];
                new_row_values.splice(
                    right_hidden_count..right_hidden_count,
                    left_hidden_values.to_vec(),
                );

                let left_other_values = &left_rows[left_hidden_count..];
                new_row_values.extend_from_slice(left_other_values);

                all_rows.push(Row {
                    values: new_row_values,
                });
            }
        }
    }
}
