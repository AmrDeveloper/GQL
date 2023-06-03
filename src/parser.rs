use lazy_static::lazy_static;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::diagnostic::GQLError;
use crate::expression::{BinaryExpression, EqualExpression, Expression, LogicalOperator};
use crate::tokenizer::{Token, TokenKind};

use crate::statement::{
    LimitStatement, OffsetStatement, OrderByStatement, SelectStatement, Statement, WhereStatement,
};

lazy_static! {
    static ref TABLES_FIELDS_NAMES: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("commits", vec!["title", "message", "name", "email"]);
        map.insert("branches", vec!["name", "ishead", "isremote"]);
        map.insert("tags", vec!["name"]);
        map
    };
}

pub fn parse_gql(tokens: Vec<Token>) -> Result<Vec<Box<dyn Statement>>, GQLError> {
    let mut statements: Vec<Box<dyn Statement>> = Vec::new();
    let len = tokens.len();
    let mut position = 0;

    let mut visisted_statements: HashSet<String> = HashSet::new();

    while position < len {
        let token = &tokens[position];

        match &token.kind {
            TokenKind::Select => {
                let is_unique = visisted_statements.insert(tokens[position].literal.to_string());
                if !is_unique {
                    return Err(GQLError {
                        message: "you already used `select` statement ".to_owned(),
                        location: token.location,
                    });
                }

                let parse_result = parse_select_statement(&tokens, &mut position);
                if parse_result.is_err() {
                    return Err(parse_result.err().unwrap());
                }
                statements.push(parse_result.ok().unwrap());
            }
            TokenKind::Where => {
                let is_unique = visisted_statements.insert(tokens[position].literal.to_string());
                if !is_unique {
                    return Err(GQLError {
                        message: "you already used `where` statement".to_owned(),
                        location: token.location,
                    });
                }
                let parse_result = parse_where_statement(&tokens, &mut position);
                if parse_result.is_err() {
                    return Err(parse_result.err().unwrap());
                }
                statements.push(parse_result.ok().unwrap());
            }
            TokenKind::Limit => {
                let is_unique = visisted_statements.insert(tokens[position].literal.to_string());
                if !is_unique {
                    return Err(GQLError {
                        message: "you already used `limit` statement".to_owned(),
                        location: token.location,
                    });
                }

                let parse_result = parse_limit_statement(&tokens, &mut position);
                if parse_result.is_err() {
                    return Err(parse_result.err().unwrap());
                }
                statements.push(parse_result.ok().unwrap());
            }
            TokenKind::Offset => {
                let is_unique = visisted_statements.insert(tokens[position].literal.to_string());
                if !is_unique {
                    return Err(GQLError {
                        message: "you already used `offset` statement".to_owned(),
                        location: token.location,
                    });
                }

                let parse_result = parse_offset_statement(&tokens, &mut position);
                if parse_result.is_err() {
                    return Err(parse_result.err().unwrap());
                }
                statements.push(parse_result.ok().unwrap());
            }
            TokenKind::Order => {
                let is_unique = visisted_statements.insert(tokens[position].literal.to_string());
                if !is_unique {
                    return Err(GQLError {
                        message: "you already used `order by` statement".to_owned(),
                        location: token.location,
                    });
                }

                let parse_result = parse_order_by_statement(&tokens, &mut position);
                if parse_result.is_err() {
                    return Err(parse_result.err().unwrap());
                }
                statements.push(parse_result.ok().unwrap());
            }
            _ => {
                return Err(GQLError {
                    message: "Unexpected statement".to_owned(),
                    location: token.location,
                })
            }
        }
    }

    return Ok(statements);
}

