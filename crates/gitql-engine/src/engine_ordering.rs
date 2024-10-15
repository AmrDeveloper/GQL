use std::cmp::Ordering;
use std::collections::HashMap;

use gitql_ast::statement::OrderByStatement;
use gitql_ast::statement::SortingOrder;
use gitql_core::environment::Environment;
use gitql_core::object::GitQLObject;
use gitql_core::object::Group;
use gitql_core::values::base::Value;
use gitql_core::values::null::NullValue;

use crate::engine_evaluator::evaluate_expression;

pub(crate) fn execute_order_by_statement(
    env: &mut Environment,
    statement: &OrderByStatement,
    gitql_object: &mut GitQLObject,
) -> Result<(), String> {
    if gitql_object.is_empty() {
        return Ok(());
    }

    if gitql_object.len() > 1 {
        gitql_object.flat();
    }

    let main_group: &mut Group = &mut gitql_object.groups[0];
    if main_group.is_empty() {
        return Ok(());
    }

    let rows_len = main_group.rows.len();
    let arguments_len = statement.arguments.len();
    let main_group_rows = &main_group.rows;
    let titles = &gitql_object.titles;

    let mut eval_map: HashMap<usize, Vec<Box<dyn Value>>> = HashMap::with_capacity(rows_len);

    for row in main_group_rows.iter() {
        let row_addr = row.values.as_ptr() as usize;
        let mut argument_evals: Vec<Box<dyn Value>> = Vec::with_capacity(arguments_len);
        for argument in statement.arguments.iter() {
            // No need to compare if the ordering argument is constants
            if argument.is_const() {
                argument_evals.push(Box::new(NullValue));
                continue;
            }

            let value = &evaluate_expression(env, argument, titles, &row.values)?;
            argument_evals.push(value.to_owned());
        }

        eval_map.insert(row_addr, argument_evals);
    }

    main_group.rows.sort_by(|a, b| {
        for arg_index in 0..arguments_len {
            let argument = &statement.arguments[arg_index];
            // No need to compare if the ordering argument is constants
            if argument.is_const() {
                continue;
            }

            // Use the Memory address of A, B as Map keys
            let a_addr = a.values.as_ptr() as usize;
            let b_addr = b.values.as_ptr() as usize;

            // Get pre evaluated values from the eval map using addr as key, arg index as offset
            let a_value = &eval_map.get(&a_addr).unwrap()[arg_index];
            let b_value = &eval_map.get(&b_addr).unwrap()[arg_index];

            // Calculate the ordering
            if let Some(order) = a_value.compare(b_value) {
                // If comparing result still equal, check the next argument
                if order == Ordering::Equal {
                    continue;
                }

                // Reverse the order if DESC order
                return if statement.sorting_orders[arg_index] == SortingOrder::Descending {
                    order.reverse()
                } else {
                    order
                };
            }
        }

        Ordering::Equal
    });

    Ok(())
}
