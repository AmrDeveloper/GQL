use std::cmp;
use std::cmp::Ordering;
use std::collections::hash_map::Entry::Vacant;
use std::collections::HashMap;
use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::io::Write;

use gitql_ast::expression::Expression;
use gitql_ast::expression::ExpressionKind;
use gitql_ast::statement::AggregateValue;
use gitql_ast::statement::AggregationsStatement;
use gitql_ast::statement::DoStatement;
use gitql_ast::statement::GlobalVariableStatement;
use gitql_ast::statement::GroupByStatement;
use gitql_ast::statement::HavingStatement;
use gitql_ast::statement::IntoStatement;
use gitql_ast::statement::LimitStatement;
use gitql_ast::statement::OffsetStatement;
use gitql_ast::statement::OrderByStatement;
use gitql_ast::statement::SelectStatement;
use gitql_ast::statement::SortingOrder;
use gitql_ast::statement::Statement;
use gitql_ast::statement::StatementKind::*;
use gitql_ast::statement::WhereStatement;
use gitql_core::environment::Environment;
use gitql_core::object::GitQLObject;
use gitql_core::object::Group;
use gitql_core::object::Row;
use gitql_core::types::DataType;
use gitql_core::value::Value;

use crate::data_provider::DataProvider;
use crate::engine_evaluator::evaluate_expression;
use crate::engine_filter::apply_filter_operation;
use crate::engine_join::apply_join_operation;

#[allow(clippy::borrowed_box)]
pub fn execute_statement(
    env: &mut Environment,
    statement: &Box<dyn Statement>,
    data_provider: &Box<dyn DataProvider>,
    gitql_object: &mut GitQLObject,
    alias_table: &mut HashMap<String, String>,
    hidden_selection: &HashMap<String, Vec<String>>,
) -> Result<(), String> {
    match statement.kind() {
        Do => {
            let statement = statement.as_any().downcast_ref::<DoStatement>().unwrap();
            execute_do_statement(env, statement, gitql_object)
        }
        Select => {
            let statement = statement
                .as_any()
                .downcast_ref::<SelectStatement>()
                .unwrap();

            execute_select_statement(
                env,
                statement,
                alias_table,
                data_provider,
                gitql_object,
                hidden_selection,
            )
        }
        Where => {
            let statement = statement.as_any().downcast_ref::<WhereStatement>().unwrap();
            execute_where_statement(env, statement, gitql_object)
        }
        Having => {
            let statement = statement
                .as_any()
                .downcast_ref::<HavingStatement>()
                .unwrap();
            execute_having_statement(env, statement, gitql_object)
        }
        Limit => {
            let statement = statement.as_any().downcast_ref::<LimitStatement>().unwrap();
            execute_limit_statement(statement, gitql_object)
        }
        Offset => {
            let statement = statement
                .as_any()
                .downcast_ref::<OffsetStatement>()
                .unwrap();
            execute_offset_statement(statement, gitql_object)
        }
        OrderBy => {
            let statement = statement
                .as_any()
                .downcast_ref::<OrderByStatement>()
                .unwrap();
            execute_order_by_statement(env, statement, gitql_object)
        }
        GroupBy => {
            let statement = statement
                .as_any()
                .downcast_ref::<GroupByStatement>()
                .unwrap();
            execute_group_by_statement(env, statement, gitql_object)
        }
        AggregateFunction => {
            let statement = statement
                .as_any()
                .downcast_ref::<AggregationsStatement>()
                .unwrap();
            execute_aggregation_function_statement(env, statement, gitql_object, alias_table)
        }
        Into => {
            let statement = statement.as_any().downcast_ref::<IntoStatement>().unwrap();
            execute_into_statement(statement, gitql_object)
        }
        GlobalVariable => {
            let statement = statement
                .as_any()
                .downcast_ref::<GlobalVariableStatement>()
                .unwrap();
            execute_global_variable_statement(env, statement)
        }
    }
}

