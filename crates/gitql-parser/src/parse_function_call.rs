use gitql_ast::expression::CallExpr;
use gitql_ast::expression::Expr;
use gitql_ast::expression::SymbolExpr;
use gitql_ast::statement::AggregateValue;
use gitql_ast::statement::OverClause;
use gitql_ast::statement::Statement;
use gitql_ast::statement::WindowFunction;
use gitql_ast::statement::WindowFunctionKind;
use gitql_core::environment::Environment;

use crate::context::ParserContext;
use crate::diagnostic::Diagnostic;
use crate::parser::consume_token_or_error;
use crate::parser::is_current_token;
use crate::parser::parse_member_access_expression;
use crate::parser::parse_order_by_statement;
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
    if *position < tokens.len() && matches!(tokens[*position].kind, TokenKind::Symbol(_)) {
        let symbol_token = &tokens[*position];
        if *position + 1 < tokens.len() && tokens[*position + 1].kind == TokenKind::LeftParen {
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
                let aggregations_count_before = context.aggregations.len();
                let mut arguments = parse_zero_or_more_values_with_comma_between(
                    context,
                    env,
                    tokens,
                    position,
                    "Aggregation function",
                )?;

                // Prevent calling aggregation function with aggregation values as argument
                let has_aggregations = context.aggregations.len() != aggregations_count_before;
                if has_aggregations {
                    return Err(Diagnostic::error(
                        "Aggregated values can't as used for aggregation function argument",
                    )
                    .with_location(function_name_location)
                    .as_boxed());
                }

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

                    let is_used_as_window_function = *position < tokens.len()
                        && matches!(tokens[*position].kind, TokenKind::Over);

                    if is_used_as_window_function && context.has_select_statement {
                        return Err(Diagnostic::error(
                            "Window function can't called after `SELECT` statement",
                        )
                        .with_location(function_name_location)
                        .as_boxed());
                    }

                    if is_used_as_window_function {
                        let order_clauses =
                            parse_window_function_over_clause(context, env, tokens, position)?;

                        let function = WindowFunction {
                            function_name: function_name.to_string(),
                            arguments,
                            order_clauses,
                            kind: WindowFunctionKind::AggregatedWindowFunction,
                        };

                        context
                            .window_functions
                            .insert(column_name.clone(), function);
                    } else {
                        let function =
                            AggregateValue::Function(function_name.to_string(), arguments);
                        context.aggregations.insert(column_name.clone(), function);
                    }

                    // Return a Symbol that reference to the aggregation function generated name
                    return Ok(Box::new(SymbolExpr {
                        value: column_name,
                        result_type: return_type,
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

                if let Some(signature) = env.window_function_signature(function_name) {
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

                    // Consume `OVER` keyword
                    consume_token_or_error(
                        tokens,
                        position,
                        TokenKind::Over,
                        "Window function must have `OVER(...)` even if it empty",
                    )?;

                    if context.has_select_statement {
                        return Err(Diagnostic::error(
                            "Window function can't called after `SELECT` statement",
                        )
                        .with_location(function_name_location)
                        .as_boxed());
                    }

                    let order_clauses =
                        parse_window_function_over_clause(context, env, tokens, position)?;

                    let function = WindowFunction {
                        function_name: function_name.to_string(),
                        arguments,
                        order_clauses,
                        kind: WindowFunctionKind::PureWindowFunction,
                    };

                    context
                        .window_functions
                        .insert(column_name.clone(), function);

                    // Return a Symbol that reference to the aggregation function generated name
                    return Ok(Box::new(SymbolExpr {
                        value: column_name,
                        result_type: return_type,
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
    }

    parse_member_access_expression(context, env, tokens, position)
}

pub(crate) fn parse_window_function_over_clause(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<OverClause, Box<Diagnostic>> {
    let mut clauses: Vec<Box<dyn Statement>> = vec![];
    // Consume `OVER` token
    *position += 1;

    // Consume `(` token
    consume_token_or_error(
        tokens,
        position,
        TokenKind::LeftParen,
        "Expect `(` after Over keyword",
    )?;

    context.inside_over_clauses = true;
    let mut has_order_by_clause = false;
    while !is_current_token(tokens, position, TokenKind::RightParen) {
        match tokens[*position].kind {
            TokenKind::Order => {
                if has_order_by_clause {
                    return Err(Diagnostic::error(
                        "`OVER` clause can contains only one `ORDER BY`",
                    )
                    .with_location(tokens[*position].location)
                    .as_boxed());
                }
                clauses.push(parse_order_by_statement(context, env, tokens, position)?);
                has_order_by_clause = true;
            }
            _ => {
                context.inside_over_clauses = false;
                // TODO: update message later to '"`OVER` clause can only support `ORDER BY` or `PARTITION BY` Statements"'
                return Err(Diagnostic::error(
                    "`OVER` clause can only support `ORDER BY` Statements",
                )
                .with_location(tokens[*position].location)
                .as_boxed());
            }
        }
    }

    context.inside_over_clauses = false;

    // Consume `)` token
    consume_token_or_error(
        tokens,
        position,
        TokenKind::RightParen,
        "Expect `)` after Over clauses",
    )?;

    Ok(OverClause { clauses })
}
