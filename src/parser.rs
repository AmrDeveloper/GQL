use crate::tokenizer::{Token, TokenKind};

use crate::statement::{SelectStatement, Statement};

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
            TokenKind::Limit => {}
            TokenKind::Offset => {}
            _ => return Err("".to_owned()),
        }
    }

    return Ok(statements);
}
