use std::cmp;
use std::collections::HashMap;

use gitql_ast::expression::Expr;
use gitql_ast::expression::ExprKind;
use gitql_ast::statement::AggregateValue;
use gitql_ast::statement::AggregationsStatement;
use gitql_ast::statement::HavingStatement;
use gitql_ast::statement::LimitStatement;
use gitql_ast::statement::OffsetStatement;
use gitql_ast::statement::QualifyStatement;
use gitql_ast::statement::SelectStatement;
use gitql_ast::statement::Statement;
use gitql_ast::statement::WhereStatement;
use gitql_core::environment::Environment;
use gitql_core::object::GitQLObject;
use gitql_core::object::Group;
use gitql_core::object::Row;
use gitql_core::values::null::NullValue;
use gitql_core::values::Value;

use crate::data_provider::DataProvider;
use crate::engine_evaluator::evaluate_expression;
use crate::engine_filter::apply_filter_operation;
use crate::engine_group::execute_group_by_statement;
use crate::engine_join::apply_join_operation;
use crate::engine_ordering::execute_order_by_statement;
use crate::engine_output_into::execute_into_statement;
use crate::engine_window_functions::execute_window_functions_statement;

#[allow(clippy::borrowed_box)]
pub fn execute_statement(
    env: &mut Environment,
    statement: &Statement,
    data_provider: &Box<dyn DataProvider>,
    gitql_object: &mut GitQLObject,
    alias_table: &mut HashMap<String, String>,
    hidden_selection: &HashMap<String, Vec<String>>,
    has_group_by_statement: bool,
) -> Result<(), String> {
    match statement {
        Statement::Select(statement) => execute_select_statement(
            env,
            statement,
            alias_table,
            data_provider,
            gitql_object,
            hidden_selection,
        ),
        Statement::Where(statement) => execute_where_statement(env, statement, gitql_object),
        Statement::Having(statement) => execute_having_statement(env, statement, gitql_object),
        Statement::Limit(statement) => execute_limit_statement(statement, gitql_object),
        Statement::Offset(statement) => execute_offset_statement(env, statement, gitql_object),
        Statement::OrderBy(statement) => {
            if gitql_object.len() > 1 {
                gitql_object.flat();
            }
            let main_group_index = 0;
            execute_order_by_statement(env, statement, gitql_object, main_group_index)
        }
        Statement::GroupBy(statement) => execute_group_by_statement(env, statement, gitql_object),
        Statement::AggregateFunction(statement) => execute_aggregation_functions_statement(
            env,
            statement,
            gitql_object,
            alias_table,
            has_group_by_statement,
        ),
        Statement::WindowFunction(statement) => {
            execute_window_functions_statement(env, statement, gitql_object, alias_table)
        }
        Statement::Qualify(statement) => execute_qualify_statement(env, statement, gitql_object),
        Statement::Into(statement) => execute_into_statement(statement, gitql_object),
    }
}

