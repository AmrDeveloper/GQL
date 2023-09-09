use std::cmp;
use std::collections::HashMap;

use gitql_ast::aggregation::AGGREGATIONS;
use gitql_ast::object::GQLObject;
use gitql_ast::statement::AggregationFunctionsStatement;
use gitql_ast::statement::GroupByStatement;
use gitql_ast::statement::HavingStatement;
use gitql_ast::statement::LimitStatement;
use gitql_ast::statement::OffsetStatement;
use gitql_ast::statement::OrderByStatement;
use gitql_ast::statement::SelectStatement;
use gitql_ast::statement::Statement;
use gitql_ast::statement::StatementKind::*;
use gitql_ast::statement::WhereStatement;
use gitql_ast::types::DataType;

use crate::engine_evaluator::evaluate_expression;
use crate::engine_function::get_column_name;
use crate::engine_function::select_gql_objects;

pub fn execute_statement(
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

            return execute_select_statement(statement, repo, groups, hidden_selection);
        }
        Where => {
            let statement = statement.as_any().downcast_ref::<WhereStatement>().unwrap();
            return execute_where_statement(statement, groups);
        }
        Having => {
            let statement = statement
                .as_any()
                .downcast_ref::<HavingStatement>()
                .unwrap();
            return execute_having_statement(statement, groups);
        }
        Limit => {
            let statement = statement.as_any().downcast_ref::<LimitStatement>().unwrap();
            return execute_limit_statement(statement, groups);
        }
        Offset => {
            let statement = statement
                .as_any()
                .downcast_ref::<OffsetStatement>()
                .unwrap();
            return execute_offset_statement(statement, groups);
        }
        OrderBy => {
            let statement = statement
                .as_any()
                .downcast_ref::<OrderByStatement>()
                .unwrap();
            return execute_order_by_statement(statement, groups);
        }
        GroupBy => {
            let statement = statement
                .as_any()
                .downcast_ref::<GroupByStatement>()
                .unwrap();
            return execute_group_by_statement(statement, groups);
        }
        AggregateFunction => {
            let statement = statement
                .as_any()
                .downcast_ref::<AggregationFunctionsStatement>()
                .unwrap();
            return execute_aggregation_function_statement(statement, groups, &alias_table);
        }
    };
}

fn execute_select_statement(
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
        repo,
        statement.table_name.to_string(),
        &fields_names,
        &statement.fields_values,
        &statement.alias_table,
    );

    // Push the selected elements as a first group
    if groups.is_empty() {
        groups.push(objects);
    } else {
        groups[0].append(&mut objects);
    }

    return Ok(());
}

fn execute_where_statement(
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
        let eval_result = evaluate_expression(&statement.condition, &object.attributes);
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

    return Ok(());
}

fn execute_having_statement(
    statement: &HavingStatement,
    groups: &mut Vec<Vec<GQLObject>>,
) -> Result<(), String> {
    if groups.is_empty() {
        return Ok(());
    }

    if groups.len() > 1 {
        flat_groups(groups);
    }

    // Perform where command only on the first group
    // because groups are already merged
    let mut filtered_group: Vec<GQLObject> = vec![];
    let first_group = groups.first().unwrap().iter();
    for object in first_group {
        let eval_result = evaluate_expression(&statement.condition, &object.attributes);
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

    return Ok(());
}

fn execute_limit_statement(
    statement: &LimitStatement,
    groups: &mut Vec<Vec<GQLObject>>,
) -> Result<(), String> {
    if groups.is_empty() {
        return Ok(());
    }

    if groups.len() > 1 {
        flat_groups(groups);
    }

    let main_group: &mut Vec<GQLObject> = groups[0].as_mut();
    if statement.count <= main_group.len() {
        main_group.drain(statement.count..main_group.len());
    }

    return Ok(());
}

fn execute_offset_statement(
    statement: &OffsetStatement,
    groups: &mut Vec<Vec<GQLObject>>,
) -> Result<(), String> {
    if groups.is_empty() {
        return Ok(());
    }

    if groups.len() > 1 {
        flat_groups(groups);
    }
    let main_group: &mut Vec<GQLObject> = groups[0].as_mut();
    main_group.drain(0..cmp::min(statement.count, main_group.len()));

    return Ok(());
}

fn execute_order_by_statement(
    statement: &OrderByStatement,
    groups: &mut Vec<Vec<GQLObject>>,
) -> Result<(), String> {
    if groups.is_empty() {
        return Ok(());
    }

    if groups.len() > 1 {
        flat_groups(groups);
    }

    let main_group: &mut Vec<GQLObject> = groups[0].as_mut();
    if main_group.is_empty() {
        return Ok(());
    }

    if main_group[0].attributes.contains_key(&statement.field_name) {
        if statement.field_type == DataType::Number {
            main_group.sort_by(|a, b| {
                let first_value = a
                    .attributes
                    .get(&statement.field_name.to_string())
                    .unwrap()
                    .as_number();

                let other = b
                    .attributes
                    .get(&statement.field_name.to_string())
                    .unwrap()
                    .as_number();

                first_value.partial_cmp(&other).unwrap()
            });
        } else {
            main_group.sort_by_cached_key(|object| {
                object
                    .attributes
                    .get(&statement.field_name.to_string())
                    .unwrap()
                    .as_text()
            });
        }

        if !statement.is_ascending {
            main_group.reverse();
        }
    }

    return Ok(());
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
        if groups_map.contains_key(&field_value.as_text()) {
            let index = *groups_map.get(&field_value.as_text()).unwrap();
            let target_group = &mut groups[index];
            target_group.push(object.to_owned());
        }
        // Push a new group for this unique value and update the next index
        else {
            groups_map.insert(field_value.as_text(), next_group_index);
            next_group_index += 1;
            groups.push(vec![object.to_owned()]);
        }
    }

    return Ok(());
}

fn execute_aggregation_function_statement(
    statement: &AggregationFunctionsStatement,
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
        for aggregation in aggregations_map {
            let function = aggregation.1;

            // Get the target aggregation function
            let aggregation_function = AGGREGATIONS.get(function.function_name.as_str()).unwrap();

            // Execute aggregation function once for group
            let result_column_name = aggregation.0;
            let argument = &function.argument;
            let result = &aggregation_function(&argument.to_string(), &group);

            // Get alias name if exists or column name by default
            let column_name = get_column_name(alias_table, result_column_name);

            // Insert the calculated value in the group objects
            for object in group.into_iter() {
                object
                    .attributes
                    .insert(column_name.to_string(), result.to_owned());
            }
        }

        // In case of group by statement is exectued
        // Remove all elements expect the first one
        if groups_count > 1 {
            group.drain(1..);
        }
    }

    return Ok(());
}

fn flat_groups(groups: &mut Vec<Vec<GQLObject>>) {
    let mut main_group: Vec<GQLObject> = Vec::new();
    for group in groups.into_iter() {
        main_group.append(group);
    }

    groups.clear();
    groups.push(main_group);
}
