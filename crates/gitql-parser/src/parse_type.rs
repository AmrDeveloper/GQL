use gitql_ast::types::array::ArrayType;
use gitql_ast::types::base::DataType;
use gitql_core::environment::Environment;

use crate::diagnostic::Diagnostic;
use crate::parser::calculate_safe_location;
use crate::parser::consume_token_or_error;
use crate::tokenizer::Token;
use crate::tokenizer::TokenKind;

pub(crate) fn parse_type(
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn DataType>, Box<Diagnostic>> {
    let mut data_type = parse_primitive_type(env, tokens, position)?;

    while *position < tokens.len() {
        match tokens[*position].kind {
            TokenKind::LeftBracket => data_type = parse_array_type(tokens, position, data_type)?,
            _ => break,
        }
    }

    Ok(data_type)
}

fn parse_array_type(
    tokens: &[Token],
    position: &mut usize,
    base_type: Box<dyn DataType>,
) -> Result<Box<dyn DataType>, Box<Diagnostic>> {
    // Make sure there is '[' After the base DataType
    if *position >= tokens.len() || tokens[*position].kind != TokenKind::LeftBracket {
        return Err(Diagnostic::error("Expect [ After Base DataType")
            .with_location(calculate_safe_location(tokens, *position - 1))
            .as_boxed());
    }

    // Consume '[' token
    *position += 1;

    // Make sure there is ']' After the base DataType
    if *position >= tokens.len() || tokens[*position].kind != TokenKind::RightBracket {
        return Err(Diagnostic::error("Expect ']' After '[' in Array DataType")
            .with_location(calculate_safe_location(tokens, *position - 1))
            .as_boxed());
    }

    // Consume ']' token
    *position += 1;

    Ok(Box::new(ArrayType { base: base_type }))
}

fn parse_primitive_type(
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn DataType>, Box<Diagnostic>> {
    // Parse `Symbol` token that represent DataType name
    let type_name_token = consume_token_or_error(
        tokens,
        position,
        TokenKind::Symbol,
        "Expect Symbol to represent Type name",
    )?;

    let type_literal = type_name_token.literal.to_string();
    if let Some(data_type) = env.types_table.lookup(type_literal.as_str()) {
        return Ok(data_type);
    }

    Err(Diagnostic::error(&format!(
        "No available type in TypeTable with name `{}`",
        type_literal
    ))
    .with_location(type_name_token.location)
    .as_boxed())
}
