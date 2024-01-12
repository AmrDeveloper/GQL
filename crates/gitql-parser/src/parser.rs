use gitql_ast::environment::Environment;
use gitql_ast::environment::TABLES_FIELDS_NAMES;
use gitql_ast::value::Value;
use std::collections::HashMap;
use std::num::IntErrorKind;
use std::num::ParseIntError;
use std::vec;

use crate::context::ParserContext;
use crate::diagnostic::Diagnostic;
use crate::tokenizer::Location;
use crate::tokenizer::Token;
use crate::tokenizer::TokenKind;
use crate::type_checker::are_types_equals;
use crate::type_checker::check_all_values_are_same_type;
use crate::type_checker::is_expression_type_equals;
use crate::type_checker::TypeCheckResult;

use gitql_ast::aggregation::AGGREGATIONS;
use gitql_ast::aggregation::AGGREGATIONS_PROTOS;
use gitql_ast::expression::*;
use gitql_ast::function::FUNCTIONS;
use gitql_ast::function::PROTOTYPES;
use gitql_ast::statement::*;
use gitql_ast::types::DataType;
use gitql_ast::types::TABLES_FIELDS_TYPES;

pub fn parse_gql(tokens: Vec<Token>, env: &mut Environment) -> Result<Query, Box<Diagnostic>> {
    let mut position = 0;
    let first_token = &tokens[position];
    let query_result = match &first_token.kind {
        TokenKind::Set => parse_set_query(env, &tokens, &mut position),
        TokenKind::Select => parse_select_query(env, &tokens, &mut position),
        _ => Err(un_expected_statement_error(&tokens, &mut position)),
    };

    // Consume optional `;` at the end of valid statement
    if let Some(last_token) = tokens.get(position) {
        if last_token.kind == TokenKind::Semicolon {
            position += 1;
        }
    }

    // Check for un expected content after valid statement
    if query_result.is_ok() && position < tokens.len() {
        return Err(un_expected_content_after_correct_statement(
            &first_token.literal,
            &tokens,
            &mut position,
        ));
    }

    query_result
}