fn parse_select_statement(
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Statement>, GQLError> {
    *position += 1;
    let mut fields: Vec<String> = Vec::new();
    if *position >= tokens.len() {
        return Err(GQLError {
            message: "Expect * or fields names after select keyword".to_owned(),
            location: tokens[*position].location,
        });
    }

    if tokens[*position].kind == TokenKind::Star {
        *position += 1;
    } else if tokens[*position].kind == TokenKind::Symbol {
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
        return Err(GQLError {
            message: "Expect `*` or `symbols` after `select` keyword".to_owned(),
            location: tokens[*position].location,
        });
    }

    if tokens[*position].kind != TokenKind::From {
        return Err(GQLError {
            message: "Expect `from` keyword after attributes".to_owned(),
            location: tokens[*position].location,
        });
    }

    *position += 1;

    let table_name = &tokens[*position].literal;
    if !TABLES_FIELDS_NAMES.contains_key(table_name.as_str()) {
        return Err(GQLError {
            message: "Invalid table name".to_owned(),
            location: tokens[*position].location,
        });
    }

    let valid_fields = TABLES_FIELDS_NAMES.get(table_name.as_str()).unwrap();
    for field in &fields {
        if !valid_fields.contains(&field.as_str()) {
            return Err(GQLError {
                message: "Invalid Field name".to_owned(),
                location: tokens[*position].location,
            });
        }
    }

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
) -> Result<Box<dyn Statement>, GQLError> {
    *position += 1;
    if *position >= tokens.len() || tokens[*position].kind != TokenKind::Symbol {
        return Err(GQLError {
            message: "Expect expression after `where` keyword".to_owned(),
            location: tokens[*position - 1].location,
        });
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
) -> Result<Box<dyn Statement>, GQLError> {
    *position += 1;
    if *position >= tokens.len() || tokens[*position].kind != TokenKind::Number {
        return Err(GQLError {
            message: "Expect number after `limit` keyword".to_owned(),
            location: tokens[*position - 1].location,
        });
    }

    let count_str = tokens[*position].literal.to_string();
    let count: usize = count_str.parse().unwrap();
    *position += 1;
    return Ok(Box::new(LimitStatement { count }));
}

fn parse_offset_statement(
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Statement>, GQLError> {
    *position += 1;
    if *position >= tokens.len() || tokens[*position].kind != TokenKind::Number {
        return Err(GQLError {
            message: "Expect number after `offset` keyword".to_owned(),
            location: tokens[*position - 1].location,
        });
    }

    let count_str = tokens[*position].literal.to_string();
    let count: usize = count_str.parse().unwrap();
    *position += 1;
    return Ok(Box::new(OffsetStatement { count }));
}

fn parse_order_by_statement(
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Statement>, GQLError> {
    *position += 1;
    if *position >= tokens.len() || tokens[*position].kind != TokenKind::By {
        return Err(GQLError {
            message: "Expect keyword `by` after keyword `order`".to_owned(),
            location: tokens[*position - 1].location,
        });
    }
    *position += 1;
    if *position >= tokens.len() || tokens[*position].kind != TokenKind::Symbol {
        return Err(GQLError {
            message: "Expect field name after `order by`".to_owned(),
            location: tokens[*position - 1].location,
        });
    }

    let field_name = tokens[*position].literal.to_string();
    *position += 1;
    return Ok(Box::new(OrderByStatement { field_name }));
}

fn parse_expression(
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, GQLError> {
    if tokens[*position].kind != TokenKind::Symbol {
        return Err(GQLError {
            message: "Expect `symbol` as field name".to_owned(),
            location: tokens[*position].location,
        });
    }

    let field_name = &tokens[*position].literal;
    *position += 1;
    let function = &tokens[*position].literal;
    *position += 1;
    let expected_value = &tokens[*position].literal;
    *position += 1;

    let expression = match function.as_str() {
        "=" => EqualExpression {
            field_name: field_name.to_string(),
            expected_value: expected_value.to_string(),
        },
        _ => {
            return Err(GQLError {
                message: "Expect `symbol` as field name".to_owned(),
                location: tokens[*position - 1].location,
            })
        }
    };

    if *position < tokens.len()
        && (tokens[*position].kind == TokenKind::And || tokens[*position].kind == TokenKind::Or)
    {
        let operator = if tokens[*position].kind == TokenKind::And {
            LogicalOperator::And
        } else {
            LogicalOperator::Or
        };

        *position += 1;
        let other_expr = parse_expression(tokens, position);

        let mut binary_expression = BinaryExpression {
            right: Box::new(expression),
            operator: operator,
            left: other_expr.ok().unwrap(),
        };

        while *position < tokens.len()
            && (tokens[*position].kind == TokenKind::And || tokens[*position].kind == TokenKind::Or)
        {
            let operator = if tokens[*position].kind == TokenKind::And {
                LogicalOperator::And
            } else {
                LogicalOperator::Or
            };

            *position += 1;
            let other_expr = parse_expression(tokens, position);
            binary_expression = BinaryExpression {
                right: Box::new(binary_expression),
                operator: operator,
                left: other_expr.ok().unwrap(),
            }
        }

        return Ok(Box::new(binary_expression));
    }

    return Ok(Box::new(expression));
}
