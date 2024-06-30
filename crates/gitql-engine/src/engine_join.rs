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
        let left_rows = selected_rows_per_table.get(&join.left).unwrap();

        for right in right_rows {
            for left in left_rows {
                let mut new_row_values: Vec<Value> = vec![];
                new_row_values.append(&mut right.values.clone());
                new_row_values.append(&mut left.values.clone());
                all_rows.push(Row {
                    values: new_row_values,
                });
            }
        }
    }
}
