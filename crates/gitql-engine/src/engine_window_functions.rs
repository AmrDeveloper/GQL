use std::collections::HashMap;

use gitql_ast::statement::GroupByStatement;
use gitql_ast::statement::OrderByStatement;
use gitql_ast::statement::WindowDefinition;
use gitql_ast::statement::WindowFunctionKind;
use gitql_ast::statement::WindowFunctionsStatement;
use gitql_core::environment::Environment;
use gitql_core::object::GitQLObject;

use crate::engine_evaluator::evaluate_expression;
use crate::engine_executor::resolve_actual_column_name;
use crate::engine_group::execute_group_by_statement;
use crate::engine_ordering::execute_order_by_statement;

pub(crate) fn execute_window_functions_statement(
    env: &mut Environment,
    statement: &WindowFunctionsStatement,
    gitql_object: &mut GitQLObject,
    alias_table: &HashMap<String, String>,
) -> Result<(), String> {
    if gitql_object.is_empty() {
        return Ok(());
    }

    if gitql_object.len() > 1 {
        gitql_object.flat()
    }

    let main_group = &mut gitql_object.groups[0];
    let rows_len = main_group.rows.len();

    for (result_column_name, function) in statement.functions.iter() {
        let args_len = function.arguments.len();
        let column_name = resolve_actual_column_name(alias_table, result_column_name);
        let column_index = gitql_object
            .titles
            .iter()
            .position(|r| r.eq(&column_name))
            .unwrap();

        // Apply window definition to end up with frames
        apply_window_definition_on_gitql_object(env, gitql_object, &function.window_definition)?;

        // Run window function on each group
        for frame_index in 0..gitql_object.len() {
            let mut frame_values = Vec::with_capacity(rows_len);
            let frame = &mut gitql_object.groups[frame_index];
            for row in frame.rows.iter_mut() {
                let mut row_selected_values = Vec::with_capacity(args_len);
                for argument in function.arguments.iter() {
                    let argument =
                        evaluate_expression(env, argument, &gitql_object.titles, &row.values)?;

                    row_selected_values.push(argument);
                }

                frame_values.push(row_selected_values);
            }

            if frame_values.is_empty() {
                continue;
            }

            // Evaluate function for this frame
            match function.kind {
                WindowFunctionKind::AggregatedWindowFunction => {
                    let aggregation_function =
                        env.aggregation_function(&function.function_name).unwrap();
                    let aggregated_value = aggregation_function(&frame_values);

                    for row in frame.rows.iter_mut() {
                        row.values[column_index] = aggregated_value.clone();
                    }
                }
                WindowFunctionKind::PureWindowFunction => {
                    let window_function = env.window_function(&function.function_name).unwrap();
                    let window_values = window_function(&frame_values);
                    for (index, value) in window_values.iter().enumerate() {
                        frame.rows[index].values[column_index] = value.clone();
                    }
                }
            };
        }
    }

    gitql_object.flat();

    Ok(())
}

fn apply_window_definition_on_gitql_object(
    env: &mut Environment,
    gitql_object: &mut GitQLObject,
    window_definition: &WindowDefinition,
) -> Result<(), String> {
    // Apply partitioning on the main group
    if let Some(partition_by) = &window_definition.partitioning_clause {
        let group_by = GroupByStatement {
            values: vec![partition_by.expr.clone()],
            has_with_roll_up: false,
        };
        execute_group_by_statement(env, &group_by, gitql_object)?;
    }

    // Apply ordering each partition
    if let Some(order_by) = &window_definition.ordering_clause {
        let order_by = OrderByStatement {
            arguments: vec![order_by.expr.clone()],
            sorting_orders: vec![order_by.ordering.clone()],
        };

        for index in 0..gitql_object.len() {
            execute_order_by_statement(env, &order_by, gitql_object, index)?;
        }
    }

    // TODO: Convert groups into window frames
    Ok(())
}
