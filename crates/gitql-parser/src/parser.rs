use std::collections::HashMap;
use std::vec;

use gitql_ast::expression::ArithmeticExpr;
use gitql_ast::expression::ArrayExpr;
use gitql_ast::expression::AssignmentExpr;
use gitql_ast::expression::BetweenExpr;
use gitql_ast::expression::BitwiseExpr;
use gitql_ast::expression::*;
use gitql_ast::operator::ArithmeticOperator;
use gitql_ast::operator::BinaryBitwiseOperator;
use gitql_ast::operator::BinaryLogicalOperator;
use gitql_ast::operator::PrefixUnaryOperator;
use gitql_ast::query::DescribeQuery;
use gitql_ast::query::DoQuery;
use gitql_ast::query::GlobalVariableDeclQuery;
use gitql_ast::query::Query;
use gitql_ast::query::SelectQuery;
use gitql_ast::statement::*;
use gitql_ast::types::any::AnyType;
use gitql_ast::types::array::ArrayType;
use gitql_ast::types::boolean::BoolType;
use gitql_ast::types::composite::CompositeType;
use gitql_ast::types::undefined::UndefType;
use gitql_ast::types::DataType;
use gitql_core::environment::Environment;

use crate::context::ParserContext;
use crate::diagnostic::Diagnostic;
use crate::name_similarity::find_closeest_string;
use crate::parse_cast::parse_cast_call_expression;
use crate::parse_cast::parse_cast_operator_expression;
use crate::parse_comparisons::parse_comparison_expression;
use crate::parse_function_call::parse_function_call_expression;
use crate::parse_function_call::parse_over_window_definition;
use crate::parse_interval::parse_interval_expression;
use crate::parse_into::parse_into_statement;
use crate::parse_ordering::parse_order_by_statement;
use crate::token::SourceLocation;
use crate::token::Token;
use crate::token::TokenKind;
use crate::type_checker::check_all_values_are_same_type;
use crate::type_checker::type_check_and_classify_selected_fields;
use crate::type_checker::type_check_projection_symbols;

pub fn parse_gql(tokens: Vec<Token>, env: &mut Environment) -> Result<Vec<Query>, Box<Diagnostic>> {
    let mut queries: Vec<Query> = vec![];
    let mut position = 0;

    while position < tokens.len() {
        env.clear_session();

        let query = match &tokens[position].kind {
            TokenKind::Do => parse_do_query(env, &tokens, &mut position),
            TokenKind::Set => parse_set_query(env, &tokens, &mut position),
            TokenKind::Select => parse_select_query(env, &tokens, &mut position),
            TokenKind::Describe => parse_describe_query(env, &tokens, &mut position),
            TokenKind::Show => parse_show_query(&tokens, &mut position),
            _ => Err(un_expected_query_start_error(&tokens, &mut position)),
        }?;

        // Consume optional `;` at the end of valid statement
        if let Some(last_token) = tokens.get(position) {
            if last_token.kind == TokenKind::Semicolon {
                position += 1;
            }
        }

        queries.push(query);
    }

    // Check for unexpected content after valid statement
    if position < tokens.len() {
        return Err(un_expected_content_after_correct_query(
            &tokens[0].to_string(),
            &tokens,
            &mut position,
        ));
    }

    Ok(queries)
}

fn parse_do_query(
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Query, Box<Diagnostic>> {
    // Consume Do keyword
    *position += 1;

    if *position >= tokens.len() {
        return Err(
            Diagnostic::error("Expect expression after Do Statement keyword")
                .with_location(calculate_safe_location(tokens, *position - 1))
                .as_boxed(),
        );
    }

    let mut context = ParserContext::default();
    let mut exprs = vec![];

    loop {
        let expression = parse_expression(&mut context, env, tokens, position)?;
        exprs.push(expression);

        if !is_current_token(tokens, position, TokenKind::Comma) {
            break;
        }

        // Consume `,`
        *position += 1;
    }

    Ok(Query::Do(DoQuery { exprs }))
}

fn parse_set_query(
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Query, Box<Diagnostic>> {
    let len = tokens.len();
    let mut context = ParserContext::default();

    // Consume Set keyword
    *position += 1;

    if !is_current_token_with_condition(tokens, position, |token| {
        matches!(token.kind, TokenKind::GlobalVariable(_))
    }) {
        return Err(Diagnostic::error(
            "Expect Global variable name start with `@` after `SET` keyword",
        )
        .with_location(calculate_safe_location(tokens, *position - 1))
        .as_boxed());
    }

    let name = &tokens[*position].to_string();

    // Consume variable name
    *position += 1;

    if *position >= len || !is_assignment_operator(&tokens[*position]) {
        return Err(
            Diagnostic::error("Expect `=` or `:=` and Value after Variable name")
                .with_location(calculate_safe_location(tokens, *position - 1))
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
                .with_location(calculate_safe_location(tokens, *position - 1))
                .as_boxed(),
        );
    }

    env.define_global(name.to_string(), value.expr_type());

    Ok(Query::GlobalVariableDecl(GlobalVariableDeclQuery {
        name: name.to_string(),
        value,
    }))
}

fn parse_describe_query(
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Query, Box<Diagnostic>> {
    // Consume `DESCRIBE` keyword
    *position += 1;

    if *position >= tokens.len() || !matches!(tokens[*position].kind, TokenKind::Symbol(_)) {
        return Err(
            Diagnostic::error("Expect table name after DESCRIBE Statement")
                .with_location(calculate_safe_location(tokens, *position))
                .as_boxed(),
        );
    }

    // Make sure table name is valid
    let table_name = tokens[*position].to_string();
    if !env
        .schema
        .tables_fields_names
        .contains_key(table_name.as_str())
    {
        let mut diagnostic =
            Diagnostic::error(&format!("Cannot find table with name `{table_name}`"))
                .add_note("You can use the `SHOW TABLES` query to get list of current tables")
                .add_note("Check the documentations to see available tables")
                .with_location(calculate_safe_location(tokens, *position));

        let canditates: Vec<&&str> = env.schema.tables_fields_names.keys().collect();
        if let Some(closest_valid_name) = find_closeest_string(&table_name, &canditates) {
            let message = &format!("a table with a similar name exists: `{closest_valid_name}`");
            diagnostic = diagnostic.add_help(message);
        }

        return Err(diagnostic.as_boxed());
    }

    // Consume Table Name
    *position += 1;

    Ok(Query::DescribeTable(DescribeQuery { table_name }))
}

fn parse_show_query(tokens: &[Token], position: &mut usize) -> Result<Query, Box<Diagnostic>> {
    // Consume SHOW keyword
    *position += 1;

    if *position >= tokens.len() || tokens[*position].to_string() != "tables" {
        return Err(
            Diagnostic::error("Show can not be followed by names other than tables")
                .add_help("A correct statement will be `SHOW TABLES`")
                .with_location(calculate_safe_location(tokens, *position - 1))
                .as_boxed(),
        );
    }

    *position += 1;
    Ok(Query::ShowTables)
}

fn parse_select_query(
    env: &mut Environment,
    tokens: &[Token],
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
                        .add_help("If you have more than one query, end each one with `;`")
                        .with_location(token.location)
                        .as_boxed());
                }
                let statement = parse_select_statement(&mut context, env, tokens, position)?;
                statements.insert("select", statement);
                context.is_single_value_query = !context.aggregations.is_empty();
                context.has_select_statement = true;
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
            TokenKind::Qualify => {
                if statements.contains_key("qualify") {
                    return Err(Diagnostic::error("You already used `QUALIFY` statement")
                        .add_note("Can't use more than one `QUALIFY` statement in the same query")
                        .with_location(token.location)
                        .as_boxed());
                }
                let statement = parse_qualify_statement(&mut context, env, tokens, position)?;
                statements.insert("qualify", statement);
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
                if is_current_token(tokens, position, TokenKind::Comma) {
                    // Prevent user from using offset statement more than one time
                    if statements.contains_key("offset") {
                        return Err(Diagnostic::error("You already used `OFFSET` statement")
                            .add_note(
                                "Can't use more than one `OFFSET` statement in the same query",
                            )
                            .with_location(token.location)
                            .as_boxed());
                    }

                    // Consume `,``
                    *position += 1;

                    if *position >= len || !matches!(tokens[*position].kind, TokenKind::Integer(_))
                    {
                        return Err(Diagnostic::error(
                            "Expects `OFFSET` amount as Integer value after `,`",
                        )
                        .add_help("Try to add constant Integer after comma")
                        .add_note("`OFFSET` value must be a constant Integer")
                        .with_location(token.location)
                        .as_boxed());
                    }

                    match tokens[*position].kind {
                        TokenKind::Integer(integer) => {
                            // Consume Offset value
                            *position += 1;

                            if integer < 0 {
                                return Err(Diagnostic::error(
                                    "Expect positive number after `OFFSET` keyword",
                                )
                                .with_location(calculate_safe_location(tokens, *position - 1))
                                .as_boxed());
                            }

                            let start = Box::new(NumberExpr {
                                value: Number::Int(integer),
                            });
                            statements.insert("offset", Box::new(OffsetStatement { start }));
                        }
                        _ => {
                            return Err(Diagnostic::error("`OFFSET` integer value is invalid")
                                .add_help(&format!(
                                    "`OFFSET` value must be between 0 and {}",
                                    usize::MAX
                                ))
                                .with_location(token.location)
                                .as_boxed())
                        }
                    }
                }
            }
            TokenKind::Offset => {
                if statements.contains_key("offset") {
                    return Err(Diagnostic::error("You already used `OFFSET` statement")
                        .add_note("Can't use more than one `OFFSET` statement in the same query")
                        .with_location(token.location)
                        .as_boxed());
                }

                let statement = parse_offset_statement(&mut context, env, tokens, position)?;
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
            TokenKind::Into => {
                if statements.contains_key("into") {
                    return Err(Diagnostic::error("You already used `INTO` statement")
                        .add_note("Can't use more than one `INTO` statement in the same query")
                        .with_location(token.location)
                        .as_boxed());
                }
                let statement = parse_into_statement(tokens, position)?;
                statements.insert("into", statement);
            }
            TokenKind::Window => {
                parse_window_named_over_clause(&mut context, env, tokens, position)?;
                continue;
            }
            _ => break,
        }
    }

    // If any aggregation function is used, add Aggregation Functions Node to the GitQL Query
    if !context.aggregations.is_empty() {
        let aggregation_functions = AggregationsStatement {
            aggregations: context.aggregations,
        };
        statements.insert("aggregation", Box::new(aggregation_functions));
    }

    // If any window function is used, add Window Functions Node to the GitQL Query
    if !context.window_functions.is_empty() {
        // TODO: Move this implementation into type checker function
        // TODO: Improve the design to get benefits of named window
        // Resolve named window clauses by their values, this is not the best option,
        // we should reorder and group window functions to reduce the name of over clauses
        for (_, window_value) in context.window_functions.iter_mut() {
            if let WindowValue::Function(function) = window_value {
                if let Some(window_name) = &function.window_definition.name {
                    if !context.named_window_clauses.contains_key(window_name) {
                        return Err(Diagnostic::error(&format!(
                            "Can't resolve `WINDOW Definition` with name {window_name}"
                        ))
                        .add_note("Make sure you already defined window over clause with this name")
                        .as_boxed());
                    }

                    function.window_definition = context.named_window_clauses[window_name].clone();
                }
            }
        }

        statements.insert(
            "window_functions",
            Box::new(WindowFunctionsStatement {
                window_values: context.window_functions,
            }),
        );
    }

    // Remove all selected fields from hidden selection
    let hidden_selections: Vec<String> = context
        .hidden_selections
        .iter()
        .filter(|n| !context.selected_fields.contains(n))
        .cloned()
        .collect();

    type_check_projection_symbols(
        env,
        &context.selected_tables,
        &context.projection_names,
        &context.projection_locations,
    )?;

    let hidden_selection_per_table =
        classify_hidden_selection(env, &context.selected_tables, &hidden_selections);

    Ok(Query::Select(SelectQuery {
        statements,
        has_aggregation_function: context.is_single_value_query,
        has_group_by_statement: context.has_group_by_statement,
        hidden_selections: hidden_selection_per_table,
        alias_table: context.name_alias_table,
    }))
}

fn classify_hidden_selection(
    env: &mut Environment,
    tables: &[String],
    hidden_selections: &[String],
) -> HashMap<String, Vec<String>> {
    let mut table_hidden_selections: HashMap<String, Vec<String>> = HashMap::new();
    for table in tables {
        table_hidden_selections.insert(table.to_string(), vec![]);
    }

    for hidden_selection in hidden_selections {
        let mut is_resolved = false;
        for table in tables {
            let table_columns = env.schema.tables_fields_names.get(table.as_str()).unwrap();
            if table_columns.contains(&hidden_selection.as_str()) {
                let hidden_selection_for_table = table_hidden_selections.get_mut(table).unwrap();
                if !hidden_selection_for_table.contains(hidden_selection) {
                    hidden_selection_for_table.push(hidden_selection.to_string());
                }
                // This symbol is resolved either if it pushed to the table or it's already their
                is_resolved = true;
                break;
            }
        }

        // If this symbol is not column name, maybe generated column
        if !is_resolved && !table_hidden_selections.is_empty() {
            table_hidden_selections
                .get_mut(&tables[0])
                .unwrap()
                .push(hidden_selection.to_string());
        }
    }

    table_hidden_selections
}

