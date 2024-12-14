use gitql_ast::expression::CallExpr;
use gitql_ast::expression::Expr;
use gitql_ast::expression::SymbolExpr;
use gitql_ast::expression::SymbolFlag;
use gitql_ast::statement::AggregateValue;
use gitql_ast::statement::WindowDefinition;
use gitql_ast::statement::WindowFunction;
use gitql_ast::statement::WindowFunctionKind;
use gitql_ast::statement::WindowOrderingClause;
use gitql_ast::statement::WindowPartitioningClause;
use gitql_core::environment::Environment;

use crate::context::ParserContext;
use crate::diagnostic::Diagnostic;
use crate::parser::consume_token_or_error;
use crate::parser::is_current_token;
use crate::parser::is_current_token_with_condition;
use crate::parser::parse_expression;
use crate::parser::parse_member_access_expression;
use crate::parser::parse_sorting_order;
use crate::parser::parse_zero_or_more_values_with_comma_between;
use crate::token::Token;
use crate::token::TokenKind;
use crate::type_checker::check_function_call_arguments;
use crate::type_checker::resolve_dynamic_data_type;

pub(crate) fn parse_function_call_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Expr>, Box<Diagnostic>> {
    // Check for <Symbol> `(` to parse function call
    if *position + 1 < tokens.len()
        && matches!(tokens[*position].kind, TokenKind::Symbol(_))
        && tokens[*position + 1].kind == TokenKind::LeftParen
    {
        let symbol_token = &tokens[*position];
        let function_name = &symbol_token.to_string();
        let function_name_location = symbol_token.location;

        // Consume function name
        *position += 1;

        // Check if this function is a Standard library functions
        if env.is_std_function(function_name) {
            let mut arguments = parse_zero_or_more_values_with_comma_between(
                context,
                env,
                tokens,
                position,
                "Std function",
            )?;

            if let Some(signature) = env.std_signature(function_name.as_str()) {
                check_function_call_arguments(
                    &mut arguments,
                    &signature.parameters,
                    function_name.to_string(),
                    function_name_location,
                )?;

                let return_type = resolve_dynamic_data_type(
                    &signature.parameters,
                    &arguments,
                    &signature.return_type,
                );

                // Register function name with return type after resolving it
                env.define(function_name.to_string(), return_type.clone());

                return Ok(Box::new(CallExpr {
                    function_name: function_name.to_string(),
                    arguments,
                    return_type,
                }));
            }

            // Function has no signature registered on the signature table
            return Err(Diagnostic::error(&format!(
                "Can't find signature for function with name {}",
                function_name
            ))
            .with_location(function_name_location)
            .as_boxed());
        }

        // Check if this function is an Aggregation functions
        if env.is_aggregation_function(function_name) {
            let mut arguments = parse_zero_or_more_values_with_comma_between(
                context,
                env,
                tokens,
                position,
                "Aggregation function",
            )?;

            if let Some(signature) = env.aggregation_signature(function_name.as_str()) {
                // Perform type checking and implicit casting if needed for function arguments
                check_function_call_arguments(
                    &mut arguments,
                    &signature.parameters,
                    function_name.to_string(),
                    function_name_location,
                )?;

                let column_name = context.name_generator.generate_temp_name();
                context.hidden_selections.push(column_name.to_string());

                let return_type = resolve_dynamic_data_type(
                    &signature.parameters,
                    &arguments,
                    &signature.return_type,
                );

                // Register aggregation generated name with return type after resolving it
                env.define(column_name.to_string(), return_type.clone());

                let is_used_as_window_function =
                    *position < tokens.len() && matches!(tokens[*position].kind, TokenKind::Over);

                if is_used_as_window_function && context.has_select_statement {
                    return Err(Diagnostic::error(
                        "Window function can't called after `SELECT` statement",
                    )
                    .with_location(function_name_location)
                    .as_boxed());
                }

                let mut flag = SymbolFlag::AggregationReference;
                if is_used_as_window_function {
                    // Consume `OVER` token
                    *position += 1;

                    let order_clauses =
                        parse_over_window_definition(context, env, tokens, position)?;

                    let function = WindowFunction {
                        function_name: function_name.to_string(),
                        arguments,
                        window_definition: order_clauses,
                        kind: WindowFunctionKind::AggregatedWindowFunction,
                    };

                    context
                        .window_functions
                        .insert(column_name.clone(), function);

                    flag = SymbolFlag::WindowReference;
                } else {
                    let function = AggregateValue::Function(function_name.to_string(), arguments);
                    context.aggregations.insert(column_name.clone(), function);
                }

                // Return a Symbol that reference to the aggregation function generated name
                return Ok(Box::new(SymbolExpr {
                    value: column_name,
                    result_type: return_type,
                    flag,
                }));
            }

            // Aggregation Function has no signature registered on the signature table
            return Err(Diagnostic::error(&format!(
                "Can't find signature for Aggregation function with name {}",
                function_name
            ))
            .with_location(function_name_location)
            .as_boxed());
        }

        // Check if this function is an Window function
        if env.is_window_function(function_name) {
            let aggregations_count_before = context.aggregations.len();
            let window_functions_count_before = context.window_functions.len();
            let mut arguments = parse_zero_or_more_values_with_comma_between(
                context,
                env,
                tokens,
                position,
                "Window function",
            )?;

            // Prevent calling window function with aggregation values as argument
            if context.aggregations.len() != aggregations_count_before {
                return Err(Diagnostic::error(
                    "Aggregated values can't as used for aggregation function argument",
                )
                .with_location(function_name_location)
                .as_boxed());
            }

            // Prevent calling window function with window function values as argument
            if context.window_functions.len() != window_functions_count_before {
                return Err(Diagnostic::error(
                    "Window functions values can't as used for Window function argument",
                )
                .with_location(function_name_location)
                .as_boxed());
            }

            if context.has_select_statement {
                return Err(Diagnostic::error(
                    "Window function can't called after `SELECT` statement",
                )
                .with_location(function_name_location)
                .as_boxed());
            }

            if let Some(signature) = env.window_function_signature(function_name) {
                // Perform type checking and implicit casting if needed for function arguments
                check_function_call_arguments(
                    &mut arguments,
                    &signature.parameters,
                    function_name.to_string(),
                    function_name_location,
                )?;

                // TODO: Make sure to be used in Order by
                let column_name = context.name_generator.generate_column_name();
                context.hidden_selections.push(column_name.to_string());

                let return_type = resolve_dynamic_data_type(
                    &signature.parameters,
                    &arguments,
                    &signature.return_type,
                );

                // Register aggregation generated name with return type after resolving it
                env.define(column_name.to_string(), return_type.clone());

                // Consume `OVER` keyword
                consume_token_or_error(
                    tokens,
                    position,
                    TokenKind::Over,
                    "Window function must have `OVER(...)` even if it empty",
                )?;

                let order_clauses = parse_over_window_definition(context, env, tokens, position)?;

                let function = WindowFunction {
                    function_name: function_name.to_string(),
                    arguments,
                    window_definition: order_clauses,
                    kind: WindowFunctionKind::PureWindowFunction,
                };

                context
                    .window_functions
                    .insert(column_name.clone(), function);

                // Return a Symbol that reference to the aggregation function generated name
                return Ok(Box::new(SymbolExpr {
                    value: column_name,
                    result_type: return_type,
                    flag: SymbolFlag::WindowReference,
                }));
            }

            // Aggregation Function has no signature registered on the signature table
            return Err(Diagnostic::error(&format!(
                "Can't find signature for Window function with name {}",
                function_name
            ))
            .with_location(function_name_location)
            .as_boxed());
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

    parse_member_access_expression(context, env, tokens, position)
}

pub(crate) fn parse_over_window_definition(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<WindowDefinition, Box<Diagnostic>> {
    // Consume one Symbol as Named Window Over, will be checked later before constructing the query object
    if is_current_token_with_condition(tokens, position, |t| matches!(t.kind, TokenKind::Symbol(_)))
    {
        let over_clause_name = tokens[*position].to_string();
        *position += 1;

        return Ok(WindowDefinition {
            name: Some(over_clause_name),
            partitioning_clause: None,
            ordering_clause: None,
        });
    }

    // Consume `(` token
    consume_token_or_error(
        tokens,
        position,
        TokenKind::LeftParen,
        "Expect `(` after Over keyword",
    )?;

    let mut window_definition = WindowDefinition {
        name: None,
        partitioning_clause: None,
        ordering_clause: None,
    };

    context.inside_over_clauses = true;
    while !is_current_token(tokens, position, TokenKind::RightParen) {
        if tokens[*position].kind == TokenKind::Partition {
            // Check if `PARTITION BY` is used more than one time
            if window_definition.partitioning_clause.is_some() {
                return Err(Diagnostic::error(
                    "This window definition already has `PARTITION BY` statement",
                )
                .with_location(tokens[*position].location)
                .as_boxed());
            }

            // Consume `PARTITION`
            *position += 1;

            // Consume `BY` or report error message
            consume_token_or_error(
                tokens,
                position,
                TokenKind::By,
                "Expect `BY` keyword after `PARTITION`",
            )?;

            let window_functions_count_before = context.window_functions.len();
            let expr = parse_expression(context, env, tokens, position)?;
            if window_functions_count_before != context.window_functions.len() {
                return Err(Diagnostic::error(
                    "Window functions are not allowed in window definitions",
                )
                .with_location(tokens[*position].location)
                .as_boxed());
            }

            let partition_by = WindowPartitioningClause { expr };
            window_definition.partitioning_clause = Some(partition_by);
            continue;
        }

        if tokens[*position].kind == TokenKind::Order {
            // Check if `ORDER BY` is used more than one time
            if window_definition.ordering_clause.is_some() {
                return Err(Diagnostic::error(
                    "This window definition already has `ORDER BY` statement",
                )
                .with_location(tokens[*position].location)
                .as_boxed());
            }

            // Consume `ORDER`
            *position += 1;

            // Consume `BY` or report error message
            consume_token_or_error(
                tokens,
                position,
                TokenKind::By,
                "Expect `BY` keyword after `ORDER`",
            )?;

            let window_functions_count_before = context.window_functions.len();
            let expr = parse_expression(context, env, tokens, position)?;
            if window_functions_count_before != context.window_functions.len() {
                return Err(Diagnostic::error(
                    "Window functions are not allowed in window definitions",
                )
                .with_location(tokens[*position].location)
                .as_boxed());
            }

            let ordering = parse_sorting_order(tokens, position)?;
            let order_by = WindowOrderingClause { expr, ordering };
            window_definition.ordering_clause = Some(order_by);
            continue;
        }

        context.inside_over_clauses = false;
        return Err(Diagnostic::error(
            "`OVER` clause can only support `ORDER BY` or `PARTITION BY`  clauses",
        )
        .with_location(tokens[*position].location)
        .as_boxed());
    }

    context.inside_over_clauses = false;

    // Consume `)` token for closing OVER clauses with empty or valid Window definition
    consume_token_or_error(
        tokens,
        position,
        TokenKind::RightParen,
        "Expect `)` after Over clauses",
    )?;

    Ok(window_definition)
}