#[allow(clippy::borrowed_box)]
fn execute_select_statement(
    env: &mut Environment,
    statement: &SelectStatement,
    alias_table: &HashMap<String, String>,
    data_provider: &Box<dyn DataProvider>,
    gitql_object: &mut GitQLObject,
    hidden_selections: &HashMap<String, Vec<String>>,
) -> Result<(), String> {
    let mut selected_rows_per_table: HashMap<String, Vec<Row>> = HashMap::new();
    let mut hidden_selection_count_per_table: HashMap<String, usize> = HashMap::new();

    let mut titles: Vec<String> = vec![];
    let mut hidden_sum = 0;

    for table_selection in &statement.table_selections {
        // Select objects from the target table
        let table_name = &table_selection.table_name;
        let selected_columns = &mut table_selection.columns_names.to_owned();

        // Insert Hidden selection items for this table first
        let mut hidden_selection_count = 0;
        if let Some(table_hidden_selection) = hidden_selections.get(table_name) {
            for hidden_selection in table_hidden_selection {
                if !selected_columns.contains(hidden_selection) {
                    selected_columns.insert(0, hidden_selection.to_string());
                    hidden_selection_count += 1;
                }
            }
        }

        hidden_selection_count_per_table.insert(table_name.to_string(), hidden_selection_count);

        // Calculate list of titles once per table
        let mut table_titles = vec![];
        for selected_column in selected_columns.iter_mut() {
            table_titles.push(resolve_actual_column_name(alias_table, selected_column));
        }

        // Call the provider only if table name is not empty
        let selected_rows: Vec<Row> = if table_name.is_empty() {
            vec![Row { values: vec![] }]
        } else {
            data_provider.provide(table_name, selected_columns)?
        };

        selected_rows_per_table.insert(table_name.to_string(), selected_rows);

        // Append hidden selection in the right position
        // at the end all hidden selections will be first
        let hidden_selection_titles = &table_titles[..hidden_selection_count];
        titles.splice(hidden_sum..hidden_sum, hidden_selection_titles.to_vec());

        // Non hidden selection should be inserted at the end
        let selection_titles = &table_titles[hidden_selection_count..];
        titles.extend_from_slice(selection_titles);
        hidden_sum += hidden_selection_count;
    }

    gitql_object.titles.append(&mut titles);

    // Apply joins operations if exists
    let mut selected_rows: Vec<Row> = vec![];
    apply_join_operation(
        env,
        &mut selected_rows,
        &statement.joins,
        &statement.table_selections,
        &mut selected_rows_per_table,
        &hidden_selection_count_per_table,
        &gitql_object.titles,
    )?;

    // Execute Selected expressions if exists
    if !statement.selected_expr.is_empty() {
        execute_expression_selection(
            env,
            &mut selected_rows,
            &gitql_object.titles,
            &statement.selected_expr_titles,
            &statement.selected_expr,
        )?;
    }

    let main_group = Group {
        rows: selected_rows,
    };

    gitql_object.groups.push(main_group);

    Ok(())
}

#[inline(always)]
fn execute_expression_selection(
    env: &mut Environment,
    selected_rows: &mut [Row],
    object_titles: &[String],
    selected_expr_titles: &[String],
    selected_expr: &[Box<dyn Expr>],
) -> Result<(), String> {
    // Cache the index of each expression position to provide fast insertion
    let mut titles_index_map: HashMap<String, usize> = HashMap::new();
    for expr_column_title in selected_expr_titles {
        let expr_title_index = object_titles
            .iter()
            .position(|r| r.eq(expr_column_title))
            .unwrap();
        titles_index_map.insert(expr_column_title.to_string(), expr_title_index);
    }

    for row in selected_rows.iter_mut() {
        for (index, expr) in selected_expr.iter().enumerate() {
            let expr_title = &selected_expr_titles[index];
            let value_index = *titles_index_map.get(expr_title).unwrap();

            if index < row.values.len() && !row.values[value_index].is_null() {
                continue;
            }

            // Ignore evaluating expression if it symbol, that mean it a reference to aggregated value or function
            let value = if expr.kind() == ExprKind::Symbol {
                Box::new(NullValue)
            } else {
                evaluate_expression(env, expr, object_titles, &row.values)?
            };

            if index >= row.values.len() {
                row.values.push(value);
            } else {
                row.values[value_index] = value;
            }
        }
    }
    Ok(())
}

fn execute_where_statement(
    env: &mut Environment,
    statement: &WhereStatement,
    gitql_object: &mut GitQLObject,
) -> Result<(), String> {
    if gitql_object.is_empty() {
        return Ok(());
    }

    apply_filter_operation(
        env,
        &statement.condition,
        &gitql_object.titles,
        &mut gitql_object.groups[0].rows,
    )?;

    Ok(())
}

fn execute_having_statement(
    env: &mut Environment,
    statement: &HavingStatement,
    gitql_object: &mut GitQLObject,
) -> Result<(), String> {
    if gitql_object.is_empty() {
        return Ok(());
    }

    if gitql_object.len() > 1 {
        gitql_object.flat()
    }

    // Perform where command only on the first group
    // because group by command not executed yet
    apply_filter_operation(
        env,
        &statement.condition,
        &gitql_object.titles,
        &mut gitql_object.groups[0].rows,
    )?;

    Ok(())
}

fn execute_qualify_statement(
    env: &mut Environment,
    statement: &QualifyStatement,
    gitql_object: &mut GitQLObject,
) -> Result<(), String> {
    if gitql_object.is_empty() {
        return Ok(());
    }

    apply_filter_operation(
        env,
        &statement.condition,
        &gitql_object.titles,
        &mut gitql_object.groups[0].rows,
    )?;

    Ok(())
}