fn execute_do_statement(
    env: &mut Environment,
    statement: &DoStatement,
    gitql_object: &mut GitQLObject,
) -> Result<(), String> {
    let row_values = &gitql_object.groups[0].rows[0].values;
    evaluate_expression(env, &statement.expression, &gitql_object.titles, row_values)?;
    Ok(())
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
            table_titles.push(get_column_name(alias_table, selected_column));
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
    selected_expr: &[Box<dyn Expression>],
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

            if index < row.values.len() && row.values[value_index].data_type() != DataType::Null {
                continue;
            }

            // Ignore evaluating expression if it symbol, that mean it a reference to aggregated value or function
            let value = if expr.kind() == ExpressionKind::Symbol {
                Value::Null
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
    // Perform where command only on the first group
    // because group by command not executed yet
    let condition = &statement.condition;
    let main_group = &gitql_object.groups.first().unwrap().rows;
    let rows = apply_filter_operation(env, condition, &gitql_object.titles, main_group)?;
    let filtered_group: Group = Group { rows };

    // Update the main group with the filtered data
    gitql_object.groups.remove(0);
    gitql_object.groups.push(filtered_group);

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
    let condition = &statement.condition;
    let main_group = &gitql_object.groups.first().unwrap().rows;
    let rows = apply_filter_operation(env, condition, &gitql_object.titles, main_group)?;
    let filtered_group: Group = Group { rows };

    // Update the main group with the filtered data
    gitql_object.groups.remove(0);
    gitql_object.groups.push(filtered_group);

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
    main_group
        .rows
        .drain(0..cmp::min(statement.count, main_group.len()));

    Ok(())
}

fn execute_order_by_statement(
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

    main_group.rows.sort_by(|a, b| {
        // The default ordering
        let mut ordering = Ordering::Equal;

        for i in 0..statement.arguments.len() {
            let argument = &statement.arguments[i];
            // No need to compare if the ordering argument is constants
            if argument.is_const() {
                continue;
            }

            // Compare the two set of attributes using the current argument
            let first = &evaluate_expression(env, argument, &gitql_object.titles, &a.values)
                .unwrap_or(Value::Null);
            let other = &evaluate_expression(env, argument, &gitql_object.titles, &b.values)
                .unwrap_or(Value::Null);

            let current_ordering = first.compare(other);

            // If comparing result still equal, check the next argument
            if current_ordering == Ordering::Equal {
                continue;
            }

            // Reverse the order if its not ASC order
            ordering = if statement.sorting_orders[i] == SortingOrder::Descending {
                current_ordering
            } else {
                current_ordering.reverse()
            };
            break;
        }

        ordering
    });

    Ok(())
}

fn execute_group_by_statement(
    env: &mut Environment,
    statement: &GroupByStatement,
    gitql_object: &mut GitQLObject,
) -> Result<(), String> {
    if gitql_object.is_empty() {
        return Ok(());
    }

    let main_group: Group = gitql_object.groups.remove(0);
    if main_group.is_empty() {
        return Ok(());
    }

    // Mapping each unique value to it group index
    let mut groups_map: HashMap<u64, usize> = HashMap::new();

    // Track current group index
    let mut next_group_index = 0;
    let values_count = statement.values.len();

    for object in main_group.rows.into_iter() {
        let mut row_values: Vec<String> = Vec::with_capacity(values_count);

        for expression in &statement.values {
            let value = evaluate_expression(env, expression, &gitql_object.titles, &object.values)?;
            row_values.push(value.to_string());
        }

        // Compute the hash for row of values
        let mut hasher = DefaultHasher::new();
        row_values.hash(&mut hasher);
        let values_hash = hasher.finish();

        // Push a new group for this unique value and update the next index
        if let Vacant(e) = groups_map.entry(values_hash) {
            e.insert(next_group_index);
            next_group_index += 1;
            gitql_object.groups.push(Group { rows: vec![object] });
        }
        // If there is an existing group for this value, append current object to it
        else {
            let index = *groups_map.get(&values_hash).unwrap();
            let target_group = &mut gitql_object.groups[index];
            target_group.rows.push(object);
        }
    }

    Ok(())
}

fn execute_aggregation_function_statement(
    env: &mut Environment,
    statement: &AggregationsStatement,
    gitql_object: &mut GitQLObject,
    alias_table: &HashMap<String, String>,
) -> Result<(), String> {
    // Make sure you have at least one aggregation function to calculate
    let aggregations_map = &statement.aggregations;
    if aggregations_map.is_empty() {
        return Ok(());
    }

    // Used to determine if group by statement is executed before or not
    let groups_count = gitql_object.len();

    // We should run aggregation function for each group
    for group in &mut gitql_object.groups {
        // No need to apply all aggregation if there is no selected elements
        if group.is_empty() {
            continue;
        }

        // Resolve all aggregations functions first
        for aggregation in aggregations_map {
            if let AggregateValue::Function(function, arguments) = aggregation.1 {
                // Get alias name if exists or column name by default

                let result_column_name = aggregation.0;
                let column_name = get_column_name(alias_table, result_column_name);

                let column_index = gitql_object
                    .titles
                    .iter()
                    .position(|r| r.eq(&column_name))
                    .unwrap();

                // Evaluate the Arguments to Values
                let mut group_values: Vec<Vec<Value>> = Vec::with_capacity(group.rows.len());
                for object in &mut group.rows {
                    let mut row_values: Vec<Value> = Vec::with_capacity(object.values.len());
                    for argument in arguments {
                        let value = evaluate_expression(
                            env,
                            argument,
                            &gitql_object.titles,
                            &object.values,
                        )?;

                        row_values.push(value);
                    }

                    group_values.push(row_values);
                }

                // Get the target aggregation function
                let aggregation_function = env.aggregation_function(function.as_str()).unwrap();
                let result = &aggregation_function(group_values);

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
        for aggregation in aggregations_map {
            if let AggregateValue::Expression(expr) = aggregation.1 {
                // Get alias name if exists or column name by default
                let result_column_name = aggregation.0;
                let column_name = get_column_name(alias_table, result_column_name);

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
        if groups_count > 1 {
            group.rows.drain(1..);
        }
    }

    Ok(())
}

fn execute_into_statement(
    statement: &IntoStatement,
    gitql_object: &mut GitQLObject,
) -> Result<(), String> {
    let mut buffer = String::new();

    // Headers
    let header = gitql_object.titles.join(",");
    buffer.push_str(&header);
    buffer.push('\n');

    // Rows
    let rows = &gitql_object.groups[0].rows;
    for row in rows {
        let row_values: Vec<String> = row.values.iter().map(|r| r.to_string()).collect();
        buffer.push_str(&row_values.join(","));
        buffer.push('\n');
    }

    let file_result = std::fs::File::create(statement.file_path.clone());
    if let Err(error) = file_result {
        return Err(error.to_string());
    }

    let mut file = file_result.ok().unwrap();
    let write_result = file.write_all(buffer.as_bytes());
    if let Err(error) = write_result {
        return Err(error.to_string());
    }

    Ok(())
}

pub fn execute_global_variable_statement(
    env: &mut Environment,
    statement: &GlobalVariableStatement,
) -> Result<(), String> {
    let value = evaluate_expression(env, &statement.value, &[], &vec![])?;
    env.globals.insert(statement.name.to_string(), value);
    Ok(())
}

#[inline(always)]
pub fn get_column_name(alias_table: &HashMap<String, String>, name: &str) -> String {
    alias_table
        .get(name)
        .unwrap_or(&name.to_string())
        .to_string()
}
