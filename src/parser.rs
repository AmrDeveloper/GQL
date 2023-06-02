use crate::expression::{EqualExpression, Expression};
use crate::tokenizer::{Token, TokenKind};

use crate::statement::{
    LimitStatement, OffsetStatement, SelectStatement, Statement, WhereStatement,
};

pub fn parse_gql(tokens: Vec<Token>) -> Result<Vec<Box<dyn Statement>>, String> {
    let mut statements: Vec<Box<dyn Statement>> = Vec::new();
    let len = tokens.len();
    let mut position = 0;

    while position < len {
        let token = &tokens[position];
        match &token.kind {
            TokenKind::Select => {
                let parse_result = parse_select_statement(&tokens, &mut position);
                if parse_result.is_err() {
                    return Err(parse_result.err().unwrap());
                }
                statements.push(parse_result.ok().unwrap());
            }
            TokenKind::Where => {
                let parse_result = parse_where_statement(&tokens, &mut position);
                if parse_result.is_err() {
                    return Err(parse_result.err().unwrap());
                }
                statements.push(parse_result.ok().unwrap());
            }
            TokenKind::Limit => {
                let parse_result = parse_limit_statement(&tokens, &mut position);
                if parse_result.is_err() {
                    return Err(parse_result.err().unwrap());
                }
                statements.push(parse_result.ok().unwrap());
            }
            TokenKind::Offset => {
                let parse_result = parse_offset_statement(&tokens, &mut position);
                if parse_result.is_err() {
                    return Err(parse_result.err().unwrap());
                }
                statements.push(parse_result.ok().unwrap());
            }
            _ => return Err("Invalid statement".to_owned()),
        }
    }

    return Ok(statements);
}

fn parse_select_statement(
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Statement>, String> {
    *position += 1;
    let mut fields: Vec<String> = Vec::new();
    if *position >= tokens.len() {
        return Err("Expect * or fields names after select keyword".to_owned());
    }

    if tokens[*position].kind != TokenKind::Star {
        while *position < tokens.len() {
            fields.push(tokens[*position].literal.to_string());
            *position += 1;
            if tokens[*position].kind == TokenKind::Comma {
                *position += 1;
            } else {
                break;
            }
        }
    } else {
        *position += 1;
    }

    if tokens[*position].kind != TokenKind::From {
        return Err("Expect `from` keyword after attributes".to_owned());
    }

    *position += 1;
    let table_name = &tokens[*position].literal;
    *position += 1;

    let statement = SelectStatement {
        table_name: table_name.to_string(),
        fields,
    };

    return Ok(Box::new(statement));
}

fn parse_where_statement(
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Statement>, String> {
    *position += 1;
    if *position >= tokens.len() || tokens[*position].kind != TokenKind::Symbol {
        return Err("Expect expression after `where` keyword".to_owned());
    }

    let expression_result = parse_expression(&tokens, position);
    if expression_result.is_err() {
        return Err(expression_result.err().unwrap());
    }

    return Ok(Box::new(WhereStatement {
        condition: expression_result.ok().unwrap(),
    }));
}

fn parse_limit_statement(
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Statement>, String> {
    *position += 1;
    if *position >= tokens.len() || tokens[*position].kind != TokenKind::Number {
        return Err("Expect number after `limit` keyword".to_owned());
    }

    let count_str = tokens[*position].literal.to_string();
    let count: usize = count_str.parse().unwrap();
    *position += 1;
    return Ok(Box::new(LimitStatement { count }));
}

fn parse_offset_statement(
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Statement>, String> {
    *position += 1;
    if *position >= tokens.len() || tokens[*position].kind != TokenKind::Number {
        return Err("Expect number after `offset` keyword".to_owned());
    }

    let count_str = tokens[*position].literal.to_string();
    let count: usize = count_str.parse().unwrap();
    *position += 1;
    return Ok(Box::new(OffsetStatement { count }));
}

fn parse_expression(
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, String> {
    if tokens[*position].kind != TokenKind::Symbol {
        return Err("Expect `symbol` as field name".to_owned());
    }

    let field_name = &tokens[*position].literal;
    *position += 1;
    let function = &tokens[*position].literal;
    *position += 1;
    let expected_value = &tokens[*position].literal;
    *position += 1;

    if function != "equals" {
        return Err("Invalid function name".to_owned());
    }

    let equals_expression = EqualExpression {
        field_name: field_name.to_string(),
        expected_value: expected_value.to_string(),
    };

    return Ok(Box::new(equals_expression));
}