fn execute_limit_statement(
    statement: &LimitStatement,
    gitql_object: &mut GitQLObject,
) -> Result<(), String> {
    if gitql_object.is_empty() {
        return Ok(());
    }

    if gitql_object.len() > 1 {
        gitql_object.flat()
    }

    let main_group: &mut Group = &mut gitql_object.groups[0];
    if statement.count <= main_group.len() {
        main_group.rows.drain(statement.count..main_group.len());
    }

    Ok(())
}

fn execute_offset_statement(
    env: &mut Environment,
    statement: &OffsetStatement,
    gitql_object: &mut GitQLObject,
) -> Result<(), String> {
    if gitql_object.is_empty() {
        return Ok(());
    }

    if gitql_object.len() > 1 {
        gitql_object.flat()
    }

    let main_group: &mut Group = &mut gitql_object.groups[0];
    let start = &evaluate_expression(
        env,
        &statement.start,
        &gitql_object.titles,
        &main_group.rows[0].values,
    )?;

    // If start evaluates to NULL, it is treated the same as OFFSET 0
    if start.is_null() {
        return Ok(());
    }

    let offset = start.as_int().unwrap() as usize;
    let main_group: &mut Group = &mut gitql_object.groups[0];
    main_group.rows.drain(0..cmp::min(offset, main_group.len()));

    Ok(())
}

fn execute_aggregation_functions_statement(
    env: &mut Environment,
    statement: &AggregationsStatement,
    gitql_object: &mut GitQLObject,
    alias_table: &HashMap<String, String>,
    is_query_has_group_by: bool,
) -> Result<(), String> {
    // Make sure you have at least one aggregation function to calculate
    let aggregations_map = &statement.aggregations;
    if aggregations_map.is_empty() {
        return Ok(());
    }

    // We should run aggregation function for each group
    for group in &mut gitql_object.groups {
        // No need to apply all aggregation if there is no selected elements
        if group.is_empty() {
            continue;
        }

        // Resolve all aggregations functions first
        for (result_column_name, aggregation) in aggregations_map {
            if let AggregateValue::Function(function, arguments) = aggregation {
                // Get alias name if exists or column name by default
                let column_name = resolve_actual_column_name(alias_table, result_column_name);
                let column_index = gitql_object
                    .titles
                    .iter()
                    .position(|r| r.eq(&column_name))
                    .unwrap();

                // Evaluate the Arguments to Values
                let mut group_arguments: Vec<Vec<Box<dyn Value>>> =
                    Vec::with_capacity(group.rows.len());
                for object in &mut group.rows {
                    let mut row_values: Vec<Box<dyn Value>> =
                        Vec::with_capacity(object.values.len());
                    for argument in arguments {
                        let value = evaluate_expression(
                            env,
                            argument,
                            &gitql_object.titles,
                            &object.values,
                        )?;

                        row_values.push(value);
                    }

                    group_arguments.push(row_values);
                }

                // Get the target aggregation function
                let aggregation_function = env.aggregation_function(function).unwrap();
                let result = &aggregation_function(&group_arguments);

                // Insert the calculated value in the group objects
                for object in &mut group.rows {
                    if column_index < object.values.len() {
                        object.values[column_index] = result.clone();
                    } else {
                        object.values.push(result.clone());
                    }
                }
            }
        }

        // Resolve aggregations expressions
        for (result_column_name, aggregation) in aggregations_map {
            if let AggregateValue::Expression(expr) = aggregation {
                // Get alias name if exists or column name by default
                let column_name = resolve_actual_column_name(alias_table, result_column_name);
                let column_index = gitql_object
                    .titles
                    .iter()
                    .position(|r| r.eq(&column_name))
                    .unwrap();

                // Insert the calculated value in the group objects
                for object in group.rows.iter_mut() {
                    let result =
                        evaluate_expression(env, expr, &gitql_object.titles, &object.values)?;
                    if column_index < object.values.len() {
                        object.values[column_index] = result.clone();
                    } else {
                        object.values.push(result.clone());
                    }
                }
            }
        }

        // In case of group by statement is executed
        // Remove all elements expect the first one
        if is_query_has_group_by {
            group.rows.drain(1..);
        }
    }

    Ok(())
}

#[inline(always)]
pub fn resolve_actual_column_name(alias_table: &HashMap<String, String>, name: &str) -> String {
    if let Some(column_name) = alias_table.get(name) {
        return column_name.to_string();
    }

    name.to_string()
}