fn parse_set_query(
    env: &mut Environment,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Query, Box<Diagnostic>> {
    let len = tokens.len();
    let mut context = ParserContext::default();

    // Consume Set keyword
    *position += 1;

    if *position >= len || tokens[*position].kind != TokenKind::GlobalVariable {
        return Err(Diagnostic::error(
            "Expect Global variable name start with `@` after `SET` keyword",
        )
        .with_location(get_safe_location(tokens, *position - 1))
        .as_boxed());
    }

    let name = &tokens[*position].literal;

    // Consume variable name
    *position += 1;

    if *position >= len || !is_assignment_operator(&tokens[*position]) {
        return Err(
            Diagnostic::error("Expect `=` or `:=` and Value after Variable name")
                .with_location(get_safe_location(tokens, *position - 1))
                .as_boxed(),
        );
    }

    // Consume `=` or `:=` token
    *position += 1;

    let aggregations_count_before = context.aggregations.len();
    let value = parse_expression(&mut context, env, tokens, position)?;
    let has_aggregations = context.aggregations.len() != aggregations_count_before;

    // Until supports sub queries, aggregation value can't be stored in variables
    if has_aggregations {
        return Err(
            Diagnostic::error("Aggregation value can't be assigned to global variable")
                .with_location(get_safe_location(tokens, *position - 1))
                .as_boxed(),
        );
    }

    env.define_global(name.to_string(), value.expr_type(env));

    Ok(Query::GlobalVariableDeclaration(GlobalVariableStatement {
        name: name.to_string(),
        value,
    }))
}

fn parse_select_query(
    env: &mut Environment,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Query, Box<Diagnostic>> {
    let len = tokens.len();

    let mut context = ParserContext::default();
    let mut statements: HashMap<&'static str, Box<dyn Statement>> = HashMap::new();

    while *position < len {
        let token = &tokens[*position];

        match &token.kind {
            TokenKind::Select => {
                if statements.contains_key("select") {
                    return Err(Diagnostic::error("You already used `SELECT` statement")
                        .add_note("Can't use more than one `SELECT` statement in the same query")
                        .with_location(token.location)
                        .as_boxed());
                }
                let statement = parse_select_statement(&mut context, env, tokens, position)?;
                statements.insert("select", statement);
                context.is_single_value_query = !context.aggregations.is_empty();
            }
            TokenKind::Where => {
                if statements.contains_key("where") {
                    return Err(Diagnostic::error("You already used `WHERE` statement")
                        .add_note("Can't use more than one `WHERE` statement in the same query")
                        .with_location(token.location)
                        .as_boxed());
                }

                let statement = parse_where_statement(&mut context, env, tokens, position)?;
                statements.insert("where", statement);
            }
            TokenKind::Group => {
                if statements.contains_key("group") {
                    return Err(Diagnostic::error("`You already used `GROUP BY` statement")
                        .add_note("Can't use more than one `GROUP BY` statement in the same query")
                        .with_location(token.location)
                        .as_boxed());
                }

                let statement = parse_group_by_statement(&mut context, env, tokens, position)?;
                statements.insert("group", statement);
            }
            TokenKind::Having => {
                if statements.contains_key("having") {
                    return Err(Diagnostic::error("You already used `HAVING` statement")
                        .add_note("Can't use more than one `HAVING` statement in the same query")
                        .with_location(token.location)
                        .as_boxed());
                }

                if !statements.contains_key("group") {
                    return Err(Diagnostic::error(
                        "`HAVING` must be used after `GROUP BY` statement",
                    )
                    .add_note(
                        "`HAVING` statement must be used in a query that has `GROUP BY` statement",
                    )
                    .with_location(token.location)
                    .as_boxed());
                }

                let statement = parse_having_statement(&mut context, env, tokens, position)?;
                statements.insert("having", statement);
            }
            TokenKind::Limit => {
                if statements.contains_key("limit") {
                    return Err(Diagnostic::error("You already used `LIMIT` statement")
                        .add_note("Can't use more than one `LIMIT` statement in the same query")
                        .with_location(token.location)
                        .as_boxed());
                }

                let statement = parse_limit_statement(tokens, position)?;
                statements.insert("limit", statement);

                // Check for Limit and Offset shortcut
                if *position < len && tokens[*position].kind == TokenKind::Comma {
                    // Prevent user from using offset statement more than one time
                    if statements.contains_key("offset") {
                        return Err(Diagnostic::error("You already used `OFFSET` statement")
                            .add_note(
                                "Can't use more than one `OFFSET` statement in the same query",
                            )
                            .with_location(token.location)
                            .as_boxed());
                    }

                    // Consume Comma
                    *position += 1;

                    if *position >= len || tokens[*position].kind != TokenKind::Integer {
                        return Err(Diagnostic::error(
                            "Expects `OFFSET` amount as Integer value after `,`",
                        )
                        .add_help("Try to add constant Integer after comma")
                        .add_note("`OFFSET` value must be a constant Integer")
                        .with_location(token.location)
                        .as_boxed());
                    }

                    let count_result: Result<usize, ParseIntError> =
                        tokens[*position].literal.parse();

                    // Report clear error for Integer parsing
                    if let Err(error) = &count_result {
                        if error.kind().eq(&IntErrorKind::PosOverflow) {
                            return Err(Diagnostic::error("`OFFSET` integer value is too large")
                                .add_help("Try to use smaller value")
                                .add_note(&format!(
                                    "`OFFSET` value must be between 0 and {}",
                                    usize::MAX
                                ))
                                .with_location(token.location)
                                .as_boxed());
                        }

                        return Err(Diagnostic::error("`OFFSET` integer value is invalid")
                            .add_help(&format!(
                                "`OFFSET` value must be between 0 and {}",
                                usize::MAX
                            ))
                            .with_location(token.location)
                            .as_boxed());
                    }

                    // Consume Offset value
                    *position += 1;

                    let count = count_result.unwrap();
                    statements.insert("offset", Box::new(OffsetStatement { count }));
                }
            }
            TokenKind::Offset => {
                if statements.contains_key("offset") {
                    return Err(Diagnostic::error("You already used `OFFSET` statement")
                        .add_note("Can't use more than one `OFFSET` statement in the same query")
                        .with_location(token.location)
                        .as_boxed());
                }

                let statement = parse_offset_statement(tokens, position)?;
                statements.insert("offset", statement);
            }
            TokenKind::Order => {
                if statements.contains_key("order") {
                    return Err(Diagnostic::error("You already used `ORDER BY` statement")
                        .add_note("Can't use more than one `ORDER BY` statement in the same query")
                        .with_location(token.location)
                        .as_boxed());
                }

                let statement = parse_order_by_statement(&mut context, env, tokens, position)?;
                statements.insert("order", statement);
            }
            _ => break,
        }
    }

    // If any aggregation function is used, add Aggregation Functions Node to the GQL Query
    if !context.aggregations.is_empty() {
        let aggregation_functions = AggregationsStatement {
            aggregations: context.aggregations,
        };
        statements.insert("aggregation", Box::new(aggregation_functions));
    }

    // Remove all selected fields from hidden selection
    let hidden_selections: Vec<String> = context
        .hidden_selections
        .iter()
        .filter(|n| !context.selected_fields.contains(n))
        .cloned()
        .collect();

    Ok(Query::Select(GQLQuery {
        statements,
        has_aggregation_function: context.is_single_value_query,
        has_group_by_statement: context.has_group_by_statement,
        hidden_selections,
    }))
}

fn parse_select_statement(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Statement>, Box<Diagnostic>> {
    // Consume select keyword
    *position += 1;

    if *position >= tokens.len() {
        return Err(Diagnostic::error("Incomplete input for select statement")
            .add_help("Try select one or more values in the `SELECT` statement")
            .add_note("Select statements requires at least selecting one value")
            .with_location(get_safe_location(tokens, *position - 1))
            .as_boxed());
    }

    let mut table_name = "";
    let mut fields_names: Vec<String> = Vec::new();
    let mut fields_values: Vec<Box<dyn Expression>> = Vec::new();
    let mut alias_table: HashMap<String, String> = HashMap::new();
    let mut is_select_all = false;
    let mut is_distinct = false;

    // Check if select has distinct keyword after it
    if tokens[*position].kind == TokenKind::Distinct {
        is_distinct = true;
        *position += 1;
    }

    // Select all option
    if *position < tokens.len() && tokens[*position].kind == TokenKind::Star {
        // Consume `*`
        *position += 1;
        is_select_all = true;
    } else {
        while *position < tokens.len() && tokens[*position].kind != TokenKind::From {
            let expression = parse_expression(context, env, tokens, position)?;
            let expr_type = expression.expr_type(env).clone();
            let expression_name = get_expression_name(&expression);
            let field_name = if expression_name.is_ok() {
                expression_name.ok().unwrap()
            } else {
                context.generate_column_name()
            };

            // Assert that each selected field is unique
            if fields_names.contains(&field_name) {
                return Err(Diagnostic::error("Can't select the same field twice")
                    .with_location(get_safe_location(tokens, *position - 1))
                    .as_boxed());
            }

            // Check for Field name alias
            if *position < tokens.len() && tokens[*position].kind == TokenKind::As {
                // Consume `as` keyword
                *position += 1;
                let alias_name_token = consume_kind(tokens, *position, TokenKind::Symbol);
                if alias_name_token.is_err() {
                    return Err(Diagnostic::error("Expect `identifier` as field alias name")
                        .with_location(get_safe_location(tokens, *position))
                        .as_boxed());
                }

                // Register alias name
                let alias_name = alias_name_token.ok().unwrap().literal.to_string();
                if context.selected_fields.contains(&alias_name)
                    || alias_table.contains_key(&alias_name)
                {
                    return Err(
                        Diagnostic::error("You already have field with the same name")
                            .add_help("Try to use a new unique name for alias")
                            .with_location(get_safe_location(tokens, *position))
                            .as_boxed(),
                    );
                }

                // Consume alias name
                *position += 1;

                // Register alias name type
                env.define(alias_name.to_string(), expr_type.clone());

                context.selected_fields.push(alias_name.clone());
                alias_table.insert(field_name.to_string(), alias_name);
            }

            // Register field type
            env.define(field_name.to_string(), expr_type);

            fields_names.push(field_name.to_owned());
            context.selected_fields.push(field_name.to_owned());
            fields_values.push(expression);

            // Consume `,` or break
            if *position < tokens.len() && tokens[*position].kind == TokenKind::Comma {
                *position += 1;
            } else {
                break;
            }
        }
    }

    // Parse optional Form statement
    if *position < tokens.len() && tokens[*position].kind == TokenKind::From {
        // Consume `from` keyword
        *position += 1;

        let table_name_token = consume_kind(tokens, *position, TokenKind::Symbol);
        if table_name_token.is_err() {
            return Err(Diagnostic::error("Expect `identifier` as a table name")
                .add_note("Table name must be an identifier")
                .with_location(get_safe_location(tokens, *position))
                .as_boxed());
        }

        // Consume table name
        *position += 1;

        table_name = &table_name_token.ok().unwrap().literal;
        if !TABLES_FIELDS_NAMES.contains_key(table_name) {
            return Err(Diagnostic::error("Unresolved table name")
                .add_help("Check the documentations to see available tables")
                .with_location(get_safe_location(tokens, *position))
                .as_boxed());
        }

        register_current_table_fields_types(table_name, env);
    }

    // Make sure `SELECT *` used with specific table
    if is_select_all && table_name.is_empty() {
        return Err(
            Diagnostic::error("Expect `FROM` and table name after `SELECT *`")
                .add_note("Select all must be used with valid table name")
                .with_location(get_safe_location(tokens, *position))
                .as_boxed(),
        );
    }

    // Select input validations
    if !is_select_all && fields_names.is_empty() {
        return Err(Diagnostic::error("Incomplete input for select statement")
            .add_help("Try select one or more values in the `SELECT` statement")
            .add_note("Select statements requires at least selecting one value")
            .with_location(get_safe_location(tokens, *position - 1))
            .as_boxed());
    }

    // If it `select *` make all table fields selectable
    if is_select_all {
        select_all_table_fields(
            table_name,
            &mut context.selected_fields,
            &mut fields_names,
            &mut fields_values,
        );
    }

    // Type check all selected fields has type registered in type table
    type_check_selected_fields(env, table_name, &fields_names, tokens, *position)?;

    Ok(Box::new(SelectStatement {
        table_name: table_name.to_string(),
        fields_names,
        fields_values,
        alias_table,
        is_distinct,
    }))
}

fn parse_where_statement(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Statement>, Box<Diagnostic>> {
    *position += 1;
    if *position >= tokens.len() {
        return Err(Diagnostic::error("Expect expression after `WHERE` keyword")
            .add_help("Try to add boolean expression after `WHERE` keyword")
            .add_note("`WHERE` statement expects expression as condition")
            .with_location(get_safe_location(tokens, *position - 1))
            .as_boxed());
    }

    let aggregations_count_before = context.aggregations.len();

    // Make sure WHERE condition expression has boolean type
    let condition_location = tokens[*position].location;
    let condition = parse_expression(context, env, tokens, position)?;
    let condition_type = condition.expr_type(env);
    if condition_type != DataType::Boolean {
        return Err(Diagnostic::error(&format!(
            "Expect `WHERE` condition to be type {} but got {}",
            DataType::Boolean,
            condition_type
        ))
        .add_note("`WHERE` statement condition must be Boolean")
        .with_location(condition_location)
        .as_boxed());
    }

    let aggregations_count_after = context.aggregations.len();
    if aggregations_count_before != aggregations_count_after {
        return Err(
            Diagnostic::error("Can't use Aggregation functions in `WHERE` statement")
                .add_note("Aggregation functions must be used after `GROUP BY` statement")
                .add_note("Aggregation functions evaluated after later after `GROUP BY` statement")
                .with_location(condition_location)
                .as_boxed(),
        );
    }

    Ok(Box::new(WhereStatement { condition }))
}

fn parse_group_by_statement(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Statement>, Box<Diagnostic>> {
    *position += 1;
    if *position >= tokens.len() || tokens[*position].kind != TokenKind::By {
        return Err(
            Diagnostic::error("Expect keyword `by` after keyword `group`")
                .add_help("Try to use `BY` keyword after `GROUP")
                .with_location(get_safe_location(tokens, *position - 1))
                .as_boxed(),
        );
    }
    *position += 1;
    if *position >= tokens.len() || tokens[*position].kind != TokenKind::Symbol {
        return Err(Diagnostic::error("Expect field name after `group by`")
            .with_location(get_safe_location(tokens, *position - 1))
            .as_boxed());
    }

    let field_name = tokens[*position].literal.to_string();
    *position += 1;

    if !env.contains(&field_name) {
        return Err(
            Diagnostic::error("Current table not contains field with this name")
                .add_help("Check the documentations to see available fields for each tables")
                .with_location(get_safe_location(tokens, *position - 1))
                .as_boxed(),
        );
    }

    context.has_group_by_statement = true;
    Ok(Box::new(GroupByStatement { field_name }))
}

fn parse_having_statement(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Statement>, Box<Diagnostic>> {
    *position += 1;
    if *position >= tokens.len() {
        return Err(
            Diagnostic::error("Expect expression after `HAVING` keyword")
                .add_help("Try to add boolean expression after `HAVING` keyword")
                .add_note("`HAVING` statement expects expression as condition")
                .with_location(get_safe_location(tokens, *position - 1))
                .as_boxed(),
        );
    }

    // Make sure HAVING condition expression has boolean type
    let condition_location = tokens[*position].location;
    let condition = parse_expression(context, env, tokens, position)?;
    let condition_type = condition.expr_type(env);
    if condition_type != DataType::Boolean {
        return Err(Diagnostic::error(&format!(
            "Expect `HAVING` condition to be type {} but got {}",
            DataType::Boolean,
            condition_type
        ))
        .add_note("`HAVING` statement condition must be Boolean")
        .with_location(condition_location)
        .as_boxed());
    }

    Ok(Box::new(HavingStatement { condition }))
}

fn parse_limit_statement(
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Statement>, Box<Diagnostic>> {
    *position += 1;
    if *position >= tokens.len() || tokens[*position].kind != TokenKind::Integer {
        return Err(Diagnostic::error("Expect number after `LIMIT` keyword")
            .with_location(get_safe_location(tokens, *position - 1))
            .as_boxed());
    }

    let count_result: Result<usize, ParseIntError> = tokens[*position].literal.parse();

    // Report clear error for Integer parsing
    if let Err(error) = &count_result {
        if error.kind().eq(&IntErrorKind::PosOverflow) {
            return Err(Diagnostic::error("`LIMIT` integer value is too large")
                .add_help("Try to use smaller value")
                .add_note(&format!(
                    "`LIMIT` value must be between 0 and {}",
                    usize::MAX
                ))
                .with_location(get_safe_location(tokens, *position))
                .as_boxed());
        }

        return Err(Diagnostic::error("`LIMIT` integer value is invalid")
            .add_help(&format!(
                "`LIMIT` value must be between 0 and {}",
                usize::MAX
            ))
            .with_location(get_safe_location(tokens, *position))
            .as_boxed());
    }

    // Consume Integer value
    *position += 1;

    let count = count_result.unwrap();
    Ok(Box::new(LimitStatement { count }))
}

fn parse_offset_statement(
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Statement>, Box<Diagnostic>> {
    *position += 1;
    if *position >= tokens.len() || tokens[*position].kind != TokenKind::Integer {
        return Err(Diagnostic::error("Expect number after `OFFSET` keyword")
            .with_location(get_safe_location(tokens, *position - 1))
            .as_boxed());
    }

    let count_result: Result<usize, ParseIntError> = tokens[*position].literal.parse();

    // Report clear error for Integer parsing
    if let Err(error) = &count_result {
        if error.kind().eq(&IntErrorKind::PosOverflow) {
            return Err(Diagnostic::error("`OFFSET` integer value is too large")
                .add_help("Try to use smaller value")
                .add_note(&format!(
                    "`OFFSET` value must be between 0 and {}",
                    usize::MAX
                ))
                .with_location(get_safe_location(tokens, *position))
                .as_boxed());
        }

        return Err(Diagnostic::error("`OFFSET` integer value is invalid")
            .add_help(&format!(
                "`OFFSET` value must be between 0 and {}",
                usize::MAX
            ))
            .with_location(get_safe_location(tokens, *position))
            .as_boxed());
    }

    *position += 1;

    let count = count_result.unwrap();
    Ok(Box::new(OffsetStatement { count }))
}

fn parse_order_by_statement(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Statement>, Box<Diagnostic>> {
    // Consume `ORDER` keyword
    *position += 1;

    if *position >= tokens.len() || tokens[*position].kind != TokenKind::By {
        return Err(
            Diagnostic::error("Expect keyword `BY` after keyword `ORDER")
                .add_help("Try to use `BY` keyword after `ORDER")
                .with_location(get_safe_location(tokens, *position - 1))
                .as_boxed(),
        );
    }

    // Consume `BY` keyword
    *position += 1;

    let mut arguments: Vec<Box<dyn Expression>> = vec![];
    let mut sorting_orders: Vec<SortingOrder> = vec![];

    loop {
        let argument = parse_expression(context, env, tokens, position)?;
        arguments.push(argument);

        let mut order = SortingOrder::Ascending;
        if *position < tokens.len() && is_asc_or_desc(&tokens[*position]) {
            if tokens[*position].kind == TokenKind::Descending {
                order = SortingOrder::Descending;
            }

            // Consume `ASC or DESC` keyword
            *position += 1;
        }

        sorting_orders.push(order);
        if *position < tokens.len() && tokens[*position].kind == TokenKind::Comma {
            // Consume `,` keyword
            *position += 1;
        } else {
            break;
        }
    }

    Ok(Box::new(OrderByStatement {
        arguments,
        sorting_orders,
    }))
}

fn parse_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, Box<Diagnostic>> {
    let aggregations_count_before = context.aggregations.len();
    let expression = parse_assignment_expression(context, env, tokens, position)?;
    let has_aggregations = context.aggregations.len() != aggregations_count_before;

    if has_aggregations {
        let column_name = context.generate_column_name();
        env.define(column_name.to_string(), expression.expr_type(env));

        // Register the new aggregation generated field if the this expression is after group by
        if context.has_group_by_statement && !context.hidden_selections.contains(&column_name) {
            context.hidden_selections.push(column_name.to_string());
        }

        context
            .aggregations
            .insert(column_name.clone(), AggregateValue::Expression(expression));

        return Ok(Box::new(SymbolExpression { value: column_name }));
    }

    Ok(expression)
}

fn parse_assignment_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, Box<Diagnostic>> {
    let expression = parse_is_null_expression(context, env, tokens, position)?;
    if *position < tokens.len() && tokens[*position].kind == TokenKind::ColonEqual {
        if expression.kind() != ExpressionKind::GlobalVariable {
            return Err(Diagnostic::error(
                "Assignment expressions expect global variable name before `:=`",
            )
            .with_location(tokens[*position].location)
            .as_boxed());
        }

        let expr = expression
            .as_any()
            .downcast_ref::<GlobalVariableExpression>()
            .unwrap();

        let variable_name = expr.name.to_string();

        // Consume `:=` operator
        *position += 1;

        let value = parse_is_null_expression(context, env, tokens, position)?;
        env.define_global(variable_name.clone(), value.expr_type(env));

        return Ok(Box::new(AssignmentExpression {
            symbol: variable_name.clone(),
            value,
        }));
    }
    Ok(expression)
}

fn parse_is_null_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, Box<Diagnostic>> {
    let expression = parse_in_expression(context, env, tokens, position)?;
    if *position < tokens.len() && tokens[*position].kind == TokenKind::Is {
        let is_location = tokens[*position].location;

        // Consume `IS` keyword
        *position += 1;

        let has_not_keyword =
            if *position < tokens.len() && tokens[*position].kind == TokenKind::Not {
                // Consume `NOT` keyword
                *position += 1;
                true
            } else {
                false
            };

        if *position < tokens.len() && tokens[*position].kind == TokenKind::Null {
            // Consume `Null` keyword
            *position += 1;

            return Ok(Box::new(IsNullExpression {
                argument: expression,
                has_not: has_not_keyword,
            }));
        }

        return Err(
            Diagnostic::error("Expects `NULL` Keyword after `IS` or `IS NOT`")
                .with_location(is_location)
                .as_boxed(),
        );
    }
    Ok(expression)
}

fn parse_in_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, Box<Diagnostic>> {
    let expression = parse_between_expression(context, env, tokens, position)?;

    // Consume `NOT` keyword if IN Expression prefixed with `NOT` for example `expr NOT IN (...values)`
    let has_not_keyword = if *position < tokens.len() && tokens[*position].kind == TokenKind::Not {
        *position += 1;
        true
    } else {
        false
    };

    if *position < tokens.len() && tokens[*position].kind == TokenKind::In {
        let in_location = tokens[*position].location;

        // Consume `IN` keyword
        *position += 1;

        if consume_kind(tokens, *position, TokenKind::LeftParen).is_err() {
            return Err(
                Diagnostic::error("Expects values between `(` and `)` after `IN` keyword")
                    .with_location(in_location)
                    .as_boxed(),
            );
        }

        let values = parse_arguments_expressions(context, env, tokens, position)?;

        // Optimize the Expression if the number of values in the list is 0
        if values.is_empty() {
            return Ok(Box::new(BooleanExpression {
                is_true: has_not_keyword,
            }));
        }

        let values_type_result = check_all_values_are_same_type(env, &values);
        if values_type_result.is_none() {
            return Err(Diagnostic::error(
                "Expects values between `(` and `)` to have the same type",
            )
            .with_location(in_location)
            .as_boxed());
        }

        // Check that argument and values has the same type
        let values_type = values_type_result.unwrap();
        if values_type != DataType::Any && expression.expr_type(env) != values_type {
            return Err(Diagnostic::error(
                "Argument and Values of In Expression must have the same type",
            )
            .with_location(in_location)
            .as_boxed());
        }

        return Ok(Box::new(InExpression {
            argument: expression,
            values,
            values_type,
            has_not_keyword,
        }));
    }

    // Report error if user write `NOT` with no `IN` keyword after it
    if has_not_keyword {
        return Err(
            Diagnostic::error("Expects `IN` expression after this `NOT` keyword")
                .add_help("Try to use `IN` expression after NOT keyword")
                .add_help("Try to remove `NOT` keyword")
                .add_note("Expect to see `NOT` then `IN` keyword with a list of values")
                .with_location(get_safe_location(tokens, *position - 1))
                .as_boxed(),
        );
    }

    Ok(expression)
}

fn parse_between_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, Box<Diagnostic>> {
    let expression = parse_logical_or_expression(context, env, tokens, position)?;

    if *position < tokens.len() && tokens[*position].kind == TokenKind::Between {
        let between_location = tokens[*position].location;

        // Consume `BETWEEN` keyword
        *position += 1;

        if *position >= tokens.len() {
            return Err(
                Diagnostic::error("`BETWEEN` keyword expects two range after it")
                    .with_location(between_location)
                    .as_boxed(),
            );
        }

        let argument_type = expression.expr_type(env);
        let range_start = parse_logical_or_expression(context, env, tokens, position)?;

        if *position >= tokens.len() || tokens[*position].kind != TokenKind::DotDot {
            return Err(Diagnostic::error("Expect `..` after `BETWEEN` range start")
                .with_location(between_location)
                .as_boxed());
        }

        // Consume `..` token
        *position += 1;

        let range_end = parse_logical_or_expression(context, env, tokens, position)?;

        if argument_type != range_start.expr_type(env) || argument_type != range_end.expr_type(env)
        {
            return Err(Diagnostic::error(&format!(
                "Expect `BETWEEN` argument, range start and end to has same type but got {}, {} and {}",
                argument_type,
                range_start.expr_type(env),
                range_end.expr_type(env)
            ))
            .add_help("Try to make sure all of them has same type")
            .with_location(between_location)
            .as_boxed());
        }

        return Ok(Box::new(BetweenExpression {
            value: expression,
            range_start,
            range_end,
        }));
    }

    Ok(expression)
}

fn parse_logical_or_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, Box<Diagnostic>> {
    let expression = parse_logical_and_expression(context, env, tokens, position);
    if expression.is_err() || *position >= tokens.len() {
        return expression;
    }

    let lhs = expression.ok().unwrap();
    if tokens[*position].kind == TokenKind::LogicalOr {
        *position += 1;

        if lhs.expr_type(env) != DataType::Boolean {
            return Err(type_mismatch_error(
                tokens[*position - 2].location,
                DataType::Boolean,
                lhs.expr_type(env),
            )
            .as_boxed());
        }

        let rhs = parse_logical_and_expression(context, env, tokens, position)?;
        if rhs.expr_type(env) != DataType::Boolean {
            return Err(type_mismatch_error(
                tokens[*position].location,
                DataType::Boolean,
                lhs.expr_type(env),
            )
            .as_boxed());
        }

        return Ok(Box::new(LogicalExpression {
            left: lhs,
            operator: LogicalOperator::Or,
            right: rhs,
        }));
    }

    Ok(lhs)
}

fn parse_logical_and_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, Box<Diagnostic>> {
    let expression = parse_bitwise_or_expression(context, env, tokens, position);
    if expression.is_err() || *position >= tokens.len() {
        return expression;
    }

    let lhs = expression.ok().unwrap();
    if tokens[*position].kind == TokenKind::LogicalAnd {
        *position += 1;

        if lhs.expr_type(env) != DataType::Boolean {
            return Err(type_mismatch_error(
                tokens[*position - 2].location,
                DataType::Boolean,
                lhs.expr_type(env),
            )
            .as_boxed());
        }

        let rhs = parse_bitwise_or_expression(context, env, tokens, position)?;
        if rhs.expr_type(env) != DataType::Boolean {
            return Err(type_mismatch_error(
                tokens[*position].location,
                DataType::Boolean,
                lhs.expr_type(env),
            )
            .as_boxed());
        }

        return Ok(Box::new(LogicalExpression {
            left: lhs,
            operator: LogicalOperator::And,
            right: rhs,
        }));
    }

    Ok(lhs)
}

fn parse_bitwise_or_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, Box<Diagnostic>> {
    let expression = parse_logical_xor_expression(context, env, tokens, position);
    if expression.is_err() || *position >= tokens.len() {
        return expression;
    }

    let lhs = expression.ok().unwrap();
    if tokens[*position].kind == TokenKind::BitwiseOr {
        *position += 1;

        if lhs.expr_type(env) != DataType::Boolean {
            return Err(type_mismatch_error(
                tokens[*position - 2].location,
                DataType::Boolean,
                lhs.expr_type(env),
            )
            .as_boxed());
        }

        let rhs = parse_logical_xor_expression(context, env, tokens, position)?;
        if rhs.expr_type(env) != DataType::Boolean {
            return Err(type_mismatch_error(
                tokens[*position].location,
                DataType::Boolean,
                lhs.expr_type(env),
            )
            .as_boxed());
        }

        return Ok(Box::new(BitwiseExpression {
            left: lhs,
            operator: BitwiseOperator::Or,
            right: rhs,
        }));
    }

    Ok(lhs)
}

fn parse_logical_xor_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, Box<Diagnostic>> {
    let expression = parse_bitwise_and_expression(context, env, tokens, position);
    if expression.is_err() || *position >= tokens.len() {
        return expression;
    }

    let lhs = expression.ok().unwrap();
    if tokens[*position].kind == TokenKind::LogicalXor {
        *position += 1;

        if lhs.expr_type(env) != DataType::Boolean {
            return Err(type_mismatch_error(
                tokens[*position - 2].location,
                DataType::Boolean,
                lhs.expr_type(env),
            ));
        }

        let rhs = parse_bitwise_and_expression(context, env, tokens, position)?;
        if rhs.expr_type(env) != DataType::Boolean {
            return Err(type_mismatch_error(
                tokens[*position].location,
                DataType::Boolean,
                lhs.expr_type(env),
            ));
        }

        return Ok(Box::new(LogicalExpression {
            left: lhs,
            operator: LogicalOperator::Xor,
            right: rhs,
        }));
    }

    Ok(lhs)
}

fn parse_bitwise_and_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, Box<Diagnostic>> {
    let expression = parse_equality_expression(context, env, tokens, position);
    if expression.is_err() || *position >= tokens.len() {
        return expression;
    }

    let lhs = expression.ok().unwrap();
    if tokens[*position].kind == TokenKind::BitwiseAnd {
        *position += 1;

        if lhs.expr_type(env) != DataType::Boolean {
            return Err(type_mismatch_error(
                tokens[*position - 2].location,
                DataType::Boolean,
                lhs.expr_type(env),
            ));
        }

        let rhs = parse_equality_expression(context, env, tokens, position)?;
        if rhs.expr_type(env) != DataType::Boolean {
            return Err(type_mismatch_error(
                tokens[*position].location,
                DataType::Boolean,
                lhs.expr_type(env),
            ));
        }

        return Ok(Box::new(BitwiseExpression {
            left: lhs,
            operator: BitwiseOperator::And,
            right: rhs,
        }));
    }

    Ok(lhs)
}

fn parse_equality_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, Box<Diagnostic>> {
    let expression = parse_comparison_expression(context, env, tokens, position);
    if expression.is_err() || *position >= tokens.len() {
        return expression;
    }

    let mut lhs = expression.ok().unwrap();

    let operator = &tokens[*position];
    if operator.kind == TokenKind::Equal || operator.kind == TokenKind::BangEqual {
        *position += 1;
        let comparison_operator = if operator.kind == TokenKind::Equal {
            ComparisonOperator::Equal
        } else {
            ComparisonOperator::NotEqual
        };

        let mut rhs = parse_comparison_expression(context, env, tokens, position)?;

        match are_types_equals(env, &lhs, &rhs) {
            TypeCheckResult::Equals => {}
            TypeCheckResult::RightSideCasted(expr) => rhs = expr,
            TypeCheckResult::LeftSideCasted(expr) => lhs = expr,
            TypeCheckResult::NotEqualAndCantImplicitCast => {
                let lhs_type = lhs.expr_type(env);
                let rhs_type = rhs.expr_type(env);
                let diagnostic = Diagnostic::error(&format!(
                    "Can't compare values of different types `{}` and `{}`",
                    lhs_type, rhs_type
                ))
                .with_location(get_safe_location(tokens, *position - 2));

                // Provides help messages if use compare null to non null value
                if lhs_type.is_null() || rhs_type.is_null() {
                    return Err(diagnostic
                        .add_help("Try to use `IS NULL expr` expression")
                        .add_help("Try to use `ISNULL(expr)` function")
                        .as_boxed());
                }

                return Err(diagnostic.as_boxed());
            }
            TypeCheckResult::Error(diagnostic) => {
                return Err(diagnostic
                    .with_location(get_safe_location(tokens, *position - 2))
                    .as_boxed());
            }
        };

        return Ok(Box::new(ComparisonExpression {
            left: lhs,
            operator: comparison_operator,
            right: rhs,
        }));
    }

    Ok(lhs)
}

fn parse_comparison_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, Box<Diagnostic>> {
    let expression = parse_bitwise_shift_expression(context, env, tokens, position);
    if expression.is_err() || *position >= tokens.len() {
        return expression;
    }

    let mut lhs = expression.ok().unwrap();
    if is_comparison_operator(&tokens[*position]) {
        let operator = &tokens[*position];
        *position += 1;
        let comparison_operator = match operator.kind {
            TokenKind::Greater => ComparisonOperator::Greater,
            TokenKind::GreaterEqual => ComparisonOperator::GreaterEqual,
            TokenKind::Less => ComparisonOperator::Less,
            TokenKind::LessEqual => ComparisonOperator::LessEqual,
            _ => ComparisonOperator::NullSafeEqual,
        };

        let mut rhs = parse_bitwise_shift_expression(context, env, tokens, position)?;

        match are_types_equals(env, &lhs, &rhs) {
            TypeCheckResult::Equals => {}
            TypeCheckResult::RightSideCasted(expr) => rhs = expr,
            TypeCheckResult::LeftSideCasted(expr) => lhs = expr,
            TypeCheckResult::NotEqualAndCantImplicitCast => {
                let lhs_type = lhs.expr_type(env);
                let rhs_type = rhs.expr_type(env);
                let diagnostic = Diagnostic::error(&format!(
                    "Can't compare values of different types `{}` and `{}`",
                    lhs_type, rhs_type
                ))
                .with_location(get_safe_location(tokens, *position - 2));

                // Provides help messages if use compare null to non null value
                if lhs_type.is_null() || rhs_type.is_null() {
                    return Err(diagnostic
                        .add_help("Try to use `IS NULL expr` expression")
                        .add_help("Try to use `ISNULL(expr)` function")
                        .as_boxed());
                }

                return Err(diagnostic.as_boxed());
            }
            TypeCheckResult::Error(diagnostic) => {
                return Err(diagnostic
                    .with_location(get_safe_location(tokens, *position - 2))
                    .as_boxed());
            }
        };

        return Ok(Box::new(ComparisonExpression {
            left: lhs,
            operator: comparison_operator,
            right: rhs,
        }));
    }

    Ok(lhs)
}

fn parse_bitwise_shift_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, Box<Diagnostic>> {
    let mut lhs = parse_term_expression(context, env, tokens, position)?;

    while *position < tokens.len() && is_bitwise_shift_operator(&tokens[*position]) {
        let operator = &tokens[*position];
        *position += 1;
        let bitwise_operator = if operator.kind == TokenKind::BitwiseRightShift {
            BitwiseOperator::RightShift
        } else {
            BitwiseOperator::LeftShift
        };

        let rhs = parse_term_expression(context, env, tokens, position)?;

        // Make sure right and left hand side types are numbers
        if rhs.expr_type(env).is_int() && rhs.expr_type(env) != lhs.expr_type(env) {
            return Err(Diagnostic::error(&format!(
                "Bitwise operators require number types but got `{}` and `{}`",
                lhs.expr_type(env),
                rhs.expr_type(env)
            ))
            .with_location(get_safe_location(tokens, *position - 2))
            .as_boxed());
        }

        lhs = Box::new(BitwiseExpression {
            left: lhs,
            operator: bitwise_operator,
            right: rhs,
        });
    }

    Ok(lhs)
}

fn parse_term_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, Box<Diagnostic>> {
    let mut lhs = parse_factor_expression(context, env, tokens, position)?;

    while *position < tokens.len() && is_term_operator(&tokens[*position]) {
        let operator = &tokens[*position];
        *position += 1;
        let math_operator = if operator.kind == TokenKind::Plus {
            ArithmeticOperator::Plus
        } else {
            ArithmeticOperator::Minus
        };

        let rhs = parse_factor_expression(context, env, tokens, position)?;

        let lhs_type = lhs.expr_type(env);
        let rhs_type = rhs.expr_type(env);

        // Make sure right and left hand side types are numbers
        if lhs_type.is_number() && rhs_type.is_number() {
            lhs = Box::new(ArithmeticExpression {
                left: lhs,
                operator: math_operator,
                right: rhs,
            });

            continue;
        }

        // Report Error message that suggest to replace `+` operator by `CONCAT` function
        if math_operator == ArithmeticOperator::Plus {
            return Err(Diagnostic::error(&format!(
                "Math operators `+` both sides to be number types but got `{}` and `{}`",
                lhs_type, rhs_type
            ))
            .add_help(
                "You can use `CONCAT(Any, Any, ...Any)` function to concatenate values with different types",
            )
            .with_location(operator.location)
            .as_boxed());
        }

        return Err(Diagnostic::error(&format!(
            "Math operators require number types but got `{}` and `{}`",
            lhs_type, rhs_type
        ))
        .with_location(operator.location)
        .as_boxed());
    }

    Ok(lhs)
}

fn parse_factor_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, Box<Diagnostic>> {
    let expression = parse_like_expression(context, env, tokens, position);
    if expression.is_err() || *position >= tokens.len() {
        return expression;
    }

    let mut lhs = expression.ok().unwrap();
    while *position < tokens.len() && is_factor_operator(&tokens[*position]) {
        let operator = &tokens[*position];
        *position += 1;

        let factor_operator = match operator.kind {
            TokenKind::Star => ArithmeticOperator::Star,
            TokenKind::Slash => ArithmeticOperator::Slash,
            _ => ArithmeticOperator::Modulus,
        };

        let rhs = parse_like_expression(context, env, tokens, position)?;

        let lhs_type = lhs.expr_type(env);
        let rhs_type = rhs.expr_type(env);

        // Make sure right and left hand side types are numbers
        if lhs_type.is_number() && rhs_type.is_number() {
            lhs = Box::new(ArithmeticExpression {
                left: lhs,
                operator: factor_operator,
                right: rhs,
            });
            continue;
        }

        return Err(Diagnostic::error(&format!(
            "Math operators require number types but got `{}` and `{}`",
            lhs_type, rhs_type
        ))
        .with_location(get_safe_location(tokens, *position - 2))
        .as_boxed());
    }

    Ok(lhs)
}

fn parse_like_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, Box<Diagnostic>> {
    let expression = parse_glob_expression(context, env, tokens, position);
    if expression.is_err() || *position >= tokens.len() {
        return expression;
    }

    let lhs = expression.ok().unwrap();
    if tokens[*position].kind == TokenKind::Like {
        let location = tokens[*position].location;
        *position += 1;

        if !lhs.expr_type(env).is_text() {
            return Err(Diagnostic::error(&format!(
                "Expect `LIKE` left hand side to be `TEXT` but got {}",
                lhs.expr_type(env)
            ))
            .with_location(location)
            .as_boxed());
        }

        let pattern = parse_glob_expression(context, env, tokens, position)?;
        if !pattern.expr_type(env).is_text() {
            return Err(Diagnostic::error(&format!(
                "Expect `LIKE` right hand side to be `TEXT` but got {}",
                pattern.expr_type(env)
            ))
            .with_location(location)
            .as_boxed());
        }

        return Ok(Box::new(LikeExpression {
            input: lhs,
            pattern,
        }));
    }

    Ok(lhs)
}

fn parse_glob_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, Box<Diagnostic>> {
    let expression = parse_unary_expression(context, env, tokens, position);
    if expression.is_err() || *position >= tokens.len() {
        return expression;
    }

    let lhs = expression.ok().unwrap();
    if tokens[*position].kind == TokenKind::Glob {
        let location = tokens[*position].location;
        *position += 1;

        if !lhs.expr_type(env).is_text() {
            return Err(Diagnostic::error(&format!(
                "Expect `GLOB` left hand side to be `TEXT` but got {}",
                lhs.expr_type(env)
            ))
            .with_location(location)
            .as_boxed());
        }

        let pattern = parse_unary_expression(context, env, tokens, position)?;
        if !pattern.expr_type(env).is_text() {
            return Err(Diagnostic::error(&format!(
                "Expect `GLOB` right hand side to be `TEXT` but got {}",
                pattern.expr_type(env)
            ))
            .with_location(location)
            .as_boxed());
        }

        return Ok(Box::new(GlobExpression {
            input: lhs,
            pattern,
        }));
    }

    Ok(lhs)
}

fn parse_unary_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, Box<Diagnostic>> {
    if *position < tokens.len() && is_prefix_unary_operator(&tokens[*position]) {
        let op = if tokens[*position].kind == TokenKind::Bang {
            PrefixUnaryOperator::Bang
        } else {
            PrefixUnaryOperator::Minus
        };

        *position += 1;

        let rhs = parse_expression(context, env, tokens, position)?;
        let rhs_type = rhs.expr_type(env);
        if op == PrefixUnaryOperator::Bang && rhs_type != DataType::Boolean {
            return Err(type_mismatch_error(
                get_safe_location(tokens, *position - 1),
                DataType::Boolean,
                rhs_type,
            ));
        }

        if op == PrefixUnaryOperator::Minus && rhs_type != DataType::Integer {
            return Err(type_mismatch_error(
                get_safe_location(tokens, *position - 1),
                DataType::Integer,
                rhs_type,
            ));
        }

        return Ok(Box::new(PrefixUnary { right: rhs, op }));
    }

    parse_function_call_expression(context, env, tokens, position)
}

fn parse_function_call_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, Box<Diagnostic>> {
    let expression = parse_primary_expression(context, env, tokens, position)?;
    if *position < tokens.len() && tokens[*position].kind == TokenKind::LeftParen {
        let symbol_expression = expression.as_any().downcast_ref::<SymbolExpression>();
        let function_name_location = get_safe_location(tokens, *position);

        // Make sure function name is SymbolExpression
        if symbol_expression.is_none() {
            return Err(Diagnostic::error("Function name must be an identifier")
                .with_location(function_name_location)
                .as_boxed());
        }

        let function_name = &symbol_expression.unwrap().value;

        // Check if this function is a Standard library functions
        if FUNCTIONS.contains_key(function_name.as_str()) {
            let mut arguments = parse_arguments_expressions(context, env, tokens, position)?;
            let prototype = PROTOTYPES.get(function_name.as_str()).unwrap();
            let parameters = &prototype.parameters;
            let return_type = prototype.result.clone();

            check_function_call_arguments(
                env,
                &mut arguments,
                parameters,
                function_name.to_string(),
                function_name_location,
            )?;

            // Register function name with return type
            env.define(function_name.to_string(), return_type);

            return Ok(Box::new(CallExpression {
                function_name: function_name.to_string(),
                arguments,
                is_aggregation: false,
            }));
        }

        // Check if this function is an Aggregation functions
        if AGGREGATIONS.contains_key(function_name.as_str()) {
            let mut arguments = parse_arguments_expressions(context, env, tokens, position)?;
            let prototype = AGGREGATIONS_PROTOS.get(function_name.as_str()).unwrap();
            let parameters = &vec![prototype.parameter.clone()];
            let return_type = prototype.result.clone();

            check_function_call_arguments(
                env,
                &mut arguments,
                parameters,
                function_name.to_string(),
                function_name_location,
            )?;

            let argument_result = get_expression_name(&arguments[0]);
            if argument_result.is_err() {
                return Err(Diagnostic::error("Invalid Aggregation function argument")
                    .add_help("Try to use field name as Aggregation function argument")
                    .add_note("Aggregation function accept field name as argument")
                    .with_location(function_name_location)
                    .as_boxed());
            }

            let argument = argument_result.ok().unwrap();
            let column_name = context.generate_column_name();

            context.hidden_selections.push(column_name.to_string());

            // Register aggregation generated name with return type
            env.define(column_name.to_string(), return_type);

            context.aggregations.insert(
                column_name.clone(),
                AggregateValue::Function(function_name.to_string(), argument),
            );

            return Ok(Box::new(SymbolExpression { value: column_name }));
        }

        // Report that this function name is not standard or aggregation
        return Err(Diagnostic::error("No such function name")
            .add_help(&format!(
                "Function `{}` is not an Aggregation or Standard library function name",
                function_name,
            ))
            .with_location(function_name_location)
            .as_boxed());
    }
    Ok(expression)
}

fn parse_arguments_expressions(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Vec<Box<dyn Expression>>, Box<Diagnostic>> {
    let mut arguments: Vec<Box<dyn Expression>> = vec![];
    if consume_kind(tokens, *position, TokenKind::LeftParen).is_ok() {
        *position += 1;

        while tokens[*position].kind != TokenKind::RightParen {
            let argument = parse_expression(context, env, tokens, position)?;
            let argument_literal = get_expression_name(&argument);
            if argument_literal.is_ok() {
                let literal = argument_literal.ok().unwrap();
                context.hidden_selections.push(literal);
            }

            arguments.push(argument);

            if tokens[*position].kind == TokenKind::Comma {
                *position += 1;
            } else {
                break;
            }
        }

        if consume_kind(tokens, *position, TokenKind::RightParen).is_err() {
            return Err(
                Diagnostic::error("Expect `)` after function call arguments")
                    .add_help("Try to add ')' at the end of function call, after arguments")
                    .with_location(get_safe_location(tokens, *position))
                    .as_boxed(),
            );
        }

        *position += 1;
    }
    Ok(arguments)
}

fn parse_primary_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, Box<Diagnostic>> {
    if *position >= tokens.len() {
        return Err(un_expected_expression_error(tokens, position));
    }

    match tokens[*position].kind {
        TokenKind::String => {
            *position += 1;
            Ok(Box::new(StringExpression {
                value: tokens[*position - 1].literal.to_string(),
                value_type: StringValueType::Text,
            }))
        }
        TokenKind::Symbol => {
            *position += 1;
            let value = tokens[*position - 1].literal.to_string();
            if !context.selected_fields.contains(&value) {
                context.hidden_selections.push(value.to_string());
            }
            Ok(Box::new(SymbolExpression { value }))
        }
        TokenKind::GlobalVariable => {
            *position += 1;
            let name = tokens[*position - 1].literal.to_string();
            Ok(Box::new(GlobalVariableExpression { name }))
        }
        TokenKind::Integer => {
            *position += 1;
            let integer = tokens[*position - 1].literal.parse::<i64>().unwrap();
            let value = Value::Integer(integer);
            Ok(Box::new(NumberExpression { value }))
        }
        TokenKind::Float => {
            *position += 1;
            let float = tokens[*position - 1].literal.parse::<f64>().unwrap();
            let value = Value::Float(float);
            Ok(Box::new(NumberExpression { value }))
        }
        TokenKind::True => {
            *position += 1;
            Ok(Box::new(BooleanExpression { is_true: true }))
        }
        TokenKind::False => {
            *position += 1;
            Ok(Box::new(BooleanExpression { is_true: false }))
        }
        TokenKind::Null => {
            *position += 1;
            Ok(Box::new(NullExpression {}))
        }
        TokenKind::LeftParen => parse_group_expression(context, env, tokens, position),
        TokenKind::Case => parse_case_expression(context, env, tokens, position),
        _ => Err(un_expected_expression_error(tokens, position)),
    }
}

fn parse_group_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, Box<Diagnostic>> {
    *position += 1;
    let expression = parse_expression(context, env, tokens, position)?;
    if tokens[*position].kind != TokenKind::RightParen {
        return Err(Diagnostic::error("Expect `)` to end group expression")
            .with_location(get_safe_location(tokens, *position))
            .add_help("Try to add ')' at the end of group expression")
            .as_boxed());
    }
    *position += 1;
    Ok(expression)
}

fn parse_case_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, Box<Diagnostic>> {
    let mut conditions: Vec<Box<dyn Expression>> = vec![];
    let mut values: Vec<Box<dyn Expression>> = vec![];
    let mut default_value: Option<Box<dyn Expression>> = None;

    // Consume `case` keyword
    let case_location = tokens[*position].location;
    *position += 1;

    let mut has_else_branch = false;

    while *position < tokens.len() && tokens[*position].kind != TokenKind::End {
        // Else branch
        if tokens[*position].kind == TokenKind::Else {
            if has_else_branch {
                return Err(
                    Diagnostic::error("This case expression already has else branch")
                        .add_note("`CASE` expression can has only one `ELSE` branch")
                        .with_location(get_safe_location(tokens, *position))
                        .as_boxed(),
                );
            }

            // Consume `ELSE` keyword
            *position += 1;

            let default_value_expr = parse_expression(context, env, tokens, position)?;
            default_value = Some(default_value_expr);
            has_else_branch = true;
            continue;
        }

        // Check if current token kind is `WHEN` keyword
        let when_result = consume_kind(tokens, *position, TokenKind::When);
        if when_result.is_err() {
            return Err(Diagnostic::error("Expect `when` before case condition")
                .add_help("Try to add `WHEN` keyword before any condition")
                .with_location(get_safe_location(tokens, *position))
                .as_boxed());
        }

        // Consume `WHEN` keyword
        *position += 1;

        let condition = parse_expression(context, env, tokens, position)?;
        if condition.expr_type(env) != DataType::Boolean {
            return Err(Diagnostic::error("Case condition must be a boolean type")
                .with_location(get_safe_location(tokens, *position))
                .as_boxed());
        }

        conditions.push(condition);

        let then_result = consume_kind(tokens, *position, TokenKind::Then);
        if then_result.is_err() {
            return Err(Diagnostic::error("Expect `THEN` after case condition")
                .with_location(get_safe_location(tokens, *position))
                .as_boxed());
        }

        // Consume then keyword
        *position += 1;

        let expression = parse_expression(context, env, tokens, position)?;
        values.push(expression);
    }

    // Make sure case expression has at least else branch
    if conditions.is_empty() && !has_else_branch {
        return Err(
            Diagnostic::error("Case expression must has at least else branch")
                .with_location(get_safe_location(tokens, *position))
                .as_boxed(),
        );
    }

    // Make sure case expression end with END keyword
    if *position >= tokens.len() || tokens[*position].kind != TokenKind::End {
        return Err(Diagnostic::error("Expect `END` after case branches")
            .with_location(get_safe_location(tokens, *position))
            .as_boxed());
    }

    // Consume end
    *position += 1;

    // Make sure this case expression has else branch
    if !has_else_branch {
        return Err(Diagnostic::error("Case expression must has else branch")
            .with_location(get_safe_location(tokens, *position))
            .as_boxed());
    }

    // Assert that all values has the same type
    let values_type: DataType = values[0].expr_type(env);
    for (i, value) in values.iter().enumerate().skip(1) {
        if values_type != value.expr_type(env) {
            return Err(Diagnostic::error(&format!(
                "Case value in branch {} has different type than the last branch",
                i + 1
            ))
            .add_note("All values in `CASE` expression must has the same Type")
            .with_location(case_location)
            .as_boxed());
        }
    }

    Ok(Box::new(CaseExpression {
        conditions,
        values,
        default_value,
        values_type,
    }))
}

fn check_function_call_arguments(
    env: &mut Environment,
    arguments: &mut Vec<Box<dyn Expression>>,
    parameters: &Vec<DataType>,
    function_name: String,
    location: Location,
) -> Result<(), Box<Diagnostic>> {
    let parameters_len = parameters.len();
    let arguments_len = arguments.len();

    let mut has_optional_parameter = false;
    let mut has_varargs_parameter = false;
    if !parameters.is_empty() {
        let last_parameter = parameters.last().unwrap();
        has_optional_parameter = last_parameter.is_optional();
        has_varargs_parameter = last_parameter.is_varargs();
    }

    // Has Optional parameter type at the end
    if has_optional_parameter {
        // If function last parameter is optional make sure it at least has
        if arguments_len < parameters_len - 1 {
            return Err(Box::new(
                Diagnostic::error(&format!(
                    "Function `{}` expects at least `{}` arguments but got `{}`",
                    function_name,
                    parameters_len - 1,
                    arguments_len
                ))
                .with_location(location),
            ));
        }

        // Make sure function with optional parameter not called with too much arguments
        if arguments_len > parameters_len {
            return Err(Box::new(
                Diagnostic::error(&format!(
                    "Function `{}` expects at most `{}` arguments but got `{}`",
                    function_name, parameters_len, arguments_len
                ))
                .with_location(location),
            ));
        }
    }
    // Has Variable arguments parameter type at the end
    else if has_varargs_parameter {
        // If function last parameter is optional make sure it at least has
        if arguments_len < parameters_len - 1 {
            return Err(Box::new(
                Diagnostic::error(&format!(
                    "Function `{}` expects at least `{}` arguments but got `{}`",
                    function_name,
                    parameters_len - 1,
                    arguments_len
                ))
                .with_location(location),
            ));
        }
    }
    // No Optional or Variable arguments but has invalid number of arguments passed
    else if arguments_len != parameters_len {
        return Err(Box::new(
            Diagnostic::error(&format!(
                "Function `{}` expects `{}` arguments but got `{}`",
                function_name, parameters_len, arguments_len
            ))
            .with_location(location),
        ));
    }

    let mut last_required_parameter_index = parameters_len;
    if has_optional_parameter || has_varargs_parameter {
        last_required_parameter_index -= 1;
    }

    // Check each argument vs parameter type
    for index in 0..last_required_parameter_index {
        let parameter_type = parameters.get(index).unwrap();
        let argument = arguments.get(index).unwrap();
        match is_expression_type_equals(env, argument, parameter_type) {
            TypeCheckResult::RightSideCasted(new_expr) => {
                arguments[index] = new_expr;
            }
            TypeCheckResult::LeftSideCasted(new_expr) => {
                arguments[index] = new_expr;
            }
            TypeCheckResult::NotEqualAndCantImplicitCast => {
                let argument_type = argument.expr_type(env);
                return Err(Diagnostic::error(&format!(
                    "Function `{}` argument number {} with type `{}` don't match expected type `{}`",
                    function_name, index, argument_type, parameter_type
                ))
                .with_location(location).as_boxed());
            }
            _ => {}
        }
    }

    // Check the optional or varargs parameters if exists
    if has_optional_parameter || has_varargs_parameter {
        let last_parameter_type = parameters.get(last_required_parameter_index).unwrap();

        for index in last_required_parameter_index..arguments_len {
            let argument = arguments.get(index).unwrap();
            match is_expression_type_equals(env, argument, last_parameter_type) {
                TypeCheckResult::RightSideCasted(new_expr) => {
                    arguments[index] = new_expr;
                }
                TypeCheckResult::LeftSideCasted(new_expr) => {
                    arguments[index] = new_expr;
                }
                TypeCheckResult::NotEqualAndCantImplicitCast => {
                    let argument_type = arguments.get(index).unwrap().expr_type(env);
                    if !last_parameter_type.eq(&argument_type) {
                        return Err(Diagnostic::error(&format!(
                            "Function `{}` argument number {} with type `{}` don't match expected type `{}`",
                            function_name, index, argument_type, last_parameter_type
                        ))
                        .with_location(location).as_boxed());
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}

fn type_check_selected_fields(
    env: &mut Environment,
    table_name: &str,
    fields_names: &Vec<String>,
    tokens: &Vec<Token>,
    position: usize,
) -> Result<(), Box<Diagnostic>> {
    for field_name in fields_names {
        if let Some(data_type) = env.resolve_type(field_name) {
            if data_type.is_undefined() {
                return Err(Box::new(
                    Diagnostic::error(&format!("No field with name `{}`", field_name))
                        .with_location(get_safe_location(tokens, position)),
                ));
            }
            continue;
        }

        return Err(Diagnostic::error(&format!(
            "Table `{}` has no field with name `{}`",
            table_name, field_name
        ))
        .add_help("Check the documentations to see available fields for each tables")
        .with_location(get_safe_location(tokens, position))
        .as_boxed());
    }
    Ok(())
}

fn un_expected_statement_error(tokens: &[Token], position: &mut usize) -> Box<Diagnostic> {
    let token: &Token = &tokens[*position];
    let location = token.location;

    // Query starts with invalid statement
    if location.start == 0 {
        return Diagnostic::error("Unexpected statement")
            .add_help("Expect query to start with `SELECT` or `SET` keyword")
            .with_location(location)
            .as_boxed();
    }

    // General un expected statement error
    Diagnostic::error("Unexpected statement")
        .with_location(location)
        .as_boxed()
}

fn un_expected_expression_error(tokens: &Vec<Token>, position: &usize) -> Box<Diagnostic> {
    let location = get_safe_location(tokens, *position);

    if *position == 0 || *position >= tokens.len() {
        return Diagnostic::error("Can't complete parsing this expression")
            .with_location(location)
            .as_boxed();
    }

    let current = &tokens[*position];
    let previous = &tokens[*position - 1];

    // Make sure `ASC` and `DESC` are used in ORDER BY statement
    if current.kind == TokenKind::Ascending || current.kind == TokenKind::Descending {
        return Diagnostic::error("`ASC` and `DESC` must be used in `ORDER BY` statement")
            .with_location(location)
            .as_boxed();
    }

    // Similar to SQL just `=` is used for equality comparisons
    if previous.kind == TokenKind::Equal && current.kind == TokenKind::Equal {
        return Diagnostic::error("Unexpected `==`, Just use `=` to check equality")
            .with_location(location)
            .as_boxed();
    }

    // `< =` the user may mean to write `<=`
    if previous.kind == TokenKind::Greater && current.kind == TokenKind::Equal {
        return Diagnostic::error("Unexpected `> =`, do you mean `>=`?")
            .with_location(location)
            .as_boxed();
    }

    // `> =` the user may mean to write `>=`
    if previous.kind == TokenKind::Less && current.kind == TokenKind::Equal {
        return Diagnostic::error("Unexpected `< =`, do you mean `<=`?")
            .with_location(location)
            .as_boxed();
    }

    // `> >` the user may mean to write '>>'
    if previous.kind == TokenKind::Greater && current.kind == TokenKind::Greater {
        return Diagnostic::error("Unexpected `> >`, do you mean `>>`?")
            .with_location(location)
            .as_boxed();
    }

    // `< <` the user may mean to write `<<`
    if previous.kind == TokenKind::Less && current.kind == TokenKind::Less {
        return Diagnostic::error("Unexpected `< <`, do you mean `<<`?")
            .with_location(location)
            .as_boxed();
    }

    // `< >` the user may mean to write `<>`
    if previous.kind == TokenKind::Less && current.kind == TokenKind::Greater {
        return Diagnostic::error("Unexpected `< >`, do you mean `<>`?")
            .with_location(location)
            .as_boxed();
    }

    // Default error message
    Diagnostic::error("Can't complete parsing this expression")
        .with_location(location)
        .as_boxed()
}

/// Report error message for extra content after the end of current statement
fn un_expected_content_after_correct_statement(
    statement_name: &str,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Box<Diagnostic> {
    let error_message = &format!(
        "Unexpected content after the end of `{}` statement",
        statement_name.to_uppercase()
    );

    // The range of extra content
    let location_of_extra_content = Location {
        start: tokens[*position].location.start,
        end: tokens[tokens.len() - 1].location.end,
    };

    Diagnostic::error(error_message)
        .add_help("Try to check if statement keyword is missing")
        .add_help("Try remove un expected extra content")
        .with_location(location_of_extra_content)
        .as_boxed()
}

#[allow(clippy::borrowed_box)]
fn get_expression_name(expression: &Box<dyn Expression>) -> Result<String, ()> {
    if let Some(symbol) = expression.as_any().downcast_ref::<SymbolExpression>() {
        return Ok(symbol.value.to_string());
    }

    if let Some(variable) = expression
        .as_any()
        .downcast_ref::<GlobalVariableExpression>()
    {
        return Ok(variable.name.to_string());
    }

    Err(())
}

#[inline(always)]
fn register_current_table_fields_types(table_name: &str, symbol_table: &mut Environment) {
    let table_fields_names = &TABLES_FIELDS_NAMES[table_name];
    for field_name in table_fields_names {
        let field_type = TABLES_FIELDS_TYPES[field_name].clone();
        symbol_table.define(field_name.to_string(), field_type);
    }
}

#[inline(always)]
fn select_all_table_fields(
    table_name: &str,
    selected_fields: &mut Vec<String>,
    fields_names: &mut Vec<String>,
    fields_values: &mut Vec<Box<dyn Expression>>,
) {
    if TABLES_FIELDS_NAMES.contains_key(table_name) {
        let table_fields = &TABLES_FIELDS_NAMES[table_name];

        for field in table_fields {
            if !fields_names.contains(&field.to_string()) {
                fields_names.push(field.to_string());
                selected_fields.push(field.to_string());

                let literal_expr = Box::new(SymbolExpression {
                    value: field.to_string(),
                });

                fields_values.push(literal_expr);
            }
        }
    }
}

#[inline(always)]
fn consume_kind(tokens: &Vec<Token>, position: usize, kind: TokenKind) -> Result<&Token, ()> {
    if position < tokens.len() && tokens[position].kind == kind {
        return Ok(&tokens[position]);
    }
    Err(())
}

#[inline(always)]
fn get_safe_location(tokens: &Vec<Token>, position: usize) -> Location {
    if position < tokens.len() {
        return tokens[position].location;
    }
    tokens[tokens.len() - 1].location
}

#[inline(always)]
fn is_assignment_operator(token: &Token) -> bool {
    token.kind == TokenKind::Equal || token.kind == TokenKind::ColonEqual
}

#[inline(always)]
fn is_term_operator(token: &Token) -> bool {
    token.kind == TokenKind::Plus || token.kind == TokenKind::Minus
}

#[inline(always)]
fn is_bitwise_shift_operator(token: &Token) -> bool {
    token.kind == TokenKind::BitwiseLeftShift || token.kind == TokenKind::BitwiseRightShift
}

#[inline(always)]
fn is_prefix_unary_operator(token: &Token) -> bool {
    token.kind == TokenKind::Bang || token.kind == TokenKind::Minus
}

#[inline(always)]
fn is_comparison_operator(token: &Token) -> bool {
    token.kind == TokenKind::Greater
        || token.kind == TokenKind::GreaterEqual
        || token.kind == TokenKind::Less
        || token.kind == TokenKind::LessEqual
        || token.kind == TokenKind::NullSafeEqual
}

#[inline(always)]
fn is_factor_operator(token: &Token) -> bool {
    token.kind == TokenKind::Star
        || token.kind == TokenKind::Slash
        || token.kind == TokenKind::Percentage
}

#[inline(always)]
fn is_asc_or_desc(token: &Token) -> bool {
    token.kind == TokenKind::Ascending || token.kind == TokenKind::Descending
}

#[inline(always)]
fn type_mismatch_error(
    location: Location,
    expected: DataType,
    actual: DataType,
) -> Box<Diagnostic> {
    Diagnostic::error(&format!(
        "Type mismatch expected `{}`, got `{}`",
        expected, actual
    ))
    .with_location(location)
    .as_boxed()
}
