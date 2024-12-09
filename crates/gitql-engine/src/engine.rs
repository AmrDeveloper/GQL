use std::collections::HashMap;
use std::vec;

use gitql_ast::statement::DescribeStatement;
use gitql_ast::statement::DoStatement;
use gitql_ast::statement::GQLQuery;
use gitql_ast::statement::GlobalVariableStatement;
use gitql_ast::statement::Query;
use gitql_ast::statement::SelectStatement;
use gitql_core::environment::Environment;
use gitql_core::object::GitQLObject;
use gitql_core::object::Group;
use gitql_core::object::Row;
use gitql_core::values::base::Value;
use gitql_core::values::text::TextValue;

use crate::data_provider::DataProvider;
use crate::engine_distinct::apply_distinct_operator;
use crate::engine_evaluator::evaluate_expression;
use crate::engine_executor::execute_global_variable_statement;
use crate::engine_executor::execute_statement;

/// Static Logical Plan, later must be replaced by optimized and Logical Planner
const FIXED_LOGICAL_PLAN_LEN: usize = 9;
const FIXED_LOGICAL_PLAN: [&str; FIXED_LOGICAL_PLAN_LEN] = [
    "select",
    "where",
    "group",
    "aggregation",
    "having",
    "order",
    "window_functions",
    "offset",
    "limit",
];

pub enum EvaluationResult {
    Do(Box<dyn Value>),
    SelectedGroups(GitQLObject),
    SelectedInfo,
    SetGlobalVariable,
}

#[allow(clippy::borrowed_box)]
pub fn evaluate(
    env: &mut Environment,
    data_provider: &Box<dyn DataProvider>,
    queries: Vec<Query>,
) -> Result<Vec<EvaluationResult>, String> {
    let mut evaluations_results: Vec<EvaluationResult> = vec![];
    for query in queries {
        let evaluation_result = match query {
            Query::Do(do_statement) => evaluate_do_query(env, &do_statement),
            Query::Select(gql_query) => evaluate_select_query(env, data_provider, gql_query),
            Query::GlobalVariableDeclaration(global) => {
                evaluate_global_declaration_query(env, &global)
            }
            Query::Describe(describe_statement) => evaluate_describe_query(env, describe_statement),
            Query::ShowTables => evaluate_show_tables_query(env),
        }?;
        evaluations_results.push(evaluation_result);
    }
    Ok(evaluations_results)
}

fn evaluate_do_query(
    env: &mut Environment,
    do_statement: &DoStatement,
) -> Result<EvaluationResult, String> {
    Ok(EvaluationResult::Do(evaluate_expression(
        env,
        &do_statement.expression,
        &[],
        &vec![],
    )?))
}

