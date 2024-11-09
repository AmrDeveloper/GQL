use gitql_ast::expression::CastExpr;
use gitql_ast::expression::Expr;
use gitql_ast::types::base::DataType;
use gitql_core::environment::Environment;

use crate::context::ParserContext;
use crate::diagnostic::Diagnostic;
use crate::parse_type::parse_type;
use crate::parser::consume_token_or_error;
use crate::parser::parse_expression;
use crate::tokenizer::Location;
use crate::tokenizer::Token;
use crate::tokenizer::TokenKind;

pub(crate) fn parse_cast_call_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Expr>, Box<Diagnostic>> {
    let cast_token_location =
        consume_token_or_error(tokens, position, TokenKind::Cast, "Expect 'CAST' Keyword")?
            .location;

    consume_token_or_error(
        tokens,
        position,
        TokenKind::LeftParen,
        "Expect '(' after 'CAST' Keyword",
    )?;

    let value = parse_expression(context, env, tokens, position)?;

    consume_token_or_error(
        tokens,
        position,
        TokenKind::As,
        "Expect 'AS' keyword after 'CAST' expression value",
    )?;

    let result_type = parse_type(env, tokens, position)?;

    consume_token_or_error(
        tokens,
        position,
        TokenKind::RightParen,
        "Expect ')' at the end of 'CAST' expression",
    )?;

    check_cast_expression(value, result_type, cast_token_location)
}

fn check_cast_expression(
    value: Box<dyn Expr>,
    result_type: Box<dyn DataType>,
    location: Location,
) -> Result<Box<dyn Expr>, Box<Diagnostic>> {
    let value_type = value.expr_type();
    let value_expected_types = value_type.can_perform_explicit_cast_op_to();

    // If it's supported to cast this value to result type, just return CastExpr
    if value_expected_types.contains(&result_type) {
        return Ok(Box::new(CastExpr { value, result_type }));
    }

    // Check if it possible to implicit cast the value to one of the expected type of result type
    // then Cast from expected type to the result type
    // Examples: Cast("true" as Int) can be casted as Text -> Bool -> Int
    let expected_types = result_type.can_perform_explicit_cast_op_to();
    for expected_type in expected_types {
        if expected_type.has_implicit_cast_from(&value) {
            let casting = Box::new(CastExpr {
                value,
                result_type: expected_type.clone(),
            });

            return Ok(Box::new(CastExpr {
                value: casting,
                result_type,
            }));
        }
    }

    Err(Diagnostic::error(&format!(
        "Unsupported `CAST` operator to type {}",
        result_type.literal(),
    ))
    .with_location(location)
    .as_boxed())
}
