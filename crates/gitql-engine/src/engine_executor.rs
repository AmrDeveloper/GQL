use std::cmp;
use std::cmp::Ordering;
use std::collections::HashMap;

use gitql_ast::aggregation::AGGREGATIONS;
use gitql_ast::environment::Environment;
use gitql_ast::object::flat_gql_groups;
use gitql_ast::object::GQLObject;
use gitql_ast::statement::AggregateValue;
use gitql_ast::statement::AggregationsStatement;
use gitql_ast::statement::GlobalVariableStatement;
use gitql_ast::statement::GroupByStatement;
use gitql_ast::statement::HavingStatement;
use gitql_ast::statement::LimitStatement;
use gitql_ast::statement::OffsetStatement;
use gitql_ast::statement::OrderByStatement;
use gitql_ast::statement::SelectStatement;
use gitql_ast::statement::SortingOrder;
use gitql_ast::statement::Statement;
use gitql_ast::statement::StatementKind::*;
use gitql_ast::statement::WhereStatement;
use gitql_ast::value::Value;

use crate::engine_evaluator::evaluate_expression;
use crate::engine_function::get_column_name;
use crate::engine_function::select_gql_objects;

#[allow(clippy::borrowed_box)]
pub fn execute_statement(
    env: &mut Environment,
    statement: &Box<dyn Statement>,
    repo: &gix::Repository,
    groups: &mut Vec<Vec<GQLObject>>,
    alias_table: &mut HashMap<String, String>,
    hidden_selection: &Vec<String>,
) -> Result<(), String> {
    match statement.get_statement_kind() {
        Select => {
            let statement = statement
                .as_any()
                .downcast_ref::<SelectStatement>()
                .unwrap();

            // Copy alias table to be last later for Aggregations functions
            for alias in &statement.alias_table {
                alias_table.insert(alias.0.to_string(), alias.1.to_string());
            }

            execute_select_statement(env, statement, repo, groups, hidden_selection)
        }
        Where => {
            let statement = statement.as_any().downcast_ref::<WhereStatement>().unwrap();
            execute_where_statement(env, statement, groups)
        }
        Having => {
            let statement = statement
                .as_any()
                .downcast_ref::<HavingStatement>()
                .unwrap();
            execute_having_statement(env, statement, groups)
        }
        Limit => {
            let statement = statement.as_any().downcast_ref::<LimitStatement>().unwrap();
            execute_limit_statement(statement, groups)
        }
        Offset => {
            let statement = statement
                .as_any()
                .downcast_ref::<OffsetStatement>()
                .unwrap();
            execute_offset_statement(statement, groups)
        }
        OrderBy => {
            let statement = statement
                .as_any()
                .downcast_ref::<OrderByStatement>()
                .unwrap();
            execute_order_by_statement(env, statement, groups)
        }
        GroupBy => {
            let statement = statement
                .as_any()
                .downcast_ref::<GroupByStatement>()
                .unwrap();
            execute_group_by_statement(statement, groups)
        }
        AggregateFunction => {
            let statement = statement
                .as_any()
                .downcast_ref::<AggregationsStatement>()
                .unwrap();
            execute_aggregation_function_statement(env, statement, groups, alias_table)
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

fn execute_select_statement(
    env: &mut Environment,
    statement: &SelectStatement,
    repo: &gix::Repository,
    groups: &mut Vec<Vec<GQLObject>>,
    hidden_selections: &Vec<String>,
) -> Result<(), String> {
    // Append hidden selection to the selected fields names
    let mut fields_names = statement.fields_names.to_owned();
    if !statement.table_name.is_empty() {
        for hidden in hidden_selections {
            if !fields_names.contains(hidden) {
                fields_names.insert(0, hidden.to_string());
            }
        }
    }

    // Select obects from the target table
    let mut objects = select_gql_objects(
        env,
        repo,
        statement.table_name.to_string(),
        &fields_names,
        &statement.fields_values,
        &statement.alias_table,
    )?;

    // Push the selected elements as a first group
    if groups.is_empty() {
        groups.push(objects);
    } else {
        groups[0].append(&mut objects);
    }

    Ok(())
}

fn execute_where_statement(
    env: &mut Environment,
    statement: &WhereStatement,
    groups: &mut Vec<Vec<GQLObject>>,
) -> Result<(), String> {
    if groups.is_empty() {
        return Ok(());
    }

    // Perform where command only on the first group
    // because group by command not executed yet
    let mut filtered_group: Vec<GQLObject> = vec![];
    let first_group = groups.first().unwrap().iter();
    for object in first_group {
        let eval_result = evaluate_expression(env, &statement.condition, &object.attributes);
        if eval_result.is_err() {
            return Err(eval_result.err().unwrap());
        }

        if eval_result.ok().unwrap().as_bool() {
            filtered_group.push(object.clone());
        }
    }

    // Update the main group with the filtered data
    groups.remove(0);
    groups.push(filtered_group);

    Ok(())
}

fn execute_having_statement(
    env: &mut Environment,
    statement: &HavingStatement,
    groups: &mut Vec<Vec<GQLObject>>,
) -> Result<(), String> {
    if groups.is_empty() {
        return Ok(());
    }

    if groups.len() > 1 {
        flat_gql_groups(groups);
    }

    // Perform where command only on the first group
    // because groups are already merged
    let mut filtered_group: Vec<GQLObject> = vec![];
    let first_group = groups.first().unwrap().iter();
    for object in first_group {
        let eval_result = evaluate_expression(env, &statement.condition, &object.attributes);
        if eval_result.is_err() {
            return Err(eval_result.err().unwrap());
        }

        if eval_result.ok().unwrap().as_bool() {
            filtered_group.push(object.clone());
        }
    }

    // Update the main group with the filtered data
    groups.remove(0);
    groups.push(filtered_group);

    Ok(())
}

fn execute_limit_statement(
    statement: &LimitStatement,
    groups: &mut Vec<Vec<GQLObject>>,
) -> Result<(), String> {
    if groups.is_empty() {
        return Ok(());
    }

    if groups.len() > 1 {
        flat_gql_groups(groups);
    }

    let main_group: &mut Vec<GQLObject> = groups[0].as_mut();
    if statement.count <= main_group.len() {
        main_group.drain(statement.count..main_group.len());
    }

    Ok(())
}

fn execute_offset_statement(
    statement: &OffsetStatement,
    groups: &mut Vec<Vec<GQLObject>>,
) -> Result<(), String> {
    if groups.is_empty() {
        return Ok(());
    }

    if groups.len() > 1 {
        flat_gql_groups(groups);
    }
    let main_group: &mut Vec<GQLObject> = groups[0].as_mut();
    main_group.drain(0..cmp::min(statement.count, main_group.len()));

    Ok(())
}

fn execute_order_by_statement(
    env: &mut Environment,
    statement: &OrderByStatement,
    groups: &mut Vec<Vec<GQLObject>>,
) -> Result<(), String> {
    if groups.is_empty() {
        return Ok(());
    }

    if groups.len() > 1 {
        flat_gql_groups(groups);
    }

    let main_group: &mut Vec<GQLObject> = groups[0].as_mut();
    if main_group.is_empty() {
        return Ok(());
    }

    main_group.sort_by(|a, b| {
        // The default ordering
        let mut ordering = Ordering::Equal;

        for i in 0..statement.arguments.len() {
            let argument = &statement.arguments[i];
            // No need to compare if the ordering argument is constants
            if argument.is_const() {
                continue;
            }

            // Compare the two set of attributes using the current argument
            let first = &evaluate_expression(env, argument, &a.attributes).unwrap_or(Value::Null);
            let other = &evaluate_expression(env, argument, &b.attributes).unwrap_or(Value::Null);
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
    statement: &GroupByStatement,
    groups: &mut Vec<Vec<GQLObject>>,
) -> Result<(), String> {
    if groups.is_empty() {
        return Ok(());
    }

    let main_group: Vec<GQLObject> = groups.remove(0);
    if main_group.is_empty() {
        return Ok(());
    }

    // Mapping each unique value to it group index
    let mut groups_map: HashMap<String, usize> = HashMap::new();

    // Track current group index
    let mut next_group_index = 0;

    for object in main_group.into_iter() {
        let field_value = object.attributes.get(&statement.field_name).unwrap();

        // If there is an existing group for this value, append current object to it
        if let std::collections::hash_map::Entry::Vacant(e) =
            groups_map.entry(field_value.as_text())
        {
            e.insert(next_group_index);
            next_group_index += 1;
            groups.push(vec![object.to_owned()]);
        }
        // Push a new group for this unique value and update the next index
        else {
            let index = *groups_map.get(&field_value.as_text()).unwrap();
            let target_group = &mut groups[index];
            target_group.push(object.to_owned());
        }
    }

    Ok(())
}

fn execute_aggregation_function_statement(
    env: &mut Environment,
    statement: &AggregationsStatement,
    groups: &mut Vec<Vec<GQLObject>>,
    alias_table: &HashMap<String, String>,
) -> Result<(), String> {
    // Make sure you have at least one aggregation function to calculate
    let aggregations_map = &statement.aggregations;
    if aggregations_map.is_empty() {
        return Ok(());
    }

    // Used to determind if group by statement is executed before or not
    let groups_count = groups.len();

    // We should run aggregation function for each group
    for group in groups {
        // No need to apply all aggregation if there is no selected elements
        if group.is_empty() {
            continue;
        }

        // Resolve all aggregations functions first
        for aggregation in aggregations_map {
            if let AggregateValue::Function(function, argument) = aggregation.1 {
                // Get alias name if exists or column name by default
                let result_column_name = aggregation.0;
                let column_name = get_column_name(alias_table, result_column_name);

                // Get the target aggregation function
                let aggregation_function = AGGREGATIONS.get(function.as_str()).unwrap();
                let result = &aggregation_function(&argument.to_string(), group);

                // Insert the calculated value in the group objects
                for object in group.iter_mut() {
                    object
                        .attributes
                        .insert(column_name.to_string(), result.to_owned());
                } // Get the target aggregation function
                let aggregation_function = AGGREGATIONS.get(function.as_str()).unwrap();
                let result = &aggregation_function(&argument.to_string(), group);

                // Insert the calculated value in the group objects
                for object in group.iter_mut() {
                    object
                        .attributes
                        .insert(column_name.to_string(), result.to_owned());
                }
            }
        }

        // Resolve aggregations expressions
        for aggregation in aggregations_map {
            if let AggregateValue::Expression(expr) = aggregation.1 {
                // Get alias name if exists or column name by default
                let result_column_name = aggregation.0;
                let column_name = get_column_name(alias_table, result_column_name);

                // Insert the calculated value in the group objects
                for object in group.iter_mut() {
                    let result = evaluate_expression(env, expr, &object.attributes)?;

                    object
                        .attributes
                        .insert(column_name.to_string(), result.to_owned());
                }
            }
        }

        // In case of group by statement is exectued
        // Remove all elements expect the first one
        if groups_count > 1 {
            group.drain(1..);
        }
    }

    Ok(())
}

pub fn execute_global_variable_statement(
    env: &mut Environment,
    statement: &GlobalVariableStatement,
) -> Result<(), String> {
    let value = evaluate_expression(env, &statement.value, &HashMap::default())?;
    env.globals.insert(statement.name.to_string(), value);
    Ok(())
}
