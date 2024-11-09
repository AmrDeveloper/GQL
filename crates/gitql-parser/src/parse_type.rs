use std::collections::HashMap;

use gitql_ast::types::array::ArrayType;
use gitql_ast::types::base::DataType;
use gitql_ast::types::boolean::BoolType;
use gitql_ast::types::date::DateType;
use gitql_ast::types::datetime::DateTimeType;
use gitql_ast::types::float::FloatType;
use gitql_ast::types::integer::IntType;
use gitql_ast::types::text::TextType;
use gitql_ast::types::time::TimeType;
use gitql_core::environment::Environment;

use crate::diagnostic::Diagnostic;
use crate::parser::{consume_token_or_error, get_safe_location};
use crate::tokenizer::{Token, TokenKind};

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
            .with_location(get_safe_location(tokens, *position - 1))
            .as_boxed());
    }

    // Consume '[' token
    *position += 1;

    // Make sure there is ']' After the base DataType
    if *position >= tokens.len() || tokens[*position].kind != TokenKind::RightBracket {
        return Err(Diagnostic::error("Expect ']' After '[' in Array DataType")
            .with_location(get_safe_location(tokens, *position - 1))
            .as_boxed());
    }

    // Consume ']' token
    *position += 1;

    Ok(Box::new(ArrayType { base: base_type }))
}

fn parse_primitive_type(
    _env: &mut Environment, // Access type table from env :D
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

    let mut type_table: HashMap<String, Box<dyn DataType>> = HashMap::new();
    type_table.insert("integer".to_string(), Box::new(IntType));
    type_table.insert("int".to_string(), Box::new(IntType));
    type_table.insert("real".to_string(), Box::new(FloatType));
    type_table.insert("float".to_string(), Box::new(FloatType));
    type_table.insert("boolean".to_string(), Box::new(BoolType));
    type_table.insert("bool".to_string(), Box::new(BoolType));
    type_table.insert("text".to_string(), Box::new(TextType));
    type_table.insert("date".to_string(), Box::new(DateType));
    type_table.insert("time".to_string(), Box::new(TimeType));
    type_table.insert("datetime".to_string(), Box::new(DateTimeType));

    let type_literal = &type_name_token.literal;
    if type_table.contains_key(type_literal) {
        return Ok(type_table.get(type_literal).unwrap().clone());
    }

    Err(Diagnostic::error(&format!(
        "No available type in TypeTable with name `{}`",
        type_literal
    ))
    .with_location(type_name_token.location)
    .as_boxed())
}