fn parse_select_statement(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Statement>, Box<Diagnostic>> {
    // Consume `SELECT` keyword
    *position += 1;

    if *position >= tokens.len() {
        return Err(Diagnostic::error("Incomplete input for select statement")
            .add_help("Try select one or more values in the `SELECT` statement")
            .add_note("Select statements requires at least selecting one value")
            .with_location(calculate_safe_location(tokens, *position - 1))
            .as_boxed());
    }

    // Parse `DISTINCT` or `DISTINCT ON(...)`
    let distinct = parse_select_distinct_option(context, tokens, position)?;

    // Parse `*` or `expressions`
    let mut fields_names: Vec<String> = vec![];
    let mut selected_expr_titles: Vec<String> = vec![];
    let mut selected_expr: Vec<Box<dyn Expr>> = vec![];
    let mut is_select_all = false;

    context.inside_selections = true;
    parse_select_all_or_expressions(
        context,
        env,
        tokens,
        position,
        &mut fields_names,
        &mut selected_expr_titles,
        &mut selected_expr,
        &mut is_select_all,
    )?;
    context.inside_selections = false;

    // Parse optional `FROM` with one or more tables and joins
    let mut joins: Vec<Join> = vec![];
    let mut tables_to_select_from: Vec<String> = vec![];
    parse_from_option(
        context,
        env,
        &mut tables_to_select_from,
        &mut joins,
        tokens,
        position,
    )?;

    // Make sure Aggregated functions are used with tables only
    if tables_to_select_from.is_empty() && !context.aggregations.is_empty() {
        return Err(
            Diagnostic::error("Aggregations functions should be used only with tables")
                .add_note("Try to select from one of the available tables in current schema")
                .with_location(calculate_safe_location(tokens, *position))
                .as_boxed(),
        );
    }

    // Make sure `SELECT *` used with specific table
    if is_select_all && tables_to_select_from.is_empty() {
        return Err(
            Diagnostic::error("Expect `FROM` and table name after `SELECT *`")
                .add_help("Select all must be used with valid table name")
                .with_location(calculate_safe_location(tokens, *position))
                .as_boxed(),
        );
    }

    // Select input validations
    if !is_select_all && fields_names.is_empty() {
        return Err(Diagnostic::error("Incomplete input for select statement")
            .add_help("Try select one or more values in the `SELECT` statement")
            .add_note("Select statements requires at least selecting one value")
            .with_location(calculate_safe_location(tokens, *position - 1))
            .as_boxed());
    }

    // If it `select *` make all table fields selectable
    if is_select_all {
        select_all_table_fields(
            env,
            &tables_to_select_from,
            &mut context.selected_fields,
            &mut fields_names,
        );
    }

    // Type check all selected fields has type registered in type table
    let table_selections = type_check_and_classify_selected_fields(
        env,
        &tables_to_select_from,
        &fields_names,
        calculate_safe_location(tokens, *position),
    )?;

    Ok(Box::new(SelectStatement {
        table_selections,
        joins,
        selected_expr_titles,
        selected_expr,
        distinct,
    }))
}

fn parse_select_distinct_option(
    context: &mut ParserContext,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Distinct, Box<Diagnostic>> {
    if is_current_token(tokens, position, TokenKind::Distinct) {
        // Consume `DISTINCT` keyword
        *position += 1;

        if is_current_token(tokens, position, TokenKind::On) {
            // Consume `ON` keyword
            *position += 1;

            if !is_current_token(tokens, position, TokenKind::LeftParen) {
                return Err(Diagnostic::error("Expect `(` after `DISTINCT ON`")
                    .add_help("Try to add `(` after ON and before fields")
                    .with_location(calculate_safe_location(tokens, *position))
                    .as_boxed());
            }

            // Consume `(` Left Parenthesis
            *position += 1;

            let mut distinct_fields: Vec<String> = vec![];
            while !is_current_token(tokens, position, TokenKind::RightParen) {
                let field_token = &tokens[*position];
                let literal = &field_token.to_string();
                let location = field_token.location;

                distinct_fields.push(literal.to_string());

                context.hidden_selections.push(literal.to_string());
                context.projection_names.push(literal.to_string());
                context.projection_locations.push(location);

                // Consume field name
                *position += 1;

                if is_current_token(tokens, position, TokenKind::Comma) {
                    // Consume `,`
                    *position += 1;
                } else {
                    break;
                }
            }

            // Consume `)` Right Parenthesis
            consume_token_or_error(
                tokens,
                position,
                TokenKind::RightParen,
                "Expect `)` after `DISTINCT ON fields`",
            )?;

            // Prevent passing empty fields
            if distinct_fields.is_empty() {
                return Err(Diagnostic::error(
                    "DISTINCT ON(...) must be used with one of more column",
                )
                .add_help("Try to add one or more columns from current table")
                .with_location(calculate_safe_location(tokens, *position))
                .as_boxed());
            }

            // Prevent user from writing comma after DISTINCT ON
            if is_current_token(tokens, position, TokenKind::Comma) {
                return Err(
                    Diagnostic::error("No need to add Comma `,` after DISTINCT ON")
                        .add_help("Try to remove `,` after DISTINCT ON fields")
                        .with_location(calculate_safe_location(tokens, *position))
                        .as_boxed(),
                );
            }

            return Ok(Distinct::DistinctOn(distinct_fields));
        }
        return Ok(Distinct::DistinctAll);
    }

    Ok(Distinct::None)
}

#[allow(clippy::too_many_arguments)]
fn parse_select_all_or_expressions(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
    fields_names: &mut Vec<String>,
    selected_expr_titles: &mut Vec<String>,
    selected_expr: &mut Vec<Box<dyn Expr>>,
    is_select_all: &mut bool,
) -> Result<(), Box<Diagnostic>> {
    // Check if it `SELECT *`
    if is_current_token(tokens, position, TokenKind::Star) {
        // Consume `*`
        *position += 1;
        *is_select_all = true;
        return Ok(());
    }

    // Parse list of expression separated by `,` or until end of file
    while !is_current_token(tokens, position, TokenKind::From) {
        let expression = parse_expression(context, env, tokens, position)?;
        let expr_type = expression.expr_type().clone();
        let field_name = expression_literal(&expression)
            .unwrap_or_else(|| context.name_generator.generate_column_name());

        // Assert that each selected field is unique
        if fields_names.contains(&field_name) {
            return Err(Diagnostic::error("Can't select the same field twice")
                .with_location(calculate_safe_location(tokens, *position - 1))
                .as_boxed());
        }

        // Check for Field name alias
        if is_current_token(tokens, position, TokenKind::As) {
            // Consume `as` keyword
            *position += 1;

            // Parse and consume Symbol as Elias name
            let alias_name = consume_conditional_token_or_errors(
                tokens,
                position,
                |token| matches!(token.kind, TokenKind::Symbol(_) | TokenKind::String(_)),
                "Expect `Symbol` or `Text` as field alias name",
            )?
            .to_string();

            // TODO [#120, #121]: Remove this check
            if env
                .schema
                .tables_fields_types
                .contains_key(alias_name.as_str())
            {
                return Err(Diagnostic::error("Can't use column name as Alias")
                    .add_note("Until supporting `table.column` you should use different alias name")
                    .with_location(tokens[*position - 1].location)
                    .as_boxed());
            }

            // No need to do checks or add alias
            // `SELECT C AS C` is equal to `SELECT C`
            if field_name != alias_name {
                if context.selected_fields.contains(&alias_name)
                    || context.name_alias_table.contains_key(&alias_name)
                {
                    return Err(
                        Diagnostic::error("You already have field with the same name")
                            .add_help("Try to use a new unique name for alias")
                            .with_location(tokens[*position - 1].location)
                            .as_boxed(),
                    );
                }

                // Register alias name type
                env.define(alias_name.to_string(), expr_type.clone());

                context.selected_fields.push(alias_name.clone());
                context
                    .name_alias_table
                    .insert(field_name.to_string(), alias_name.to_string());
            }

            selected_expr_titles.push(alias_name.to_owned());
        } else {
            selected_expr_titles.push(field_name.to_owned());
        }

        // Register field type
        env.define(field_name.to_string(), expr_type);

        fields_names.push(field_name.to_owned());
        context.selected_fields.push(field_name.to_owned());

        selected_expr.push(expression);

        // Consume `,` or break
        if is_current_token(tokens, position, TokenKind::Comma) {
            *position += 1;
        } else {
            break;
        }
    }

    Ok(())
}

fn parse_from_option(
    context: &mut ParserContext,
    env: &mut Environment,
    tables_to_select_from: &mut Vec<String>,
    joins: &mut Vec<Join>,
    tokens: &[Token],
    position: &mut usize,
) -> Result<(), Box<Diagnostic>> {
    if is_current_token(tokens, position, TokenKind::From) {
        // Consume `From` keyword
        *position += 1;

        // Parse and consume Symbol as Table name
        let table_name = consume_conditional_token_or_errors(
            tokens,
            position,
            |token| matches!(token.kind, TokenKind::Symbol(_)),
            "Expect `Table` value after `FROM` keyword",
        )?
        .to_string();

        if !env
            .schema
            .tables_fields_names
            .contains_key(table_name.as_str())
        {
            let mut diagnostic =
                Diagnostic::error(&format!("Cannot find table with name `{table_name}`"))
                    .add_note("You can use the `SHOW TABLES` query to get list of current tables")
                    .add_note("Check the documentations to see available tables")
                    .with_location(calculate_safe_location(tokens, *position));

            let canditates: Vec<&&str> = env.schema.tables_fields_names.keys().collect();
            if let Some(closest_valid_name) = find_closeest_string(&table_name, &canditates) {
                let message =
                    &format!("a table with a similar name exists: `{closest_valid_name}`");
                diagnostic = diagnostic.add_help(message);
            }

            return Err(diagnostic.as_boxed());
        }

        // Register the table
        tables_to_select_from.push(table_name.to_string());
        context.selected_tables.push(table_name.to_string());
        register_current_table_fields_types(env, &table_name)?;

        // Parse Joins
        let mut number_previous_of_joins = 0;
        while is_join_or_join_type_token(tokens, position) {
            let join_token = &tokens[*position];

            // The default join type now is cross join because we don't support `ON` Condition
            let mut join_kind = JoinKind::Default;
            if join_token.kind != TokenKind::Join {
                join_kind = match join_token.kind {
                    TokenKind::Left => JoinKind::Left,
                    TokenKind::Right => JoinKind::Right,
                    TokenKind::Cross => JoinKind::Cross,
                    TokenKind::Inner => JoinKind::Inner,
                    _ => JoinKind::Default,
                };

                // Consume Left, Right, Inner or Cross
                *position += 1;

                // Parse optional `OUTER` token after `LEFT` or `RIGHT` only
                if is_current_token(tokens, position, TokenKind::Outer) {
                    if !matches!(join_kind, JoinKind::Left | JoinKind::Right) {
                        return Err(Diagnostic::error(
                            "`OUTER` keyword used with LEFT or RIGHT JOIN only",
                        )
                        .with_location(calculate_safe_location(tokens, *position))
                        .as_boxed());
                    }

                    // Consume `OUTER` keyword
                    *position += 1;
                }

                if *position >= tokens.len() || tokens[*position].kind != TokenKind::Join {
                    return Err(Diagnostic::error(
                        "Expect `JOIN` keyword after Cross, Left, Right, Inner",
                    )
                    .with_location(calculate_safe_location(tokens, *position))
                    .as_boxed());
                }
            }

            // Consume `JOIN` keyword
            let join_location = tokens[*position].location;
            *position += 1;

            if *position >= tokens.len() || !matches!(tokens[*position].kind, TokenKind::Symbol(_))
            {
                return Err(Diagnostic::error("Expect table name after `JOIN` keyword")
                    .with_location(calculate_safe_location(tokens, *position))
                    .as_boxed());
            }

            let other_table = &tokens[*position];
            let other_table_name = &other_table.to_string();

            // Make sure the RIGHT and LEFT tables names are not the same
            if number_previous_of_joins == 0 && table_name.eq(other_table_name) {
                return Err(Diagnostic::error(
                    "The two tables of join must be unique or have different alias",
                )
                .with_location(calculate_safe_location(tokens, *position))
                .as_boxed());
            }

            tables_to_select_from.push(other_table_name.to_string());
            context.selected_tables.push(other_table_name.to_string());
            register_current_table_fields_types(env, other_table_name)?;

            // Consume Other table name
            *position += 1;

            // Parse the `ON` predicate
            let mut predicate: Option<Box<dyn Expr>> = None;
            if is_current_token(tokens, position, TokenKind::On) {
                // Consume `ON` keyword
                *position += 1;
                predicate = Some(parse_expression(context, env, tokens, position)?);
            }

            // Make sure user set predicate condition for LEFT or RIGHT JOIN
            if predicate.is_none() && matches!(join_kind, JoinKind::Right | JoinKind::Left) {
                return Err(Diagnostic::error(
                    "You must set predicate condition using `ON` Keyword for `LEFT` OR `RIGHT` JOINS",
                )
                .with_location(join_location)
                .as_boxed());
            }

            let join_operand = if number_previous_of_joins == 0 {
                JoinOperand::OuterAndInner(table_name.to_string(), other_table_name.to_string())
            } else {
                JoinOperand::Inner(other_table_name.to_string())
            };

            joins.push(Join {
                operand: join_operand,
                kind: join_kind,
                predicate,
            });

            number_previous_of_joins += 1;
        }
    }
    Ok(())
}

fn parse_where_statement(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Statement>, Box<Diagnostic>> {
    *position += 1;
    if *position >= tokens.len() {
        return Err(Diagnostic::error("Expect expression after `WHERE` keyword")
            .add_help("Try to add boolean expression after `WHERE` keyword")
            .add_note("`WHERE` statement expects expression as condition")
            .with_location(calculate_safe_location(tokens, *position - 1))
            .as_boxed());
    }

    let aggregations_count_before = context.aggregations.len();

    // Make sure WHERE condition expression has boolean type or can implicit casted to boolean
    let condition_location = tokens[*position].location;
    let mut condition = parse_expression(context, env, tokens, position)?;

    // Make sure that the condition type is boolean, or can implicit cast to boolean.
    if !condition.expr_type().is_bool() {
        let expected_type: Box<dyn DataType> = Box::new(BoolType);
        if !expected_type.has_implicit_cast_from(&condition) {
            return Err(Diagnostic::error(&format!(
                "Expect `WHERE` condition to be type {} but got {}",
                "Boolean",
                condition.expr_type().literal()
            ))
            .add_note("`WHERE` statement condition must be Boolean")
            .with_location(condition_location)
            .as_boxed());
        }

        // Implicit cast the condition to boolean
        condition = Box::new(CastExpr {
            value: condition,
            result_type: expected_type.clone(),
        })
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
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Statement>, Box<Diagnostic>> {
    // Consume `Group` keyword
    *position += 1;

    // Consume `By` keyword
    consume_token_or_error(
        tokens,
        position,
        TokenKind::By,
        "Expect keyword `BY` after keyword `group`",
    )?;

    // Parse one or more expression
    let mut values: Vec<Box<dyn Expr>> = vec![];
    while *position < tokens.len() {
        values.push(parse_expression(context, env, tokens, position)?);
        if is_current_token(tokens, position, TokenKind::Comma) {
            // Consume Comma `,`
            *position += 1;
            continue;
        }
        break;
    }

    let mut has_with_rollup = false;
    if is_current_token(tokens, position, TokenKind::With) {
        // Consume Comma `WITH``
        *position += 1;

        // Consume `Rollup` keyword
        consume_token_or_error(
            tokens,
            position,
            TokenKind::Rollup,
            "Expect keyword `ROLLUP` after keyword `with`",
        )?;

        has_with_rollup = true;
    }

    context.has_group_by_statement = true;
    Ok(Box::new(GroupByStatement {
        values,
        has_with_roll_up: has_with_rollup,
    }))
}

fn parse_having_statement(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Statement>, Box<Diagnostic>> {
    context.inside_having = true;

    // Consume `HAVING` token
    *position += 1;

    if *position >= tokens.len() {
        return Err(
            Diagnostic::error("Expect expression after `HAVING` keyword")
                .add_help("Try to add boolean expression after `HAVING` keyword")
                .add_note("`HAVING` statement expects expression as condition")
                .with_location(calculate_safe_location(tokens, *position - 1))
                .as_boxed(),
        );
    }

    // Make sure HAVING condition expression has boolean type
    let condition_location = tokens[*position].location;
    let mut condition = parse_expression(context, env, tokens, position)?;

    // Make sure that the condition type is boolean, or can implicit cast to boolean.
    if !condition.expr_type().is_bool() {
        let expected_type: Box<dyn DataType> = Box::new(BoolType);
        if !expected_type.has_implicit_cast_from(&condition) {
            return Err(Diagnostic::error(&format!(
                "Expect `HAVING` condition to be type {} but got {}",
                "Boolean",
                condition.expr_type().literal()
            ))
            .add_note("`HAVING` statement condition must be Boolean")
            .with_location(condition_location)
            .as_boxed());
        }

        // Implicit cast the condition to boolean
        condition = Box::new(CastExpr {
            value: condition,
            result_type: expected_type.clone(),
        })
    }

    context.inside_having = false;
    Ok(Box::new(HavingStatement { condition }))
}

fn parse_qualify_statement(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Statement>, Box<Diagnostic>> {
    // Consume `QUALIFY` token
    *position += 1;

    if *position >= tokens.len() {
        return Err(
            Diagnostic::error("Expect expression after `QUALIFY` keyword")
                .add_help("Try to add boolean expression after `QUALIFY` keyword")
                .add_note("`QUALIFY` statement expects expression as condition")
                .with_location(calculate_safe_location(tokens, *position - 1))
                .as_boxed(),
        );
    }

    // Make sure QUALIFY condition expression has boolean type
    let condition_location = tokens[*position].location;
    let mut condition = parse_expression(context, env, tokens, position)?;

    // Make sure that the condition type is boolean, or can implicit cast to boolean.
    if !condition.expr_type().is_bool() {
        let expected_type: Box<dyn DataType> = Box::new(BoolType);
        if !expected_type.has_implicit_cast_from(&condition) {
            return Err(Diagnostic::error(&format!(
                "Expect `QUALIFY` condition to be type {} but got {}",
                "Boolean",
                condition.expr_type().literal()
            ))
            .add_note("`QUALIFY` statement condition must be Boolean")
            .with_location(condition_location)
            .as_boxed());
        }

        // Implicit cast the condition to boolean
        condition = Box::new(CastExpr {
            value: condition,
            result_type: expected_type.clone(),
        })
    }

    Ok(Box::new(QualifyStatement { condition }))
}

fn parse_limit_statement(
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Statement>, Box<Diagnostic>> {
    // Consume `LIMIT` keyword
    *position += 1;

    if *position >= tokens.len() {
        return Err(Diagnostic::error("Expect number after `LIMIT` keyword")
            .with_location(calculate_safe_location(tokens, *position - 1))
            .as_boxed());
    }

    match tokens[*position].kind {
        TokenKind::Integer(integer) => {
            // Consume Integer value
            *position += 1;

            // Make sure limit value is always positive
            if integer < 0 {
                return Err(
                    Diagnostic::error("Expect positive number after `LIMIT` keyword")
                        .with_location(calculate_safe_location(tokens, *position - 1))
                        .as_boxed(),
                );
            }

            let count = integer as usize;
            Ok(Box::new(LimitStatement { count }))
        }
        _ => Err(Diagnostic::error("Expect number after `LIMIT` keyword")
            .with_location(calculate_safe_location(tokens, *position - 1))
            .as_boxed()),
    }
}

fn parse_offset_statement(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Statement>, Box<Diagnostic>> {
    // Consume `OFFSET` keyword
    *position += 1;

    if *position >= tokens.len() {
        return Err(Diagnostic::error("Expect number after `OFFSET` keyword")
            .with_location(calculate_safe_location(tokens, *position - 1))
            .as_boxed());
    }

    let start = parse_expression(context, env, tokens, position)?;
    if start.expr_type().is_int() {
        return Ok(Box::new(OffsetStatement { start }));
    }

    Err(Diagnostic::error("Expect int after `OFFSET` keyword")
        .with_location(calculate_safe_location(tokens, *position - 1))
        .as_boxed())
}

fn parse_window_named_over_clause(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<(), Box<Diagnostic>> {
    consume_token_or_error(
        tokens,
        position,
        TokenKind::Window,
        "Expect `WINDOW` keyword.",
    )?;

    let window_name_token = consume_conditional_token_or_errors(
        tokens,
        position,
        |t| matches!(t.kind, TokenKind::Symbol(_)),
        "Expect `Identifier` as window over clauses name.",
    )?;

    let location = window_name_token.location;
    let window_name = window_name_token.to_string();

    consume_token_or_error(
        tokens,
        position,
        TokenKind::As,
        "Expect `AS` keyword after window name.",
    )?;

    let over_clauses = parse_over_window_definition(context, env, tokens, position)?;

    // Make sure each window clauses has unique name
    if context.named_window_clauses.contains_key(&window_name) {
        return Err(Diagnostic::error(&format!(
            "There is already defined window clauses with name {window_name}"
        ))
        .add_note("Window over clauses names must be unique from each other")
        .with_location(location)
        .as_boxed());
    }

    // Register window over clauses with name
    context
        .named_window_clauses
        .insert(window_name, over_clauses);

    Ok(())
}

pub(crate) fn parse_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Expr>, Box<Diagnostic>> {
    let aggregation_count_before = context.aggregations.len();
    let window_count_before = context.window_functions.len();
    let expression = parse_assignment_expression(context, env, tokens, position)?;

    if expression.kind() != ExprKind::Symbol {
        // This Expression contains aggregate function call or aggregate value
        if aggregation_count_before != context.aggregations.len() {
            let column_name = context.name_generator.generate_column_name();
            let expr_type = expression.expr_type();
            env.define(column_name.to_string(), expr_type.clone());

            // Register the new aggregation generated field if the this expression is after group by
            if context.has_group_by_statement && !context.hidden_selections.contains(&column_name) {
                context.hidden_selections.push(column_name.to_string());
            }

            context
                .aggregations
                .insert(column_name.clone(), AggregateValue::Expression(expression));

            return Ok(Box::new(SymbolExpr {
                value: column_name,
                expr_type,
                flag: SymbolFlag::None,
            }));
        }

        // This Expression contains window function call or window value
        if window_count_before != context.window_functions.len() {
            let column_name = context.name_generator.generate_column_name();
            let expr_type = expression.expr_type();
            env.define(column_name.to_string(), expr_type.clone());

            // Register the new window generated field if the this expression is after group by
            if context.has_group_by_statement && !context.hidden_selections.contains(&column_name) {
                context.hidden_selections.push(column_name.to_string());
            }

            context
                .window_functions
                .insert(column_name.clone(), WindowValue::Expression(expression));

            return Ok(Box::new(SymbolExpr {
                value: column_name,
                expr_type,
                flag: SymbolFlag::None,
            }));
        }

        return Ok(expression);
    }

    Ok(expression)
}

fn parse_assignment_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Expr>, Box<Diagnostic>> {
    let expression = parse_regex_expression(context, env, tokens, position)?;
    if is_current_token(tokens, position, TokenKind::ColonEqual) {
        if expression.kind() != ExprKind::GlobalVariable {
            return Err(Diagnostic::error(
                "Assignment expressions expect global variable name before `:=`",
            )
            .with_location(tokens[*position].location)
            .as_boxed());
        }

        let expr = expression
            .as_any()
            .downcast_ref::<GlobalVariableExpr>()
            .unwrap();

        let variable_name = expr.name.to_string();

        // Consume `:=` operator
        *position += 1;

        let value = parse_regex_expression(context, env, tokens, position)?;
        env.define_global(variable_name.clone(), value.expr_type());

        return Ok(Box::new(AssignmentExpr {
            symbol: variable_name.clone(),
            value,
        }));
    }
    Ok(expression)
}

fn parse_regex_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Expr>, Box<Diagnostic>> {
    let lhs = parse_is_null_expression(context, env, tokens, position)?;

    // Check for `REGEXP` or `NOT REGEXP`
    // <expr> REGEXP <expr> AND <expr>
    // <expr> NOT REGEXP <expr> AND <expr>
    if is_current_token(tokens, position, TokenKind::RegExp)
        || (is_current_token(tokens, position, TokenKind::Not)
            && is_next_token(tokens, position, TokenKind::RegExp))
    {
        let has_not_keyword = is_current_token(tokens, position, TokenKind::Not);
        let operator_location = if has_not_keyword {
            // Consume `NOT` and `REGEXP` keyword
            *position += 2;
            let mut not_location = tokens[*position - 2].location;
            let between_location = tokens[*position - 1].location;
            not_location.expand_until(between_location);
            not_location
        } else {
            // Consume `REGEXP` keyword
            *position += 1;
            tokens[*position - 1].location
        };

        let pattern = parse_is_null_expression(context, env, tokens, position)?;

        let lhs_type = lhs.expr_type();
        let rhs_type = pattern.expr_type();

        // Can perform this operator between LHS and RHS
        let expected_rhs_types = lhs_type.can_perform_regexp_op_with();
        if expected_rhs_types.contains(&rhs_type) {
            let regex_expr = Box::new(RegexExpr {
                input: lhs,
                pattern,
            });

            return Ok(apply_not_keyword_if_exists(regex_expr, has_not_keyword));
        }

        // Check if RHS expr can be implicit casted to Expected LHS type to make this
        // Expression valid
        for expected_type in expected_rhs_types.iter() {
            if !expected_type.has_implicit_cast_from(&pattern) {
                continue;
            }

            let casting = Box::new(CastExpr {
                value: pattern,
                result_type: expected_type.clone(),
            });

            let expr = Box::new(RegexExpr {
                input: lhs,
                pattern: casting,
            });

            return Ok(apply_not_keyword_if_exists(expr, has_not_keyword));
        }

        // Return error if this operator can't be performed even with implicit cast
        return Err(Diagnostic::error(&format!(
            "Operator `REGEXP` can't be performed between types `{lhs_type}` and `{rhs_type}`",
        ))
        .with_location(operator_location)
        .as_boxed());
    }

    Ok(lhs)
}

fn parse_is_null_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Expr>, Box<Diagnostic>> {
    let expression = parse_in_expression(context, env, tokens, position)?;
    if is_current_token(tokens, position, TokenKind::Is) {
        let is_location = tokens[*position].location;

        // Consume `IS` keyword
        *position += 1;

        let has_not_keyword = if is_current_token(tokens, position, TokenKind::Not) {
            // Consume `NOT` keyword
            *position += 1;
            true
        } else {
            false
        };

        if is_current_token(tokens, position, TokenKind::Null) {
            // Consume `Null` keyword
            *position += 1;

            return Ok(Box::new(IsNullExpr {
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
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Expr>, Box<Diagnostic>> {
    let expression = parse_logical_or_expression(context, env, tokens, position)?;

    // Consume NOT if current token is `NOT` and next one is `IN`
    let has_not_keyword = if *position < tokens.len() - 1
        && tokens[*position].kind == TokenKind::Not
        && tokens[*position + 1].kind == TokenKind::In
    {
        *position += 1;
        true
    } else {
        false
    };

    if is_current_token(tokens, position, TokenKind::In) {
        let in_location = tokens[*position].location;

        // Consume `IN` keyword
        *position += 1;

        if !is_current_token(tokens, position, TokenKind::LeftParen) {
            return Err(Diagnostic::error("Expects `(` After `IN` Keyword")
                .with_location(in_location)
                .as_boxed());
        }

        let values =
            parse_zero_or_more_values_with_comma_between(context, env, tokens, position, "IN")?;

        // Optimize the Expression if the number of values in the list is 0
        if values.is_empty() {
            let is_true = has_not_keyword;
            return Ok(Box::new(BooleanExpr { is_true }));
        }

        let values_type_result = check_all_values_are_same_type(&values);
        if values_type_result.is_none() {
            return Err(Diagnostic::error(
                "Expects values between `(` and `)` to have the same type",
            )
            .with_location(in_location)
            .as_boxed());
        }

        // Check that argument and values has the same type
        let values_type = values_type_result.unwrap();
        if !values_type.is_any() && !expression.expr_type().equals(&values_type) {
            return Err(Diagnostic::error(
                "Argument and Values of In Expression must have the same type",
            )
            .with_location(in_location)
            .as_boxed());
        }

        return Ok(Box::new(InExpr {
            argument: expression,
            values,
            values_type,
            has_not_keyword,
        }));
    }

    Ok(expression)
}

fn parse_logical_or_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Expr>, Box<Diagnostic>> {
    let mut lhs = parse_logical_and_expression(context, env, tokens, position)?;

    'parse_expr: while is_logical_or_operator(tokens, position) {
        let operator = &tokens[*position];

        // Consume`OR` operator
        *position += 1;

        let rhs = parse_logical_and_expression(context, env, tokens, position)?;

        let lhs_type = lhs.expr_type();
        let rhs_type = rhs.expr_type();

        let expected_rhs_types = lhs_type.can_perform_logical_or_op_with();

        // Can perform this operator between LHS and RHS
        if expected_rhs_types.contains(&rhs_type) {
            lhs = Box::new(LogicalExpr {
                left: lhs,
                operator: BinaryLogicalOperator::Or,
                right: rhs,
            });

            continue 'parse_expr;
        }

        // Check if RHS expr can be implicit casted to Expected LHS type to make this
        // Expression valid
        for expected_type in expected_rhs_types {
            if !expected_type.has_implicit_cast_from(&lhs) {
                continue;
            }

            let casting = Box::new(CastExpr {
                value: rhs,
                result_type: expected_type.clone(),
            });

            lhs = Box::new(LogicalExpr {
                left: lhs,
                operator: BinaryLogicalOperator::Or,
                right: casting,
            });

            continue 'parse_expr;
        }

        // Check if LHS expr can be implicit casted to Expected RHS type to make this
        // Expression valid
        let expected_lhs_types = rhs_type.can_perform_logical_or_op_with();
        for expected_type in expected_lhs_types.iter() {
            if !expected_type.has_implicit_cast_from(&lhs) {
                continue;
            }

            let casting = Box::new(CastExpr {
                value: lhs,
                result_type: expected_type.clone(),
            });

            lhs = Box::new(LogicalExpr {
                left: casting,
                operator: BinaryLogicalOperator::Or,
                right: rhs,
            });

            continue 'parse_expr;
        }

        // Return error if this operator can't be performed even with implicit cast
        return Err(Diagnostic::error(&format!(
            "Operator `OR` can't be performed between types `{lhs_type}` and `{rhs_type}`"
        ))
        .with_location(operator.location)
        .as_boxed());
    }

    Ok(lhs)
}

fn parse_logical_and_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Expr>, Box<Diagnostic>> {
    let mut lhs = parse_bitwise_or_expression(context, env, tokens, position)?;

    'parse_expr: while is_logical_and_operator(tokens, position) {
        let operator = &tokens[*position];

        // Consume`AND` operator
        *position += 1;

        let rhs = parse_bitwise_or_expression(context, env, tokens, position)?;

        let lhs_type = lhs.expr_type();
        let rhs_type = rhs.expr_type();

        let expected_rhs_types = lhs_type.can_perform_logical_and_op_with();

        // Can perform this operator between LHS and RHS
        if expected_rhs_types.contains(&rhs_type) {
            lhs = Box::new(LogicalExpr {
                left: lhs,
                operator: BinaryLogicalOperator::And,
                right: rhs,
            });

            continue 'parse_expr;
        }

        // Check if RHS expr can be implicit casted to Expected LHS type to make this
        // Expression valid
        for expected_type in expected_rhs_types.iter() {
            if !expected_type.has_implicit_cast_from(&rhs) {
                continue;
            }

            let casting = Box::new(CastExpr {
                value: rhs,
                result_type: expected_type.clone(),
            });

            lhs = Box::new(LogicalExpr {
                left: lhs,
                operator: BinaryLogicalOperator::And,
                right: casting,
            });

            continue 'parse_expr;
        }

        // Check if LHS expr can be implicit casted to Expected RHS type to make this
        // Expression valid
        let expected_lhs_types = rhs_type.can_perform_logical_and_op_with();
        for expected_type in expected_lhs_types.iter() {
            if !expected_type.has_implicit_cast_from(&lhs) {
                continue;
            }

            let casting = Box::new(CastExpr {
                value: lhs,
                result_type: expected_type.clone(),
            });

            lhs = Box::new(LogicalExpr {
                left: casting,
                operator: BinaryLogicalOperator::And,
                right: rhs,
            });

            continue 'parse_expr;
        }

        // Return error if this operator can't be performed even with implicit cast
        return Err(Diagnostic::error(&format!(
            "Operator `AND` can't be performed between types `{lhs_type}` and `{rhs_type}`"
        ))
        .with_location(operator.location)
        .as_boxed());
    }

    Ok(lhs)
}

fn parse_bitwise_or_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Expr>, Box<Diagnostic>> {
    let mut lhs = parse_bitwise_xor_expression(context, env, tokens, position)?;

    'parse_expr: while is_current_token(tokens, position, TokenKind::BitwiseOr) {
        let operator = &tokens[*position];

        // Consume `|` token
        *position += 1;

        let rhs = parse_bitwise_xor_expression(context, env, tokens, position)?;

        let lhs_type = lhs.expr_type();
        let rhs_type = rhs.expr_type();

        let expected_rhs_types = lhs_type.can_perform_or_op_with();

        // Can perform this operator between LHS and RHS
        if expected_rhs_types.contains(&rhs_type) {
            lhs = Box::new(BitwiseExpr {
                left: lhs,
                operator: BinaryBitwiseOperator::Or,
                right: rhs,
                result_type: lhs_type.or_op_result_type(&rhs_type),
            });

            continue 'parse_expr;
        }

        // Check if RHS expr can be implicit casted to Expected LHS type to make this
        // Expression valid
        for expected_type in expected_rhs_types.iter() {
            if !expected_type.has_implicit_cast_from(&rhs) {
                continue;
            }

            let casting = Box::new(CastExpr {
                value: rhs,
                result_type: expected_type.clone(),
            });

            lhs = Box::new(BitwiseExpr {
                left: lhs,
                operator: BinaryBitwiseOperator::Or,
                right: casting,
                result_type: lhs_type.or_op_result_type(expected_type),
            });

            continue 'parse_expr;
        }

        // Check if LHS expr can be implicit casted to Expected RHS type to make this
        // Expression valid
        let expected_lhs_types = rhs_type.can_perform_or_op_with();
        for expected_type in expected_lhs_types.iter() {
            if !expected_type.has_implicit_cast_from(&lhs) {
                continue;
            }

            let casting = Box::new(CastExpr {
                value: lhs,
                result_type: expected_type.clone(),
            });

            lhs = Box::new(BitwiseExpr {
                left: casting,
                operator: BinaryBitwiseOperator::Or,
                right: rhs,
                result_type: rhs_type.or_op_result_type(expected_type),
            });

            continue 'parse_expr;
        }

        // Return error if this operator can't be performed even with implicit cast
        return Err(Diagnostic::error(&format!(
            "Operator `|` can't be performed between types `{lhs_type}` and `{rhs_type}`"
        ))
        .with_location(operator.location)
        .as_boxed());
    }

    Ok(lhs)
}

fn parse_bitwise_xor_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Expr>, Box<Diagnostic>> {
    let mut lhs = parse_logical_xor_expression(context, env, tokens, position)?;

    'parse_expr: while is_current_token(tokens, position, TokenKind::BitwiseXor) {
        let operator = &tokens[*position];

        // Consume`#` operator
        *position += 1;

        let rhs = parse_logical_xor_expression(context, env, tokens, position)?;

        let lhs_type = lhs.expr_type();
        let rhs_type = rhs.expr_type();

        let expected_rhs_types = lhs_type.can_perform_xor_op_with();

        // Can perform this operator between LHS and RHS
        if expected_rhs_types.contains(&rhs_type) {
            lhs = Box::new(BitwiseExpr {
                left: lhs,
                operator: BinaryBitwiseOperator::Xor,
                right: rhs,
                result_type: lhs_type.xor_op_result_type(&rhs_type),
            });

            continue 'parse_expr;
        }

        // Check if RHS expr can be implicit casted to Expected LHS type to make this
        // Expression valid
        for expected_type in expected_rhs_types.iter() {
            if !expected_type.has_implicit_cast_from(&rhs) {
                continue;
            }

            let casting = Box::new(CastExpr {
                value: rhs,
                result_type: expected_type.clone(),
            });

            lhs = Box::new(BitwiseExpr {
                left: lhs,
                operator: BinaryBitwiseOperator::Xor,
                right: casting,
                result_type: lhs_type.or_op_result_type(expected_type),
            });

            continue 'parse_expr;
        }

        // Check if LHS expr can be implicit casted to Expected RHS type to make this
        // Expression valid
        let expected_lhs_types = rhs_type.can_perform_xor_op_with();
        for expected_type in expected_lhs_types.iter() {
            if !expected_type.has_implicit_cast_from(&lhs) {
                continue;
            }

            let casting = Box::new(CastExpr {
                value: lhs,
                result_type: expected_type.clone(),
            });

            lhs = Box::new(BitwiseExpr {
                left: casting,
                operator: BinaryBitwiseOperator::Xor,
                right: rhs,
                result_type: rhs_type.or_op_result_type(expected_type),
            });

            continue 'parse_expr;
        }

        // Return error if this operator can't be performed even with implicit cast
        return Err(Diagnostic::error(&format!(
            "Operator `#` can't be performed between types `{lhs_type}` and `{rhs_type}`"
        ))
        .with_location(operator.location)
        .as_boxed());
    }

    Ok(lhs)
}

fn parse_logical_xor_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Expr>, Box<Diagnostic>> {
    let mut lhs = parse_bitwise_and_expression(context, env, tokens, position)?;

    'parse_expr: while is_current_token(tokens, position, TokenKind::XorKeyword) {
        let operator = &tokens[*position];

        // Consume`XOR` operator
        *position += 1;

        let rhs = parse_bitwise_and_expression(context, env, tokens, position)?;

        let lhs_type = lhs.expr_type();
        let rhs_type = rhs.expr_type();

        let expected_rhs_types = lhs_type.can_perform_logical_xor_op_with();

        // Can perform this operator between LHS and RHS
        if expected_rhs_types.contains(&rhs_type) {
            lhs = Box::new(LogicalExpr {
                left: lhs,
                operator: BinaryLogicalOperator::Xor,
                right: rhs,
            });

            continue 'parse_expr;
        }

        // Check if RHS expr can be implicit casted to Expected LHS type to make this
        // Expression valid
        for expected_type in expected_rhs_types.iter() {
            if !expected_type.has_implicit_cast_from(&rhs) {
                continue;
            }

            let casting = Box::new(CastExpr {
                value: rhs,
                result_type: expected_type.clone(),
            });

            lhs = Box::new(LogicalExpr {
                left: lhs,
                operator: BinaryLogicalOperator::Xor,
                right: casting,
            });

            continue 'parse_expr;
        }

        // Check if LHS expr can be implicit casted to Expected RHS type to make this
        // Expression valid
        let expected_lhs_types = rhs_type.can_perform_logical_xor_op_with();
        for expected_type in expected_lhs_types.iter() {
            if !expected_type.has_implicit_cast_from(&lhs) {
                continue;
            }

            let casting = Box::new(CastExpr {
                value: lhs,
                result_type: expected_type.clone(),
            });

            lhs = Box::new(LogicalExpr {
                left: casting,
                operator: BinaryLogicalOperator::Xor,
                right: rhs,
            });

            continue 'parse_expr;
        }

        // Return error if this operator can't be performed even with implicit cast
        return Err(Diagnostic::error(&format!(
            "Operator `XOR` can't be performed between types `{lhs_type}` and `{rhs_type}`"
        ))
        .with_location(operator.location)
        .as_boxed());
    }

    Ok(lhs)
}

fn parse_bitwise_and_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Expr>, Box<Diagnostic>> {
    let mut lhs = parse_comparison_expression(context, env, tokens, position)?;

    'parse_expr: while is_current_token(tokens, position, TokenKind::BitwiseAnd) {
        let operator = &tokens[*position];

        // Consume `&&` token
        *position += 1;

        let rhs = parse_comparison_expression(context, env, tokens, position)?;

        let lhs_type = lhs.expr_type();
        let rhs_type = rhs.expr_type();

        let expected_rhs_types = lhs_type.can_perform_and_op_with();

        // Can perform this operator between LHS and RHS
        if expected_rhs_types.contains(&rhs_type) {
            lhs = Box::new(BitwiseExpr {
                left: lhs,
                operator: BinaryBitwiseOperator::And,
                right: rhs,
                result_type: lhs_type.or_op_result_type(&rhs_type),
            });

            continue 'parse_expr;
        }

        // Check if RHS expr can be implicit casted to Expected LHS type to make this
        // Expression valid
        for expected_type in expected_rhs_types.iter() {
            if !expected_type.has_implicit_cast_from(&rhs) {
                continue;
            }

            let casting = Box::new(CastExpr {
                value: rhs,
                result_type: expected_type.clone(),
            });

            lhs = Box::new(BitwiseExpr {
                left: lhs,
                operator: BinaryBitwiseOperator::And,
                right: casting,
                result_type: lhs_type.or_op_result_type(expected_type),
            });

            continue 'parse_expr;
        }

        // Check if LHS expr can be implicit casted to Expected RHS type to make this
        // Expression valid
        let expected_lhs_types = rhs_type.can_perform_and_op_with();
        for expected_type in expected_lhs_types.iter() {
            if expected_type.has_implicit_cast_from(&lhs) {
                let casting = Box::new(CastExpr {
                    value: lhs,
                    result_type: expected_type.clone(),
                });

                lhs = Box::new(BitwiseExpr {
                    left: casting,
                    operator: BinaryBitwiseOperator::And,
                    right: rhs,
                    result_type: rhs_type.or_op_result_type(expected_type),
                });

                continue 'parse_expr;
            }
        }

        // Return error if this operator can't be performed even with implicit cast
        return Err(Diagnostic::error(&format!(
            "Operator `&&` can't be performed between types `{lhs_type}` and `{rhs_type}`"
        ))
        .with_location(operator.location)
        .as_boxed());
    }

    Ok(lhs)
}

pub(crate) fn parse_contains_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Expr>, Box<Diagnostic>> {
    let lhs = parse_contained_by_expression(context, env, tokens, position)?;

    if is_current_token(tokens, position, TokenKind::AtRightArrow) {
        let operator = &tokens[*position];

        // Consume `@>` token
        *position += 1;

        let rhs = parse_contained_by_expression(context, env, tokens, position)?;

        let lhs_type = lhs.expr_type();
        let rhs_type = rhs.expr_type();

        let expected_rhs_types = lhs_type.can_perform_contains_op_with();

        // Can perform this operator between LHS and RHS
        if expected_rhs_types.contains(&rhs_type) {
            return Ok(Box::new(ContainsExpr {
                left: lhs,
                right: rhs,
            }));
        }

        // Check if can perform the operator with additional implicit casting
        for expected_type in expected_rhs_types.iter() {
            if !expected_type.has_implicit_cast_from(&rhs) {
                continue;
            }

            let casting = Box::new(CastExpr {
                value: rhs,
                result_type: expected_type.clone(),
            });

            return Ok(Box::new(ContainsExpr {
                left: lhs,
                right: casting,
            }));
        }

        // Return error if this operator can't be performed even with implicit cast
        return Err(Diagnostic::error(&format!(
            "Operator `@>` can't be performed between types `{lhs_type}` and `{rhs_type}`"
        ))
        .with_location(operator.location)
        .as_boxed());
    }

    Ok(lhs)
}

fn parse_contained_by_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Expr>, Box<Diagnostic>> {
    let lhs = parse_bitwise_shift_expression(context, env, tokens, position)?;

    if is_current_token(tokens, position, TokenKind::ArrowRightAt) {
        let operator = &tokens[*position];

        // Consume `<@` token
        *position += 1;

        let rhs = parse_bitwise_shift_expression(context, env, tokens, position)?;

        let lhs_type = lhs.expr_type();
        let rhs_type = rhs.expr_type();

        let expected_lhs_types = rhs_type.can_perform_contains_op_with();

        // Can perform this operator between LHS and RHS
        if expected_lhs_types.contains(&lhs_type) {
            return Ok(Box::new(ContainedByExpr {
                left: lhs,
                right: rhs,
            }));
        }

        // Check if can perform the operator with additional implicit casting
        for expected_type in expected_lhs_types.iter() {
            if !expected_type.has_implicit_cast_from(&lhs) {
                continue;
            }

            let casting = Box::new(CastExpr {
                value: lhs,
                result_type: expected_type.clone(),
            });

            return Ok(Box::new(ContainedByExpr {
                left: casting,
                right: rhs,
            }));
        }

        // Return error if this operator can't be performed even with implicit cast
        return Err(Diagnostic::error(&format!(
            "Operator `<@` can't be performed between types `{lhs_type}` and `{rhs_type}`"
        ))
        .with_location(operator.location)
        .as_boxed());
    }

    Ok(lhs)
}

fn parse_bitwise_shift_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Expr>, Box<Diagnostic>> {
    let mut lhs = parse_term_expression(context, env, tokens, position)?;

    'parse_expr: while is_bitwise_shift_operator(tokens, position) {
        let operator = &tokens[*position];

        // Consume `<<` or `>>` operator
        *position += 1;

        let rhs = parse_term_expression(context, env, tokens, position)?;
        let lhs_type = lhs.expr_type();
        let rhs_type = rhs.expr_type();

        // Parse and Check sides for `<<` operator
        if operator.kind == TokenKind::BitwiseRightShift {
            let expected_rhs_types = lhs_type.can_perform_shr_op_with();

            // Can perform this operator between LHS and RHS
            if expected_rhs_types.contains(&rhs_type) {
                lhs = Box::new(BitwiseExpr {
                    left: lhs,
                    operator: BinaryBitwiseOperator::RightShift,
                    right: rhs,
                    result_type: rhs_type.shr_op_result_type(&rhs_type),
                });

                continue 'parse_expr;
            }

            // Check if RHS expr can be implicit casted to Expected LHS type to make this
            // Expression valid
            for expected_type in expected_rhs_types.iter() {
                if !expected_type.has_implicit_cast_from(&rhs) {
                    continue;
                }

                let casting = Box::new(CastExpr {
                    value: rhs,
                    result_type: expected_type.clone(),
                });

                lhs = Box::new(BitwiseExpr {
                    left: lhs,
                    operator: BinaryBitwiseOperator::RightShift,
                    right: casting,
                    result_type: lhs_type.shr_op_result_type(expected_type),
                });

                continue 'parse_expr;
            }

            // Check if LHS expr can be implicit casted to Expected RHS type to make this
            // Expression valid
            let expected_lhs_types = rhs_type.can_perform_shr_op_with();
            for expected_type in expected_lhs_types.iter() {
                if !expected_type.has_implicit_cast_from(&lhs) {
                    continue;
                }

                let casting = Box::new(CastExpr {
                    value: lhs,
                    result_type: expected_type.clone(),
                });

                lhs = Box::new(BitwiseExpr {
                    left: casting,
                    operator: BinaryBitwiseOperator::RightShift,
                    right: rhs,
                    result_type: rhs_type.shr_op_result_type(expected_type),
                });

                continue 'parse_expr;
            }

            // Return error if this operator can't be performed even with implicit cast
            return Err(Diagnostic::error(&format!(
                "Operator `>>` can't be performed between types `{lhs_type}` and `{rhs_type}`"
            ))
            .with_location(operator.location)
            .as_boxed());
        }

        // Parse and Check sides for `>>` operator
        if operator.kind == TokenKind::BitwiseLeftShift {
            let expected_rhs_types = lhs_type.can_perform_shl_op_with();

            // Can perform this operator between LHS and RHS
            if expected_rhs_types.contains(&rhs_type) {
                lhs = Box::new(BitwiseExpr {
                    left: lhs,
                    operator: BinaryBitwiseOperator::LeftShift,
                    right: rhs,
                    result_type: lhs_type.shl_op_result_type(&rhs_type),
                });

                continue 'parse_expr;
            }

            // Check if RHS expr can be implicit casted to Expected LHS type to make this
            // Expression valid
            for expected_type in expected_rhs_types.iter() {
                if !expected_type.has_implicit_cast_from(&rhs) {
                    continue;
                }

                let casting = Box::new(CastExpr {
                    value: rhs,
                    result_type: expected_type.clone(),
                });

                lhs = Box::new(BitwiseExpr {
                    left: lhs,
                    operator: BinaryBitwiseOperator::LeftShift,
                    right: casting,
                    result_type: lhs_type.shr_op_result_type(expected_type),
                });

                continue 'parse_expr;
            }

            // Check if LHS expr can be implicit casted to Expected RHS type to make this
            // Expression valid
            let expected_lhs_types = rhs_type.can_perform_shr_op_with();
            for expected_type in expected_lhs_types.iter() {
                if !expected_type.has_implicit_cast_from(&lhs) {
                    continue;
                }

                let casting = Box::new(CastExpr {
                    value: lhs,
                    result_type: expected_type.clone(),
                });

                lhs = Box::new(BitwiseExpr {
                    left: casting,
                    operator: BinaryBitwiseOperator::LeftShift,
                    right: rhs,
                    result_type: rhs_type.shr_op_result_type(expected_type),
                });

                continue 'parse_expr;
            }

            // Return error if this operator can't be performed even with implicit cast
            return Err(Diagnostic::error(&format!(
                "Operator `<<` can't be performed between types `{lhs_type}` and `{rhs_type}`"
            ))
            .with_location(operator.location)
            .as_boxed());
        }
    }

    Ok(lhs)
}

fn parse_term_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Expr>, Box<Diagnostic>> {
    let mut lhs = parse_factor_expression(context, env, tokens, position)?;

    'parse_expr: while *position < tokens.len() && is_term_operator(&tokens[*position]) {
        let operator = &tokens[*position];

        // Consume `+` or `-` operator
        *position += 1;

        let rhs = parse_factor_expression(context, env, tokens, position)?;

        let lhs_type = lhs.expr_type();
        let rhs_type = rhs.expr_type();

        // Parse and Check sides for `+` operator
        if operator.kind == TokenKind::Plus {
            let expected_rhs_types = lhs_type.can_perform_add_op_with();

            // Can perform this operator between LHS and RHS
            if expected_rhs_types.contains(&rhs_type) {
                lhs = Box::new(ArithmeticExpr {
                    left: lhs,
                    operator: ArithmeticOperator::Plus,
                    right: rhs,
                    result_type: lhs_type.add_op_result_type(&rhs_type),
                });

                continue 'parse_expr;
            }

            // Check if RHS expr can be implicit casted to Expected LHS type to make this
            // Expression valid
            for expected_type in expected_rhs_types.iter() {
                if !expected_type.has_implicit_cast_from(&rhs) {
                    continue;
                }

                let casting = Box::new(CastExpr {
                    value: rhs,
                    result_type: expected_type.clone(),
                });

                lhs = Box::new(ArithmeticExpr {
                    left: lhs,
                    operator: ArithmeticOperator::Plus,
                    right: casting,
                    result_type: lhs_type.add_op_result_type(expected_type),
                });

                continue 'parse_expr;
            }

            // Check if LHS expr can be implicit casted to Expected RHS type to make this
            // Expression valid
            let expected_lhs_types = rhs_type.can_perform_add_op_with();
            for expected_type in expected_lhs_types.iter() {
                if !expected_type.has_implicit_cast_from(&lhs) {
                    continue;
                }

                let casting = Box::new(CastExpr {
                    value: lhs,
                    result_type: expected_type.clone(),
                });

                lhs = Box::new(ArithmeticExpr {
                    left: casting,
                    operator: ArithmeticOperator::Plus,
                    right: rhs,
                    result_type: rhs_type.add_op_result_type(expected_type),
                });

                continue 'parse_expr;
            }

            // Return error if this operator can't be performed even with implicit cast
            return Err(Diagnostic::error(&format!(
                "Operator `+` can't be performed between types `{lhs_type}` and `{rhs_type}`"
            ))
            .add_help(
                "You can use `CONCAT(Any, Any, ...Any)` function to concatenate values with different types",
            )
            .with_location(operator.location)
            .as_boxed());
        }

        // Parse and Check sides for `-` operator
        if operator.kind == TokenKind::Minus {
            let expected_rhs_types = lhs_type.can_perform_sub_op_with();

            // Can perform this operator between LHS and RHS
            if expected_rhs_types.contains(&rhs_type) {
                lhs = Box::new(ArithmeticExpr {
                    left: lhs,
                    operator: ArithmeticOperator::Minus,
                    right: rhs,
                    result_type: lhs_type.sub_op_result_type(&rhs_type),
                });
                continue 'parse_expr;
            }

            // Check if RHS expr can be implicit casted to Expected LHS type to make this
            // Expression valid
            for expected_type in expected_rhs_types.iter() {
                if !expected_type.has_implicit_cast_from(&rhs) {
                    continue;
                }

                let casting = Box::new(CastExpr {
                    value: rhs,
                    result_type: expected_type.clone(),
                });

                lhs = Box::new(ArithmeticExpr {
                    left: lhs,
                    operator: ArithmeticOperator::Minus,
                    right: casting,
                    result_type: lhs_type.sub_op_result_type(expected_type),
                });

                continue 'parse_expr;
            }

            // Check if LHS expr can be implicit casted to Expected RHS type to make this
            // Expression valid
            let expected_lhs_types = rhs_type.can_perform_sub_op_with();
            for expected_type in expected_lhs_types.iter() {
                if !expected_type.has_implicit_cast_from(&lhs) {
                    continue;
                }

                let casting = Box::new(CastExpr {
                    value: lhs,
                    result_type: expected_type.clone(),
                });

                lhs = Box::new(ArithmeticExpr {
                    left: casting,
                    operator: ArithmeticOperator::Minus,
                    right: rhs,
                    result_type: rhs_type.sub_op_result_type(expected_type),
                });

                continue 'parse_expr;
            }

            // Return error if this operator can't be performed even with implicit cast
            return Err(Diagnostic::error(&format!(
                "Operator `-` can't be performed between types `{lhs_type}` and `{rhs_type}`"
            ))
            .with_location(operator.location)
            .as_boxed());
        }
    }

    Ok(lhs)
}

fn parse_factor_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Expr>, Box<Diagnostic>> {
    let mut lhs = parse_like_expression(context, env, tokens, position)?;

    'parse_expr: while is_factor_operator(tokens, position) {
        let operator = &tokens[*position];

        // Consume `*`, '/`, '%' or '^` operator
        *position += 1;

        let rhs = parse_like_expression(context, env, tokens, position)?;

        let lhs_type = lhs.expr_type();
        let rhs_type = rhs.expr_type();

        // Parse and Check sides for `*` operator
        if operator.kind == TokenKind::Star {
            let expected_rhs_types = lhs_type.can_perform_mul_op_with();

            // Can perform this operator between LHS and RHS
            if expected_rhs_types.contains(&rhs_type) {
                lhs = Box::new(ArithmeticExpr {
                    left: lhs,
                    operator: ArithmeticOperator::Star,
                    right: rhs,
                    result_type: lhs_type.mul_op_result_type(&rhs_type),
                });

                continue 'parse_expr;
            }

            // Check if RHS expr can be implicit casted to Expected LHS type to make this
            // Expression valid
            for expected_type in expected_rhs_types.iter() {
                if !expected_type.has_implicit_cast_from(&rhs) {
                    continue;
                }

                let casting = Box::new(CastExpr {
                    value: rhs,
                    result_type: expected_type.clone(),
                });

                lhs = Box::new(ArithmeticExpr {
                    left: lhs,
                    operator: ArithmeticOperator::Star,
                    right: casting,
                    result_type: lhs_type.mul_op_result_type(expected_type),
                });

                continue 'parse_expr;
            }

            // Check if LHS expr can be implicit casted to Expected RHS type to make this
            // Expression valid
            let expected_lhs_types = rhs_type.can_perform_mul_op_with();
            for expected_type in expected_lhs_types.iter() {
                if !expected_type.has_implicit_cast_from(&lhs) {
                    continue;
                }

                let casting = Box::new(CastExpr {
                    value: lhs,
                    result_type: expected_type.clone(),
                });

                lhs = Box::new(ArithmeticExpr {
                    left: casting,
                    operator: ArithmeticOperator::Star,
                    right: rhs,
                    result_type: rhs_type.mul_op_result_type(expected_type),
                });

                continue 'parse_expr;
            }

            // Return error if this operator can't be performed even with implicit cast
            return Err(Diagnostic::error(&format!(
                "Operator `*` can't be performed between types `{lhs_type}` and `{rhs_type}`"
            ))
            .with_location(operator.location)
            .as_boxed());
        }

        // Parse and Check sides for `/` operator
        if operator.kind == TokenKind::Slash {
            let expected_rhs_types = lhs_type.can_perform_div_op_with();

            // Can perform this operator between LHS and RHS
            if expected_rhs_types.contains(&rhs_type) {
                lhs = Box::new(ArithmeticExpr {
                    left: lhs,
                    operator: ArithmeticOperator::Slash,
                    right: rhs,
                    result_type: lhs_type.div_op_result_type(&rhs_type),
                });

                continue 'parse_expr;
            }

            // Check if RHS expr can be implicit casted to Expected LHS type to make this
            // Expression valid
            for expected_type in expected_rhs_types.iter() {
                if !expected_type.has_implicit_cast_from(&rhs) {
                    continue;
                }

                let casting = Box::new(CastExpr {
                    value: rhs,
                    result_type: expected_type.clone(),
                });

                lhs = Box::new(ArithmeticExpr {
                    left: lhs,
                    operator: ArithmeticOperator::Slash,
                    right: casting,
                    result_type: lhs_type.div_op_result_type(expected_type),
                });

                continue 'parse_expr;
            }

            // Check if LHS expr can be implicit casted to Expected RHS type to make this
            // Expression valid
            let expected_lhs_types = rhs_type.can_perform_div_op_with();
            for expected_type in expected_lhs_types.iter() {
                if !expected_type.has_implicit_cast_from(&lhs) {
                    continue;
                }

                let casting = Box::new(CastExpr {
                    value: lhs,
                    result_type: expected_type.clone(),
                });

                lhs = Box::new(ArithmeticExpr {
                    left: casting,
                    operator: ArithmeticOperator::Slash,
                    right: rhs,
                    result_type: rhs_type.div_op_result_type(expected_type),
                });

                continue 'parse_expr;
            }

            // Return error if this operator can't be performed even with implicit cast
            return Err(Diagnostic::error(&format!(
                "Operator `/` can't be performed between types `{lhs_type}` and `{rhs_type}`"
            ))
            .with_location(operator.location)
            .as_boxed());
        }

        // Parse and Check sides for `%` operator
        if operator.kind == TokenKind::Percentage {
            let expected_rhs_types = lhs_type.can_perform_rem_op_with();

            // Can perform this operator between LHS and RHS
            if expected_rhs_types.contains(&rhs_type) {
                lhs = Box::new(ArithmeticExpr {
                    left: lhs,
                    operator: ArithmeticOperator::Modulus,
                    right: rhs,
                    result_type: lhs_type.rem_op_result_type(&rhs_type),
                });

                continue 'parse_expr;
            }

            // Check if RHS expr can be implicit casted to Expected LHS type to make this
            // Expression valid
            for expected_type in expected_rhs_types.iter() {
                if !expected_type.has_implicit_cast_from(&rhs) {
                    continue;
                }

                let casting = Box::new(CastExpr {
                    value: rhs,
                    result_type: expected_type.clone(),
                });

                lhs = Box::new(ArithmeticExpr {
                    left: lhs,
                    operator: ArithmeticOperator::Modulus,
                    right: casting,
                    result_type: lhs_type.rem_op_result_type(expected_type),
                });

                continue 'parse_expr;
            }

            // Check if LHS expr can be implicit casted to Expected RHS type to make this
            // Expression valid
            let expected_lhs_types = rhs_type.can_perform_rem_op_with();
            for expected_type in expected_lhs_types.iter() {
                if !expected_type.has_implicit_cast_from(&lhs) {
                    continue;
                }

                let casting = Box::new(CastExpr {
                    value: lhs,
                    result_type: expected_type.clone(),
                });

                lhs = Box::new(ArithmeticExpr {
                    left: casting,
                    operator: ArithmeticOperator::Modulus,
                    right: rhs,
                    result_type: rhs_type.rem_op_result_type(expected_type),
                });

                continue 'parse_expr;
            }

            // Return error if this operator can't be performed even with implicit cast
            return Err(Diagnostic::error(&format!(
                "Operator `%` can't be performed between types `{lhs_type}` and `{rhs_type}`"
            ))
            .with_location(operator.location)
            .as_boxed());
        }

        // Parse and Check sides for `^` operator
        if operator.kind == TokenKind::Caret {
            let expected_rhs_types = lhs_type.can_perform_caret_op_with();

            if expected_rhs_types.contains(&rhs_type) {
                lhs = Box::new(ArithmeticExpr {
                    left: lhs,
                    operator: ArithmeticOperator::Exponentiation,
                    right: rhs,
                    result_type: lhs_type.caret_op_result_type(&rhs_type),
                });

                continue 'parse_expr;
            }

            // Check if RHS expr can be implicit casted to Expected LHS type to make this
            // Expression valid
            for expected_type in expected_rhs_types.iter() {
                if !expected_type.has_implicit_cast_from(&rhs) {
                    continue;
                }

                let casting = Box::new(CastExpr {
                    value: rhs,
                    result_type: expected_type.clone(),
                });

                lhs = Box::new(ArithmeticExpr {
                    left: lhs,
                    operator: ArithmeticOperator::Exponentiation,
                    right: casting,
                    result_type: lhs_type.caret_op_result_type(expected_type),
                });

                continue 'parse_expr;
            }

            // Check if LHS expr can be implicit casted to Expected RHS type to make this
            // Expression valid
            let expected_lhs_types = rhs_type.can_perform_caret_op_with();
            for expected_type in expected_lhs_types.iter() {
                if !expected_type.has_implicit_cast_from(&lhs) {
                    continue;
                }

                let casting = Box::new(CastExpr {
                    value: lhs,
                    result_type: expected_type.clone(),
                });

                lhs = Box::new(ArithmeticExpr {
                    left: casting,
                    operator: ArithmeticOperator::Exponentiation,
                    right: rhs,
                    result_type: rhs_type.caret_op_result_type(expected_type),
                });

                continue 'parse_expr;
            }

            // Return error if this operator can't be performed even with implicit cast
            return Err(Diagnostic::error(&format!(
                "Operator `^` can't be performed between types `{lhs_type}` and `{rhs_type}`"
            ))
            .with_location(operator.location)
            .as_boxed());
        }
    }

    Ok(lhs)
}

fn parse_like_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Expr>, Box<Diagnostic>> {
    let lhs = parse_glob_expression(context, env, tokens, position)?;

    // Check for `LIKE` or `NOT LIKE`
    // <expr> LIKE <expr> AND <expr>
    // <expr> NOT LIKE <expr> AND <expr>
    if is_current_token(tokens, position, TokenKind::Like)
        || (is_current_token(tokens, position, TokenKind::Not)
            && is_next_token(tokens, position, TokenKind::Like))
    {
        let has_not_keyword = is_current_token(tokens, position, TokenKind::Not);
        let operator_location: SourceLocation = if has_not_keyword {
            // Consume `NOT` and `LIKE` keyword
            *position += 2;
            let mut not_location = tokens[*position - 2].location;
            let between_location = tokens[*position - 1].location;
            not_location.expand_until(between_location);
            not_location
        } else {
            // Consume `LIKE` keyword
            *position += 1;
            tokens[*position - 1].location
        };

        let pattern = parse_glob_expression(context, env, tokens, position)?;

        let lhs_type = lhs.expr_type();
        let rhs_type = pattern.expr_type();

        // Can perform this operator between LHS and RHS
        let expected_rhs_types = lhs_type.can_perform_like_op_with();
        if expected_rhs_types.contains(&rhs_type) {
            let expr = Box::new(LikeExpr {
                input: lhs,
                pattern,
            });

            return Ok(apply_not_keyword_if_exists(expr, has_not_keyword));
        }

        // Check if RHS expr can be implicit casted to Expected LHS type to make this
        // Expression valid
        for expected_type in expected_rhs_types.iter() {
            if !expected_type.has_implicit_cast_from(&pattern) {
                continue;
            }

            let casting = Box::new(CastExpr {
                value: pattern,
                result_type: expected_type.clone(),
            });

            let expr = Box::new(LikeExpr {
                input: lhs,
                pattern: casting,
            });

            return Ok(apply_not_keyword_if_exists(expr, has_not_keyword));
        }

        // Return error if this operator can't be performed even with implicit cast
        return Err(Diagnostic::error(&format!(
            "Operator `LIKE` can't be performed between types `{lhs_type}` and `{rhs_type}`"
        ))
        .with_location(operator_location)
        .as_boxed());
    }

    Ok(lhs)
}

fn parse_glob_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Expr>, Box<Diagnostic>> {
    let lhs = parse_cast_operator_expression(context, env, tokens, position)?;

    if is_current_token(tokens, position, TokenKind::Glob) {
        let glob_location = tokens[*position].location;

        // Consume `GLOB` Token
        *position += 1;

        let pattern = parse_cast_operator_expression(context, env, tokens, position)?;

        let lhs_type = lhs.expr_type();
        let rhs_type = pattern.expr_type();

        // Can perform this operator between LHS and RHS
        let expected_rhs_types = lhs_type.can_perform_glob_op_with();
        if expected_rhs_types.contains(&rhs_type) {
            return Ok(Box::new(GlobExpr {
                input: lhs,
                pattern,
            }));
        }

        // Check if RHS expr can be implicit casted to Expected LHS type to make this
        // Expression valid
        for expected_type in expected_rhs_types.iter() {
            if !expected_type.has_implicit_cast_from(&pattern) {
                continue;
            }

            let casting = Box::new(CastExpr {
                value: pattern,
                result_type: expected_type.clone(),
            });

            return Ok(Box::new(GlobExpr {
                input: lhs,
                pattern: casting,
            }));
        }

        // Return error if this operator can't be performed even with implicit cast
        return Err(Diagnostic::error(&format!(
            "Operator `GLOB` can't be performed between types `{lhs_type}` and `{rhs_type}`"
        ))
        .with_location(glob_location)
        .as_boxed());
    }

    Ok(lhs)
}

pub(crate) fn parse_index_or_slice_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Expr>, Box<Diagnostic>> {
    let mut lhs = parse_prefix_unary_expression(context, env, tokens, position)?;

    'parse_expr: while *position < tokens.len() && tokens[*position].kind == TokenKind::LeftBracket
    {
        let operator = &tokens[*position];

        // Consume Left Bracket `[`
        *position += 1;

        let lhs_type = lhs.expr_type();

        // Slice with end only range [:end]
        if is_current_token(tokens, position, TokenKind::Colon) {
            // Consume Colon `:`
            *position += 1;

            // In case the user use default slice start and end, we can ignore the slice expression
            // and return array or any kind of expression value directly
            if is_current_token(tokens, position, TokenKind::RightBracket) {
                // Consume right bracket `]`
                *position += 1;
                return Ok(lhs);
            }

            let slice_end = parse_prefix_unary_expression(context, env, tokens, position)?;
            let end_type = slice_end.expr_type();

            // Check if LHS already support slice op
            if !lhs_type.can_perform_slice_op() {
                return Err(Diagnostic::error(&format!(
                    "Operator `[:]` can't be performed on type `{lhs_type}`"
                ))
                .with_location(calculate_safe_location(tokens, *position))
                .as_boxed());
            }

            // Check that LHS support slice op with this type
            let rhs_expected_types = lhs_type.can_perform_slice_op_with();
            if !rhs_expected_types.contains(&end_type) {
                return Err(Diagnostic::error(&format!(
                    "Operator `[:]` can't be performed with type of index `{}`",
                    end_type.literal()
                ))
                .with_location(calculate_safe_location(tokens, *position))
                .as_boxed());
            }

            // Consume Right Bracket `]`
            consume_token_or_error(
                tokens,
                position,
                TokenKind::RightBracket,
                "Expect `]` After Slice expression",
            )?;

            lhs = Box::new(SliceExpr {
                collection: lhs,
                start: None,
                end: Some(slice_end),
                result_type: lhs_type.clone(),
            });

            continue 'parse_expr;
        }

        let index = parse_prefix_unary_expression(context, env, tokens, position)?;
        let index_type = index.expr_type();

        // Slice Expression with Start and End range [start:end]
        if is_current_token(tokens, position, TokenKind::Colon) {
            // Consume Colon `:`
            *position += 1;

            // Slice with start only range [start:]
            if is_current_token(tokens, position, TokenKind::RightBracket) {
                // Consume Right Bracket `]`
                *position += 1;

                let rhs_expected_types = lhs_type.can_perform_slice_op_with();
                if rhs_expected_types.contains(&index_type) {
                    return Ok(Box::new(SliceExpr {
                        collection: lhs,
                        start: Some(index),
                        end: None,
                        result_type: lhs_type.clone(),
                    }));
                }

                return Err(Diagnostic::error(&format!(
                    "Operator Slice `[:]` can't be performed between on {} with start `{}` and end `{}`",
                    lhs_type,
                    index_type.literal(),
                    "None"
                ))
                .with_location(operator.location)
                .as_boxed());
            }

            let slice_end = parse_prefix_unary_expression(context, env, tokens, position)?;
            let end_type = slice_end.expr_type();

            // Make sure slice start and end types are equals
            if !index_type.equals(&end_type) {
                return Err(Diagnostic::error(&format!(
                    "Operator Slice `[:]` start and end types must be equals but found `{}` and  `{}`",
                    index_type.literal(),
                    end_type.literal()
                ))
                .with_location(operator.location)
                .as_boxed());
            }

            let rhs_expected_types = lhs_type.can_perform_slice_op_with();
            if !rhs_expected_types.contains(&end_type) {
                return Err(Diagnostic::error(&format!(
                    "Operator Slice `[:]` can't be performed between on {} with start `{}` and end `{}`",
                    lhs_type,
                    index_type.literal(),
                    end_type.literal()
                ))
                .with_location(operator.location)
                .as_boxed());
            }

            // Consume Right Bracket `]`
            consume_token_or_error(
                tokens,
                position,
                TokenKind::RightBracket,
                "Expect `]` After Slice expression",
            )?;

            lhs = Box::new(SliceExpr {
                collection: lhs,
                start: Some(index),
                end: Some(slice_end),
                result_type: lhs_type.clone(),
            });

            continue 'parse_expr;
        }

        // Index Expression
        let rhs_expected_types = lhs_type.can_perform_index_op_with();
        if !rhs_expected_types.contains(&index_type) {
            return Err(Diagnostic::error(&format!(
                "Operator Index `[ ]` can't be performed on type `{}` with index type `{}`",
                lhs_type,
                index_type.literal(),
            ))
            .add_help("Check the Type documentation to know the available Index types")
            .with_location(operator.location)
            .as_boxed());
        }

        // Consume Right Bracket `]`
        consume_token_or_error(
            tokens,
            position,
            TokenKind::RightBracket,
            "Expect `]` After Index expression",
        )?;

        let array_element_type =
            if let Some(array_type) = lhs_type.as_any().downcast_ref::<ArrayType>() {
                array_type.base.clone()
            } else {
                Box::new(AnyType)
            };

        let result_type = lhs_type.index_op_result_type(&index_type);

        lhs = Box::new(IndexExpr {
            collection: lhs,
            element_type: array_element_type.clone(),
            index,
            result_type,
        });

        continue 'parse_expr;
    }

    Ok(lhs)
}

fn parse_prefix_unary_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Expr>, Box<Diagnostic>> {
    if *position < tokens.len() && is_prefix_unary_operator(&tokens[*position]) {
        let operator = &tokens[*position];

        // Consume `!`, `-` or `~` operator
        *position += 1;

        let rhs = parse_prefix_unary_expression(context, env, tokens, position)?;
        let rhs_type = rhs.expr_type();

        // Parse and Check side for unary `!`or `NOT` operator
        if operator.kind == TokenKind::Bang || operator.kind == TokenKind::Not {
            // Can perform this operator between RHS
            if rhs_type.can_perform_bang_op() {
                return Ok(Box::new(UnaryExpr {
                    right: rhs,
                    operator: PrefixUnaryOperator::Bang,
                    result_type: rhs_type.bang_op_result_type(),
                }));
            }

            // Return error if this operator can't be performed even with implicit cast
            let op_name = if operator.kind == TokenKind::Bang {
                "!"
            } else {
                "NOT"
            };

            return Err(Diagnostic::error(&format!(
                "Operator unary `{op_name}` can't be performed on type `{rhs_type}`"
            ))
            .with_location(operator.location)
            .as_boxed());
        }

        // Parse and Check side for unary `-` operator
        if operator.kind == TokenKind::Minus {
            // Can perform this operator between RHS
            if rhs_type.can_perform_neg_op() {
                return Ok(Box::new(UnaryExpr {
                    right: rhs,
                    operator: PrefixUnaryOperator::Negative,
                    result_type: rhs_type.neg_op_result_type(),
                }));
            }

            // Return error if this operator can't be performed even with implicit cast
            return Err(Diagnostic::error(&format!(
                "Operator unary `-` can't be performed on type `{rhs_type}`"
            ))
            .with_location(operator.location)
            .as_boxed());
        }

        // Parse and Check side for unary `~` operator
        if operator.kind == TokenKind::BitwiseNot {
            // Can perform this operator between RHS
            if rhs_type.can_perform_not_op() {
                return Ok(Box::new(UnaryExpr {
                    right: rhs,
                    operator: PrefixUnaryOperator::Not,
                    result_type: rhs_type.not_op_result_type(),
                }));
            }

            // Return error if this operator can't be performed even with implicit cast
            return Err(Diagnostic::error(&format!(
                "Operator unary `~` can't be performed on type `{rhs_type}`"
            ))
            .with_location(operator.location)
            .as_boxed());
        }
    }

    parse_between_expression(context, env, tokens, position)
}

fn parse_between_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Expr>, Box<Diagnostic>> {
    let expression = parse_function_call_expression(context, env, tokens, position)?;

    // Check for `BETWEEN` or `NOT BETWEEN`
    // <expr> BETWEEN <expr> AND <expr>
    // <expr> NOT BETWEEN <expr> AND <expr>
    if is_current_token(tokens, position, TokenKind::Between)
        || (is_current_token(tokens, position, TokenKind::Not)
            && is_next_token(tokens, position, TokenKind::Between))
    {
        let has_not_keyword = is_current_token(tokens, position, TokenKind::Not);
        let operator_location = if has_not_keyword {
            // Consume `NOT` and `BETWEEN` keyword
            *position += 2;
            let mut not_location = tokens[*position - 2].location;
            let between_location = tokens[*position - 1].location;
            not_location.expand_until(between_location);
            not_location
        } else {
            // Consume `BETWEEN` keyword
            *position += 1;
            tokens[*position - 1].location
        };

        let kind = parse_between_expr_kind(tokens, position);
        let range_start = parse_function_call_expression(context, env, tokens, position)?;

        // Consume `AND` token
        consume_token_or_error(
            tokens,
            position,
            TokenKind::AndKeyword,
            "Expect `AND` after `BETWEEN` range start",
        )?;

        let range_end = parse_function_call_expression(context, env, tokens, position)?;

        let lhs_type = expression.expr_type();
        let range_start_type = &range_start.expr_type();
        let range_end_type = &range_end.expr_type();

        // Make sure LHS and Range start and end types all are equals
        if !lhs_type.equals(range_start_type) || !lhs_type.equals(range_end_type) {
            return Err(Diagnostic::error(&format!(
                "Expect `BETWEEN` Left hand side type, range start and end to has same type but got {lhs_type}, {} and {}",
                range_start_type.literal(),
                range_end_type.literal()
            ))
            .add_help("Try to make sure all of them has same type")
            .with_location(operator_location)
            .as_boxed());
        }

        // Make sure that type is supporting >= operator
        if !lhs_type.can_perform_gte_op_with().contains(&lhs_type) {
            return Err(Diagnostic::error(&format!(
                "Type `{lhs_type}` used in Between expression can't support `>=` operator"
            ))
            .with_location(operator_location)
            .as_boxed());
        }

        // Make sure that type is supporting <= operator
        if !lhs_type.can_perform_lte_op_with().contains(&lhs_type) {
            return Err(Diagnostic::error(&format!(
                "Type `{lhs_type}` used in Between expression can't support `<=` operator"
            ))
            .with_location(operator_location)
            .as_boxed());
        }

        let between_expr = Box::new(BetweenExpr {
            value: expression,
            range_start,
            range_end,
            kind,
        });

        return Ok(apply_not_keyword_if_exists(between_expr, has_not_keyword));
    }

    Ok(expression)
}

fn parse_between_expr_kind(tokens: &[Token], position: &mut usize) -> BetweenKind {
    if *position < tokens.len() {
        let token_kind = &tokens[*position].kind;
        if matches!(token_kind, TokenKind::Asymmetric | TokenKind::Symmetric) {
            *position += 1;
            return if token_kind == &TokenKind::Asymmetric {
                BetweenKind::Asymmetric
            } else {
                BetweenKind::Symmetric
            };
        }
    }
    BetweenKind::Asymmetric
}

pub(crate) fn parse_zero_or_more_values_with_comma_between(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
    expression_name: &str,
) -> Result<Vec<Box<dyn Expr>>, Box<Diagnostic>> {
    // Consume `(` token at the start of list of values
    consume_token_or_error(
        tokens,
        position,
        TokenKind::LeftParen,
        &format!("Expect `(` after {expression_name}"),
    )?;

    let mut arguments: Vec<Box<dyn Expr>> = vec![];
    while *position < tokens.len() && tokens[*position].kind != TokenKind::RightParen {
        let argument = parse_expression(context, env, tokens, position)?;
        if let Some(argument_literal) = expression_literal(&argument) {
            context.hidden_selections.push(argument_literal);
        }

        arguments.push(argument);

        if *position < tokens.len() && tokens[*position].kind == TokenKind::Comma {
            *position += 1;
        } else {
            break;
        }
    }

    // Consume `)` token at the end of values with comma betweens
    consume_token_or_error(
        tokens,
        position,
        TokenKind::RightParen,
        "Expect `)` at the end of zero or more values",
    )?;

    Ok(arguments)
}

pub(crate) fn parse_member_access_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Expr>, Box<Diagnostic>> {
    let expr = parse_primary_expression(context, env, tokens, position)?;

    if is_current_token(tokens, position, TokenKind::Dot) {
        let dot_token = &tokens[*position];

        // The syntax for member access is (composite).member
        // The composite expr must be in group, that make it different from table access table.name
        if expr.kind() != ExprKind::Grouping {
            return Err(
                Diagnostic::error("Dot token expect composite value between `(` and `)`")
                    .add_note("The syntax for accessing composite element is (composite).member")
                    .with_location(dot_token.location)
                    .as_boxed(),
            );
        }

        // Consume `.` token
        *position += 1;

        // select (user).name from users
        if let Some(composite_type) = expr.expr_type().as_any().downcast_ref::<CompositeType>() {
            if *position < tokens.len() && matches!(tokens[*position].kind, TokenKind::Symbol(_)) {
                let member_token = &tokens[*position];

                // Consume `Symbol` token
                *position += 1;

                let member_name = member_token.to_string();

                // Make sure the member name is valid for this composite type
                if !composite_type.members.contains_key(&member_name) {
                    return Err(Diagnostic::error(&format!(
                        "Compose type {} has no member with name {}",
                        composite_type.name, member_name
                    ))
                    .add_help("Check the Composite type to see what are his members")
                    .with_location(member_token.location)
                    .as_boxed());
                }

                let member_type = composite_type.members.get(&member_name).unwrap().clone();
                return Ok(Box::new(MemberAccessExpr {
                    composite: expr,
                    member_name,
                    member_type,
                }));
            }

            // Member Access expect symbol as member name
            return Err(Diagnostic::error("Member access expect symbol after `.`")
                .add_note("The syntax for accessing composite element is (composite).member")
                .with_location(dot_token.location)
                .as_boxed());
        }

        // Make sure the expression type in group expression is Composite type
        return Err(Diagnostic::error(&format!(
            "Member access expect Composite type between `(` and `)` but got {}",
            expr.expr_type().literal()
        ))
        .with_location(dot_token.location)
        .as_boxed());
    }

    Ok(expr)
}

fn parse_primary_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Expr>, Box<Diagnostic>> {
    if *position >= tokens.len() {
        return Err(un_expected_expression_error(tokens, position));
    }

    match &tokens[*position].kind {
        TokenKind::Integer(_) => parse_const_integer_expression(tokens, position),
        TokenKind::Float(_) => parse_const_float_expression(tokens, position),
        TokenKind::Infinity => parse_float_infinity_or_nan_expression(tokens, position),
        TokenKind::NaN => parse_float_infinity_or_nan_expression(tokens, position),
        TokenKind::Symbol(_) => parse_symbol_expression(context, env, tokens, position),
        TokenKind::Array => parse_array_value_expression(context, env, tokens, position),
        TokenKind::LeftBracket => parse_array_value_expression(context, env, tokens, position),
        TokenKind::LeftParen => parse_group_expression(context, env, tokens, position),
        TokenKind::Case => parse_case_expression(context, env, tokens, position),
        TokenKind::Cast => parse_cast_call_expression(context, env, tokens, position),
        TokenKind::Benchmark => parse_benchmark_call_expression(context, env, tokens, position),
        TokenKind::GlobalVariable(_) => parse_global_variable_expression(env, tokens, position),
        TokenKind::Interval => parse_interval_expression(tokens, position),
        TokenKind::String(str) => {
            *position += 1;
            let value = str.to_string();
            Ok(Box::new(StringExpr { value }))
        }
        TokenKind::True => {
            *position += 1;
            Ok(Box::new(BooleanExpr { is_true: true }))
        }
        TokenKind::False => {
            *position += 1;
            Ok(Box::new(BooleanExpr { is_true: false }))
        }
        TokenKind::Null => {
            *position += 1;
            Ok(Box::new(NullExpr {}))
        }
        _ => Err(un_expected_expression_error(tokens, position)),
    }
}

fn parse_const_integer_expression(
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Expr>, Box<Diagnostic>> {
    match tokens[*position].kind {
        TokenKind::Integer(integer) => {
            *position += 1;
            let value = Number::Int(integer);
            Ok(Box::new(NumberExpr { value }))
        }
        _ => Err(Diagnostic::error("Too big Integer value")
            .add_help("Try to use smaller value")
            .add_note(&format!(
                "Integer value must be between {} and {}",
                i64::MIN,
                i64::MAX
            ))
            .with_location(tokens[*position].location)
            .as_boxed()),
    }
}

fn parse_const_float_expression(
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Expr>, Box<Diagnostic>> {
    match tokens[*position].kind {
        TokenKind::Float(float) => {
            *position += 1;
            let value = Number::Float(float);
            Ok(Box::new(NumberExpr { value }))
        }
        _ => Err(Diagnostic::error("Too big Float value")
            .add_help("Try to use smaller value")
            .add_note(&format!(
                "Float value must be between {} and {}",
                f64::MIN,
                f64::MAX
            ))
            .with_location(tokens[*position].location)
            .as_boxed()),
    }
}

fn parse_float_infinity_or_nan_expression(
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Expr>, Box<Diagnostic>> {
    if tokens[*position].kind == TokenKind::Infinity {
        *position += 1;
        let value = Number::Float(f64::INFINITY);
        return Ok(Box::new(NumberExpr { value }));
    }

    *position += 1;

    let value = Number::Float(f64::NAN);
    Ok(Box::new(NumberExpr { value }))
}

fn parse_symbol_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Expr>, Box<Diagnostic>> {
    let mut value = tokens[*position].to_string();
    let location = tokens[*position].location;

    // Collect projections only inside select statement
    if !context.has_select_statement {
        context.projection_names.push(value.to_string());
        context.projection_locations.push(location);

        // If user perform member access with Composite type, composite type name should be in hidden selection
        // For example `SELECT (commit).author_name`, commit should be in hidden selection
        if let Some(symbol_type) = env.schema.tables_fields_types.get(&value.as_str()) {
            if symbol_type.is_composite() && !context.hidden_selections.contains(&value) {
                context.hidden_selections.push(value.to_string());
            }
        }
    }

    // In case of using un selected column name inside OVER(....) clauses, mark it as hidden selection for now
    if context.inside_over_clauses
        && env.schema.tables_fields_types.contains_key(&value.as_str())
        && !context.hidden_selections.contains(&value)
    {
        context.hidden_selections.push(value.to_string());
    }

    if context.has_select_statement {
        // Replace name by alias if it used after select statement
        // This workaround will help to execute query like
        // SELECT commit_count as cc from branches where commit_count > 1
        if context.name_alias_table.contains_key(&value) {
            value = context.name_alias_table[&value].to_string();
        }

        if !env.scopes.contains_key(&value) {
            return Err(Diagnostic::error("Unresolved column or variable name")
                .add_help("Please check schema from docs website or SHOW query")
                .with_location(tokens[*position].location)
                .as_boxed());
        }

        if !context.selected_fields.contains(&value) {
            context.hidden_selections.push(value.to_string());
        }
    }

    let mut symbol_name = &value;

    // If this symbol is alias, resolve it back to the original name and perform the checks
    let has_alias = context.name_alias_table.values().any(|v| v.eq(symbol_name));
    if has_alias {
        for (key, value) in context.name_alias_table.iter() {
            if value.eq(symbol_name) {
                symbol_name = key;
                break;
            }
        }
    }

    // If this symbol is a reference to Aggregate value, make sure it's used in the right place
    if context.aggregations.contains_key(symbol_name)
        && !(context.inside_selections || context.inside_having || context.inside_order_by)
    {
        return Err(Diagnostic::error(
            "Can't use the value of aggregation function outside selection or order by",
        )
        .with_location(calculate_safe_location(tokens, *position))
        .as_boxed());
    }

    // If this symbol is a reference to Window function value, make sure it's used in the right place
    if context.window_functions.contains_key(symbol_name)
        && !(context.inside_selections || context.inside_order_by)
    {
        return Err(Diagnostic::error(
            "Can't use the value of window function outside selection or order by",
        )
        .with_location(calculate_safe_location(tokens, *position))
        .as_boxed());
    }

    // Consume `Symbol` token
    *position += 1;

    let result_type = resolve_symbol_type_or_undefine(env, &value);

    Ok(Box::new(SymbolExpr {
        value,
        expr_type: result_type,
        flag: SymbolFlag::None,
    }))
}

fn parse_array_value_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Expr>, Box<Diagnostic>> {
    // Consume the Optional Array keyword
    if *position < tokens.len() && tokens[*position].kind == TokenKind::Array {
        // Consume Array keyword
        *position += 1;

        // Make sure Array keyword followed by [
        if *position >= tokens.len() || tokens[*position].kind != TokenKind::LeftBracket {
            return Err(Diagnostic::error("Expect `[` after `ARRAY` keyword")
                .with_location(calculate_safe_location(tokens, *position))
                .add_help("Try to add '[' after `ARRAY` keyword")
                .as_boxed());
        }
    }

    // Consume Left Bracket `[`
    *position += 1;

    // Parse array values
    let mut array_values: Vec<Box<dyn Expr>> = vec![];
    let mut array_data_type: Box<dyn DataType> = Box::new(AnyType);
    while *position < tokens.len() && tokens[*position].kind != TokenKind::RightBracket {
        let value = parse_expression(context, env, tokens, position)?;
        let value_type = value.expr_type();
        if !value_type.equals(&array_data_type) {
            return Err(Diagnostic::error("Expect Array values to have same types")
                .with_location(calculate_safe_location(tokens, *position))
                .as_boxed());
        }

        array_data_type = value_type;
        array_values.push(value);

        if *position < tokens.len() && tokens[*position].kind == TokenKind::Comma {
            *position += 1;
        } else {
            break;
        }
    }

    // Make sure Array values end with by ]
    if *position >= tokens.len() || tokens[*position].kind != TokenKind::RightBracket {
        return Err(Diagnostic::error("Expect `]` at the end of array values")
            .with_location(calculate_safe_location(tokens, *position))
            .add_help("Try to add ']' at the end of array values")
            .as_boxed());
    }

    // Consume Right Bracket `]`
    *position += 1;

    Ok(Box::new(ArrayExpr {
        values: array_values,
        element_type: array_data_type,
    }))
}

fn parse_group_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Expr>, Box<Diagnostic>> {
    // Consume '(' token
    *position += 1;

    let expression = parse_expression(context, env, tokens, position)?;
    if tokens[*position].kind != TokenKind::RightParen {
        return Err(Diagnostic::error("Expect `)` to end group expression")
            .with_location(calculate_safe_location(tokens, *position))
            .add_help("Try to add ')' at the end of group expression")
            .as_boxed());
    }

    // Consume ')' token
    *position += 1;

    Ok(Box::new(GroupExpr { expr: expression }))
}

fn parse_case_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Expr>, Box<Diagnostic>> {
    let mut conditions: Vec<Box<dyn Expr>> = vec![];
    let mut values: Vec<Box<dyn Expr>> = vec![];
    let mut default_value: Option<Box<dyn Expr>> = None;

    // Consume `case` keyword
    let case_location = tokens[*position].location;
    *position += 1;

    let mut has_else_branch = false;

    while *position < tokens.len() && tokens[*position].kind != TokenKind::End {
        // Else branch
        if tokens[*position].kind == TokenKind::Else {
            if has_else_branch {
                return Err(
                    Diagnostic::error("This `CASE` expression already has else branch")
                        .add_note("`CASE` expression can has only one `ELSE` branch")
                        .with_location(calculate_safe_location(tokens, *position))
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

        // Consume `WHEN` keyword
        consume_token_or_error(
            tokens,
            position,
            TokenKind::When,
            "Expect `when` before case condition",
        )?;

        let condition = parse_expression(context, env, tokens, position)?;
        if !condition.expr_type().is_bool() {
            return Err(Diagnostic::error("Case condition must be a boolean type")
                .with_location(calculate_safe_location(tokens, *position))
                .as_boxed());
        }

        conditions.push(condition);

        // Consume `THEN` keyword
        consume_token_or_error(
            tokens,
            position,
            TokenKind::Then,
            "Expect `THEN` after case condition",
        )?;

        let expression = parse_expression(context, env, tokens, position)?;
        values.push(expression);
    }

    // Make sure case expression has at least else branch
    if conditions.is_empty() && !has_else_branch {
        return Err(
            Diagnostic::error("Case expression must has at least else branch")
                .with_location(calculate_safe_location(tokens, *position))
                .as_boxed(),
        );
    }

    // Make sure case expression end with END keyword
    if *position >= tokens.len() || tokens[*position].kind != TokenKind::End {
        return Err(Diagnostic::error("Expect `END` after case branches")
            .with_location(calculate_safe_location(tokens, *position))
            .as_boxed());
    }

    // Consume end
    *position += 1;

    // Make sure this case expression has else branch
    if !has_else_branch {
        return Err(Diagnostic::error("Case expression must has else branch")
            .with_location(calculate_safe_location(tokens, *position))
            .as_boxed());
    }

    // Assert that all values has the same type
    let values_type = values[0].expr_type();
    for (i, value) in values.iter().enumerate().skip(1) {
        if !values_type.equals(&value.expr_type()) {
            return Err(Diagnostic::error(&format!(
                "Case value in branch {} has different type than the last branch",
                i + 1
            ))
            .add_note("All values in `CASE` expression must has the same Type")
            .with_location(case_location)
            .as_boxed());
        }
    }

    Ok(Box::new(CaseExpr {
        conditions,
        values,
        default_value,
        values_type,
    }))
}

fn parse_benchmark_call_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Expr>, Box<Diagnostic>> {
    // Consume `BENCHMARK` token
    *position += 1;

    if *position >= tokens.len() || tokens[*position].kind != TokenKind::LeftParen {
        return Err(Diagnostic::error("Expect `(` after `Benchmark` keyword")
            .with_location(calculate_safe_location(tokens, *position))
            .add_help("Try to add '(' right after `Benchmark` keyword")
            .as_boxed());
    }

    // Consume `(` token
    *position += 1;

    let count = parse_expression(context, env, tokens, position)?;
    if !count.expr_type().is_int() {
        return Err(
            Diagnostic::error("Benchmark expect first argument to be integer")
                .with_location(calculate_safe_location(tokens, *position))
                .add_help("Try to integer value as first argument, eg: `Benchmark(10, 1 + 1)`")
                .as_boxed(),
        );
    }

    if *position >= tokens.len() || tokens[*position].kind != TokenKind::Comma {
        return Err(
            Diagnostic::error("Expect `,` after Benchmark first argument value")
                .with_location(calculate_safe_location(tokens, *position))
                .add_help("Make sure you passed two arguments to the Benchmark function")
                .as_boxed(),
        );
    }

    // Consume `,` token
    *position += 1;

    let expression = parse_expression(context, env, tokens, position)?;

    if *position >= tokens.len() || tokens[*position].kind != TokenKind::RightParen {
        return Err(Diagnostic::error("Expect `)` after `Benchmark` arguments")
            .with_location(calculate_safe_location(tokens, *position))
            .add_help("Try to add ')` after `Benchmark` arguments")
            .as_boxed());
    }

    // Consume `)` token
    *position += 1;

    Ok(Box::new(BenchmarkCallExpr { expression, count }))
}

fn parse_global_variable_expression(
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Expr>, Box<Diagnostic>> {
    let name = tokens[*position].to_string();
    *position += 1;
    let result_type = if env.globals_types.contains_key(&name) {
        env.globals_types[name.as_str()].clone()
    } else {
        Box::new(UndefType)
    };
    Ok(Box::new(GlobalVariableExpr { name, result_type }))
}

fn un_expected_query_start_error(tokens: &[Token], position: &mut usize) -> Box<Diagnostic> {
    let token: &Token = &tokens[*position];
    let location = token.location;

    // Query starts with invalid keyword
    if *position == 0 {
        return Diagnostic::error("Unexpected query start")
            .add_help("Expect query to start with `SELECT`, `DO`, `SET` or `DESCRIBE` keyword")
            .with_location(location)
            .as_boxed();
    }

    // General un expected query error
    Diagnostic::error("Unexpected statement")
        .with_location(location)
        .as_boxed()
}

fn un_expected_expression_error(tokens: &[Token], position: &usize) -> Box<Diagnostic> {
    let location = calculate_safe_location(tokens, *position);

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
            .add_help("Try to remove the extra `=`")
            .with_location(location)
            .as_boxed();
    }

    // `> =` the user may want to write `>=`
    if previous.kind == TokenKind::Greater && current.kind == TokenKind::Equal {
        return Diagnostic::error("Unexpected `> =`, do you mean `>=`?")
            .add_help("Try to remove space between `> =`")
            .with_location(location)
            .as_boxed();
    }

    // `< =` the user may want to write `<=`
    if previous.kind == TokenKind::Less && current.kind == TokenKind::Equal {
        return Diagnostic::error("Unexpected `< =`, do you mean `<=`?")
            .add_help("Try to remove space between `< =`")
            .with_location(location)
            .as_boxed();
    }

    // `> >` the user may want to write '>>'
    if previous.kind == TokenKind::Greater && current.kind == TokenKind::Greater {
        return Diagnostic::error("Unexpected `> >`, do you mean `>>`?")
            .add_help("Try to remove space between `> >`")
            .with_location(location)
            .as_boxed();
    }

    // `< <` the user may want to write `<<`
    if previous.kind == TokenKind::Less && current.kind == TokenKind::Less {
        return Diagnostic::error("Unexpected `< <`, do you mean `<<`?")
            .add_help("Try to remove space between `< <`")
            .with_location(location)
            .as_boxed();
    }

    // `< >` the user may want to write `<>`
    if previous.kind == TokenKind::Less && current.kind == TokenKind::Greater {
        return Diagnostic::error("Unexpected `< >`, do you mean `<>`?")
            .add_help("Try to remove space between `< >`")
            .with_location(location)
            .as_boxed();
    }

    // `<= >` the user may want to write `<=>`
    if previous.kind == TokenKind::LessEqual && current.kind == TokenKind::Greater {
        return Diagnostic::error("Unexpected `<= >`, do you mean `<=>`?")
            .add_help("Try to remove space between `<= >`")
            .with_location(location)
            .as_boxed();
    }

    // Default error message
    Diagnostic::error("Can't complete parsing this expression")
        .with_location(location)
        .as_boxed()
}

/// Report error message for extra content after the end of current query
fn un_expected_content_after_correct_query(
    statement_name: &str,
    tokens: &[Token],
    position: &mut usize,
) -> Box<Diagnostic> {
    let error_message = &format!(
        "Unexpected content after the end of `{}` query",
        statement_name.to_uppercase()
    );

    // The range of extra content
    let last_token_location = tokens[tokens.len() - 1].location;
    let mut location_of_extra_content = tokens[*position].location;
    location_of_extra_content.expand_until(last_token_location);

    Diagnostic::error(error_message)
        .add_help("Try to check if query keyword is missing")
        .add_help("Try remove un expected extra content")
        .with_location(location_of_extra_content)
        .as_boxed()
}

#[inline(always)]
#[allow(clippy::borrowed_box)]
fn expression_literal(expression: &Box<dyn Expr>) -> Option<String> {
    if let Some(symbol) = expression.as_any().downcast_ref::<SymbolExpr>() {
        return Some(symbol.value.to_string());
    }
    None
}

#[inline(always)]
fn resolve_symbol_type_or_undefine(env: &mut Environment, name: &String) -> Box<dyn DataType> {
    if let Some(symbol_type) = env.scopes.get(name) {
        symbol_type.clone()
    } else if let Some(symbol_type) = env.globals_types.get(name) {
        symbol_type.clone()
    } else if let Some(symbol_type) = env.schema.tables_fields_types.get(name.as_str()) {
        symbol_type.clone()
    } else {
        Box::new(UndefType)
    }
}

#[inline(always)]
fn register_current_table_fields_types(
    env: &mut Environment,
    table_name: &str,
) -> Result<(), Box<Diagnostic>> {
    let table_fields_names = &env.schema.tables_fields_names[table_name].clone();
    for field_name in table_fields_names {
        if env.schema.tables_fields_types.contains_key(field_name) {
            let field_type = env.schema.tables_fields_types[field_name].clone();
            env.define(field_name.to_string(), field_type);
            continue;
        }

        return Err(Diagnostic::error(
            &format!(
                "Column name {field_name} in table {table_name} has no type registered in the schema",
            )
            .to_string(),
        )
        .as_boxed());
    }

    Ok(())
}

#[inline(always)]
fn select_all_table_fields(
    env: &mut Environment,
    table_name: &[String],
    selected_fields: &mut Vec<String>,
    fields_names: &mut Vec<String>,
) {
    let mut tables_columns: Vec<&str> = vec![];
    for table in table_name {
        let columns = env.schema.tables_fields_names.get(table.as_str()).unwrap();
        for column in columns {
            tables_columns.push(column);
        }
    }

    for field in tables_columns {
        if !fields_names.contains(&field.to_string()) {
            fields_names.push(field.to_string());
            selected_fields.push(field.to_string());
        }
    }
}

#[inline(always)]
fn apply_not_keyword_if_exists(expr: Box<dyn Expr>, is_not_exists: bool) -> Box<dyn Expr> {
    if is_not_exists {
        Box::new(UnaryExpr {
            right: expr,
            operator: PrefixUnaryOperator::Bang,
            result_type: Box::new(BoolType),
        })
    } else {
        expr
    }
}

#[inline(always)]
pub(crate) fn is_current_token(
    tokens: &[Token],
    position: &usize,
    expected_kind: TokenKind,
) -> bool {
    *position < tokens.len() && tokens[*position].kind == expected_kind
}

#[inline(always)]
pub(crate) fn is_next_token(tokens: &[Token], position: &usize, expected_kind: TokenKind) -> bool {
    *position + 1 < tokens.len() && tokens[*position + 1].kind == expected_kind
}

#[inline(always)]
pub(crate) fn is_current_token_with_condition(
    tokens: &[Token],
    position: &usize,
    condition: fn(&Token) -> bool,
) -> bool {
    *position < tokens.len() && condition(&tokens[*position])
}

#[inline(always)]
pub(crate) fn consume_token_or_error<'a>(
    tokens: &'a [Token],
    position: &'a mut usize,
    expected_kind: TokenKind,
    message: &'a str,
) -> Result<&'a Token, Box<Diagnostic>> {
    if *position < tokens.len() && tokens[*position].kind == expected_kind {
        *position += 1;
        let index = *position - 1;
        return Ok(&tokens[index]);
    }

    Err(Diagnostic::error(message)
        .with_location(calculate_safe_location(tokens, *position))
        .as_boxed())
}

#[inline(always)]
pub(crate) fn consume_conditional_token_or_errors<'a>(
    tokens: &'a [Token],
    position: &'a mut usize,
    condition: fn(&Token) -> bool,
    message: &'a str,
) -> Result<&'a Token, Box<Diagnostic>> {
    if *position < tokens.len() && condition(&tokens[*position]) {
        *position += 1;
        let index = *position - 1;
        return Ok(&tokens[index]);
    }

    Err(Diagnostic::error(message)
        .with_location(calculate_safe_location(tokens, *position))
        .as_boxed())
}

#[inline(always)]
pub(crate) fn calculate_safe_location(tokens: &[Token], position: usize) -> SourceLocation {
    if position < tokens.len() {
        return tokens[position].location;
    }
    tokens[tokens.len() - 1].location
}

#[inline(always)]
pub(crate) fn is_logical_or_operator(tokens: &[Token], position: &usize) -> bool {
    *position < tokens.len()
        && matches!(
            tokens[*position].kind,
            TokenKind::OrOr | TokenKind::OrKeyword
        )
}

#[inline(always)]
pub(crate) fn is_logical_and_operator(tokens: &[Token], position: &usize) -> bool {
    *position < tokens.len()
        && matches!(
            tokens[*position].kind,
            TokenKind::AndAnd | TokenKind::AndKeyword
        )
}

#[inline(always)]
fn is_assignment_operator(token: &Token) -> bool {
    matches!(token.kind, TokenKind::Equal | TokenKind::ColonEqual)
}

#[inline(always)]
fn is_term_operator(token: &Token) -> bool {
    matches!(token.kind, TokenKind::Plus | TokenKind::Minus)
}

#[inline(always)]
fn is_bitwise_shift_operator(tokens: &[Token], position: &usize) -> bool {
    *position < tokens.len()
        && matches!(
            tokens[*position].kind,
            TokenKind::BitwiseLeftShift | TokenKind::BitwiseRightShift
        )
}

#[inline(always)]
fn is_prefix_unary_operator(token: &Token) -> bool {
    matches!(
        token.kind,
        TokenKind::Bang | TokenKind::Not | TokenKind::Minus | TokenKind::BitwiseNot
    )
}

#[inline(always)]
fn is_factor_operator(tokens: &[Token], position: &usize) -> bool {
    *position < tokens.len()
        && matches!(
            tokens[*position].kind,
            TokenKind::Star | TokenKind::Slash | TokenKind::Percentage | TokenKind::Caret
        )
}

#[inline(always)]
fn is_join_or_join_type_token(tokens: &[Token], position: &usize) -> bool {
    *position < tokens.len()
        && matches!(
            tokens[*position].kind,
            TokenKind::Join
                | TokenKind::Left
                | TokenKind::Right
                | TokenKind::Cross
                | TokenKind::Inner
        )
}