#[allow(clippy::borrowed_box)]
fn evaluate_select_query(
    env: &mut Environment,
    data_provider: &Box<dyn DataProvider>,
    query: GQLQuery,
) -> Result<EvaluationResult, String> {
    let mut gitql_object = GitQLObject::default();
    let mut alias_table: HashMap<String, String> = query.alias_table;

    let hidden_selections_map = query.hidden_selections;
    let hidden_selections: Vec<String> =
        hidden_selections_map.values().flatten().cloned().collect();
    let mut statements_map = query.statements;
    let has_group_by_statement = statements_map.contains_key("group");

    for logical_node_name in FIXED_LOGICAL_PLAN {
        if let Some(statement) = statements_map.get_mut(logical_node_name) {
            match logical_node_name {
                "select" => {
                    // Select statement should be performed on all repositories, can be executed in parallel
                    let select_statement = statement
                        .as_any()
                        .downcast_ref::<SelectStatement>()
                        .unwrap();

                    execute_statement(
                        env,
                        statement,
                        data_provider,
                        &mut gitql_object,
                        &mut alias_table,
                        &hidden_selections_map,
                        has_group_by_statement,
                    )?;

                    // If the main group is empty, no need to perform other statements
                    if gitql_object.is_empty() || gitql_object.groups[0].is_empty() {
                        return Ok(EvaluationResult::SelectedGroups(gitql_object));
                    }

                    // Apply the distinct operation object is not empty too.
                    apply_distinct_operator(
                        &select_statement.distinct,
                        &mut gitql_object,
                        &hidden_selections,
                    );
                }
                _ => {
                    execute_statement(
                        env,
                        statement,
                        data_provider,
                        &mut gitql_object,
                        &mut alias_table,
                        &hidden_selections_map,
                        has_group_by_statement,
                    )?;
                }
            }
        }
    }

    // Remove Hidden Selection from the rows after executing the query plan
    remove_hidden_selected_from_groups(
        &mut gitql_object.titles,
        &mut gitql_object.groups,
        &hidden_selections,
    );

    let number_of_groups = gitql_object.groups.len();
    let main_group: &mut Group = &mut gitql_object.groups[0];

    // If there are many groups that mean group by is executed before.
    // must merge each group into only one element
    if number_of_groups > 1 {
        for group in gitql_object.groups.iter_mut() {
            if group.len() > 1 {
                group.rows.drain(1..);
            }
        }
        gitql_object.flat();
    }
    // If it a single group but it select only aggregations function,
    // should return only first element in the group
    else if number_of_groups == 1
        && !query.has_group_by_statement
        && query.has_aggregation_function
        && main_group.len() > 1
    {
        main_group.rows.drain(1..);
    }

    // Into statement must be executed last after flatted and remove hidden selections
    if let Some(into_statement) = statements_map.get_mut("into") {
        execute_statement(
            env,
            into_statement,
            data_provider,
            &mut gitql_object,
            &mut alias_table,
            &hidden_selections_map,
            has_group_by_statement,
        )?;

        return Ok(EvaluationResult::SelectedInfo);
    }

    Ok(EvaluationResult::SelectedGroups(gitql_object))
}

fn evaluate_global_declaration_query(
    env: &mut Environment,
    statement: &GlobalVariableStatement,
) -> Result<EvaluationResult, String> {
    execute_global_variable_statement(env, statement)?;
    Ok(EvaluationResult::SetGlobalVariable)
}

fn evaluate_describe_query(
    env: &mut Environment,
    stmt: DescribeStatement,
) -> Result<EvaluationResult, String> {
    let table_fields = env
        .schema
        .tables_fields_names
        .get(&stmt.table_name.as_str())
        .unwrap();

    let mut gitql_object = GitQLObject::default();
    gitql_object.titles.push("Field".to_owned());
    gitql_object.titles.push("Type".to_owned());

    let mut rows: Vec<Row> = Vec::with_capacity(table_fields.len());
    for field in table_fields {
        let value = env.schema.tables_fields_types.get(field).unwrap();
        rows.push(Row {
            values: vec![
                Box::new(TextValue {
                    value: field.to_owned().to_owned(),
                }),
                Box::new(TextValue {
                    value: value.literal(),
                }),
            ],
        })
    }

    gitql_object.groups.push(Group { rows });
    Ok(EvaluationResult::SelectedGroups(gitql_object))
}

fn evaluate_show_tables_query(env: &mut Environment) -> Result<EvaluationResult, String> {
    let tables = env.schema.tables_fields_names.keys();

    let mut rows: Vec<Row> = Vec::with_capacity(tables.len());
    for table in env.schema.tables_fields_names.keys() {
        let values: Vec<Box<dyn Value>> = vec![Box::new(TextValue {
            value: table.to_owned().to_owned(),
        })];

        rows.push(Row { values });
    }

    let mut gitql_object = GitQLObject::default();
    gitql_object.titles.push("Tables".to_owned());
    gitql_object.groups.push(Group { rows });

    Ok(EvaluationResult::SelectedGroups(gitql_object))
}

fn remove_hidden_selected_from_groups(
    titles: &mut Vec<String>,
    groups: &mut [Group],
    hidden_selections: &[String],
) {
    let titles_count = titles.len();
    let mut index_list: Vec<usize> = vec![];
    for i in (0..titles_count).rev() {
        if hidden_selections.contains(&titles[i]) {
            titles.remove(i);
            index_list.push(i);
        }
    }

    for group in groups.iter_mut() {
        for index_to_delete in index_list.iter() {
            for row in group.rows.iter_mut() {
                row.values.remove(*index_to_delete);
            }
        }
    }
}
