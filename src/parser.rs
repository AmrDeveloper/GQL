use crate::tokenizer::{Token, TokenKind};

use crate::statement::{LimitStatement, SelectStatement, Statement};

pub fn parse_gql(tokens: Vec<Token>) -> Result<Vec<Box<dyn Statement>>, String> {
    let mut statements: Vec<Box<dyn Statement>> = Vec::new();
    let len = tokens.len();
    let mut position = 0;

    while position < len {
        let token = &tokens[position];
        match &token.kind {
            TokenKind::Select => {
                position += 1;
                let mut fields: Vec<String> = Vec::new();
                if position >= len {
                    return Err("Expect * or fields names after select keyword".to_owned());
                }

                if tokens[position].kind != TokenKind::Star {
                    while position < len {
                        fields.push(tokens[position].literal.to_string());
                        position += 1;
                        if tokens[position].kind == TokenKind::Comma {
                            position += 1;
                        } else {
                            break;
                        }
                    }
                } else {
                    position += 1;
                }

                if tokens[position].kind != TokenKind::From {
                    return Err("Expect `from` keyword after attributes".to_owned());
                }

                position += 1;
                let table_name = &tokens[position].literal;
                position += 1;

                let statement = SelectStatement {
                    table_name: table_name.to_string(),
                    fields,
                };

                statements.push(Box::new(statement));
                continue;
            }
            TokenKind::Where => {}
            TokenKind::Limit => {
                position += 1;

                if position >= len || tokens[position].kind != TokenKind::Number {
                    return Err("Expect number after `limit` keyword".to_owned());
                }

                let count_str = tokens[position].literal.to_string();
                let count: usize = count_str.parse().unwrap();

                position += 1;

                let limit_statement = LimitStatement { count };
                statements.push(Box::new(limit_statement));
                continue;
            }
            TokenKind::Offset => {}
            _ => return Err("".to_owned()),
        }
    }

    return Ok(statements);
}
