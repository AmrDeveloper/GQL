use gitql_ast::statement::OrderByStatement;
use gitql_ast::statement::OverClause;
use gitql_ast::statement::WindowFunctionKind;
use gitql_ast::statement::WindowFunctionsStatement;
use gitql_core::environment::Environment;
use gitql_core::object::GitQLObject;

use crate::engine_evaluator::evaluate_expression;
use crate::engine_ordering::execute_order_by_statement;

pub(crate) fn execute_window_functions_statement(
    env: &mut Environment,
    statement: &WindowFunctionsStatement,
    gitql_object: &mut GitQLObject,
) -> Result<(), String> {
    if gitql_object.is_empty() {
        return Ok(());
    }

    if gitql_object.len() > 1 {
        gitql_object.flat()
    }

    let main_group = &mut gitql_object.groups[0];
    let rows_len = main_group.rows.len();

    for (column_name, function) in statement.functions.iter() {
        apply_order_clauses_on_group(env, gitql_object, &function.order_clauses)?;

        let args_len = function.arguments.len();
        let column_index = gitql_object
            .titles
            .iter()
            .position(|r| r.eq(column_name))
            .unwrap();

        let mut group_arguments = Vec::with_capacity(rows_len);

        for row in gitql_object.groups[0].rows.iter_mut() {
            // Evaluate current Row arguments to values
            let mut row_arguments = Vec::with_capacity(args_len);
            for argument in function.arguments.iter() {
                let argument =
                    evaluate_expression(env, argument, &gitql_object.titles, &row.values)?;
                row_arguments.push(argument);
            }

            group_arguments.push(row_arguments);

            let window_function = match function.kind {
                WindowFunctionKind::AggregatedWindowFunction => {
                    env.aggregation_function(&function.function_name).unwrap()
                }
                WindowFunctionKind::PureWindowFunction => {
                    env.window_function(&function.function_name).unwrap()
                }
            };
            let evaluated_value = window_function(group_arguments.clone());
            row.values[column_index] = evaluated_value;
        }
    }

    Ok(())
}

fn apply_order_clauses_on_group(
    env: &mut Environment,
    gitql_object: &mut GitQLObject,
    over_clauses: &OverClause,
) -> Result<(), String> {
    for clause in over_clauses.clauses.iter() {
        if let Some(statement) = clause.as_any().downcast_ref::<OrderByStatement>() {
            execute_order_by_statement(env, statement, gitql_object)?;
            continue;
        }
    }
    Ok(())
}
