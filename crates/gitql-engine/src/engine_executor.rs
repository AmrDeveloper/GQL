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
use crate::engine_function::select_gql_objects;

pub fn execute_statement(
    statement: &Box<dyn Statement>,
    repo: &git2::Repository,
    groups: &mut Vec<Vec<GQLObject>>,
) {
    match statement.get_statement_kind() {
        Select => {
            let statement = statement
                .as_any()
                .downcast_ref::<SelectStatement>()
                .unwrap();
            execute_select_statement(statement, repo, groups);
        }
        Where => {
            let statement = statement.as_any().downcast_ref::<WhereStatement>().unwrap();
            execute_where_statement(statement, groups);
        }
        Having => {
            let statement = statement
                .as_any()
                .downcast_ref::<HavingStatement>()
                .unwrap();
            execute_having_statement(statement, groups);
        }
        Limit => {
            let statement = statement.as_any().downcast_ref::<LimitStatement>().unwrap();
            execute_limit_statement(statement, groups);
        }
        Offset => {
            let statement = statement
                .as_any()
                .downcast_ref::<OffsetStatement>()
                .unwrap();
            execute_offset_statement(statement, groups);
        }
        OrderBy => {
            let statement = statement
                .as_any()
                .downcast_ref::<OrderByStatement>()
                .unwrap();
            execute_order_by_statement(statement, groups);
        }
        GroupBy => {
            let statement = statement
                .as_any()
                .downcast_ref::<GroupByStatement>()
                .unwrap();
            execute_group_by_statement(statement, groups);
        }
        AggregateFunction => {
            let statement = statement
                .as_any()
                .downcast_ref::<AggregationFunctionsStatement>()
                .unwrap();
            execute_aggregation_function_statement(statement, groups);
        }
    }
}

fn execute_select_statement(
    statement: &SelectStatement,
    repo: &git2::Repository,
    groups: &mut Vec<Vec<GQLObject>>,
) {
    // Select obects from the target table
    let mut objects = select_gql_objects(
        repo,
        statement.table_name.to_string(),
        statement.fields.to_owned(),
        statement.alias_table.to_owned(),
    );

    // Push the selected elements as a first group
    if groups.is_empty() {
        groups.push(objects);
    } else {
        groups[0].append(&mut objects);
    }
}

fn execute_where_statement(statement: &WhereStatement, groups: &mut Vec<Vec<GQLObject>>) {
    if groups.is_empty() {
        return;
    }
    // Perform where command only on the first group
    // because group by command not executed yet
    let filtered_group: Vec<GQLObject> = groups
        .first()
        .unwrap()
        .iter()
        .filter(|&object| evaluate_expression(&statement.condition, object).eq("true"))
        .cloned()
        .collect();

    // Update the main group with the filtered data
    groups.remove(0);
    groups.push(filtered_group);
}

fn execute_having_statement(statement: &HavingStatement, groups: &mut Vec<Vec<GQLObject>>) {
    if groups.is_empty() {
        return;
    }

    if groups.len() > 1 {
        flat_groups(groups);
    }

    let main_group: &mut Vec<GQLObject> = groups[0].as_mut();

    let result: Vec<GQLObject> = main_group
        .iter()
        .filter(|&object| evaluate_expression(&statement.condition, object).eq("true"))
        .cloned()
        .collect();

    main_group.clear();

    for object in result {
        main_group.push(object);
    }
}

fn execute_limit_statement(statement: &LimitStatement, groups: &mut Vec<Vec<GQLObject>>) {
    if groups.is_empty() {
        return;
    }

    if groups.len() > 1 {
        flat_groups(groups);
    }

    let main_group: &mut Vec<GQLObject> = groups[0].as_mut();
    if statement.count <= main_group.len() {
        main_group.drain(statement.count..main_group.len());
    }
}

fn execute_offset_statement(statement: &OffsetStatement, groups: &mut Vec<Vec<GQLObject>>) {
    if groups.is_empty() {
        return;
    }

    if groups.len() > 1 {
        flat_groups(groups);
    }

    let main_group: &mut Vec<GQLObject> = groups[0].as_mut();
    main_group.drain(0..cmp::min(statement.count, main_group.len()));
}

fn execute_order_by_statement(statement: &OrderByStatement, groups: &mut Vec<Vec<GQLObject>>) {
    if groups.is_empty() {
        return;
    }

    if groups.len() > 1 {
        flat_groups(groups);
    }

    let main_group: &mut Vec<GQLObject> = groups[0].as_mut();
    if main_group.is_empty() {
        return;
    }

    if main_group[0].attributes.contains_key(&statement.field_name) {
        if statement.field_type == DataType::Number {
            main_group.sort_by(|a, b| {
                let first_value = a
                    .attributes
                    .get(&statement.field_name.to_string())
                    .unwrap()
                    .to_string()
                    .parse::<i64>()
                    .unwrap();

                let other = b
                    .attributes
                    .get(&statement.field_name.to_string())
                    .unwrap()
                    .to_string()
                    .parse::<i64>()
                    .unwrap();
                first_value.partial_cmp(&other).unwrap()
            });
        } else {
            main_group.sort_by_cached_key(|object| {
                object
                    .attributes
                    .get(&statement.field_name.to_string())
                    .unwrap()
                    .to_string()
            });
        }

        if !statement.is_ascending {
            main_group.reverse();
        }
    }
}

fn execute_group_by_statement(statement: &GroupByStatement, groups: &mut Vec<Vec<GQLObject>>) {
    if groups.is_empty() {
        return;
    }

    let main_group: Vec<GQLObject> = groups.remove(0);
    if main_group.is_empty() {
        return;
    }

    // Mapping each unique value to it group index
    let mut groups_map: HashMap<String, usize> = HashMap::new();

    // Track current group index
    let mut next_group_index = 0;

    for object in main_group.into_iter() {
        let field_value = object.attributes.get(&statement.field_name).unwrap();

        // If there is an existing group for this value, append current object to it
        if groups_map.contains_key(field_value) {
            let index = *groups_map.get(field_value).unwrap();
            let target_group = &mut groups[index];
            target_group.push(object.to_owned());
        }
        // Push a new group for this unique value and update the next index
        else {
            groups_map.insert(field_value.to_string(), next_group_index);
            next_group_index += 1;
            groups.push(vec![object.to_owned()]);
        }
    }
}

fn execute_aggregation_function_statement(
    statement: &AggregationFunctionsStatement,
    groups: &mut Vec<Vec<GQLObject>>,
) {
    // Make sure you have at least one aggregation function to calculate
    let aggregations_map = &statement.aggregations;
    if aggregations_map.is_empty() {
        return;
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
            let result = &aggregation_function(&function.argument, &group);

            // Insert the calculated value in the group objects
            for object in group.into_iter() {
                object
                    .attributes
                    .insert(result_column_name.to_string(), result.to_string());
            }
        }

        // In case of group by statement is exectued
        // Remove all elements expect the first one
        if groups_count > 1 {
            group.drain(1..);
        }
    }
}

fn flat_groups(groups: &mut Vec<Vec<GQLObject>>) {
    let mut main_group: Vec<GQLObject> = Vec::new();
    for group in groups.into_iter() {
        main_group.append(group);
    }

    groups.clear();
    groups.push(main_group);
}
