use gitql_ast::expression::CastExpr;
use gitql_ast::expression::Expr;
use gitql_ast::expression::GlobExpr;
use gitql_ast::expression::LikeExpr;
use gitql_ast::expression::RegexExpr;
use gitql_core::environment::Environment;

use crate::context::ParserContext;
use crate::diagnostic::Diagnostic;
use crate::parse_cast::parse_cast_operator_expression;
use crate::parser::apply_not_keyword_if_exists;
use crate::parser::is_current_token;
use crate::parser::is_next_token;
use crate::parser::parse_is_null_expression;
use crate::token::Token;
use crate::token::TokenKind;

pub(crate) fn parse_like_expression(
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
        let operator_location = if has_not_keyword {
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

pub(crate) fn parse_glob_expression(
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

pub(crate) fn parse_regex_expression(
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
