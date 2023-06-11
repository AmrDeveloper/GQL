use lazy_static::lazy_static;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::diagnostic::GQLError;
use crate::expression::{CallExpression, CheckOperator, ComparisonOperator, LogicalOperator};
use crate::expression::{CheckExpression, ComparisonExpression, LogicalExpression, NotExpression};
use crate::expression::{Expression, StringExpression, SymbolExpression};
use crate::tokenizer::{Token, TokenKind};

use crate::statement::{
    LimitStatement, OffsetStatement, OrderByStatement, SelectStatement, Statement, WhereStatement,
};

use crate::transformation::TRANSFORMATIONS;

lazy_static! {
    static ref TABLES_FIELDS_NAMES: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("commits", vec!["title", "message", "name", "email", "time"]);
        map.insert("branches", vec!["name", "ishead", "isremote"]);
        map.insert("tags", vec!["name"]);
        map
    };
}

static mut current_table_fields: Vec<String> = Vec::new();

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

pub fn consume_kind(token: &Token, kind: TokenKind) -> Result<&Token, i32> {
    if token.kind == kind {
        return Ok(token);
    }
    return Err(0);
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
            let field_name_result = consume_kind(&tokens[*position], TokenKind::Symbol);
            if field_name_result.is_err() {
                return Err(GQLError {
                    message: "Expect `identifier` as a field name".to_owned(),
                    location: tokens[*position].location,
                });
            }

            fields.push(field_name_result.ok().unwrap().literal.to_string());
            *position += 1;
            if tokens[*position].kind == TokenKind::Comma {
                *position += 1;
            } else {
                break;
            }
        }
    } else {
        return Err(GQLError {
            message: "Expect `*` or `identifier` after `select` keyword".to_owned(),
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

    let table_name_result = consume_kind(&tokens[*position], TokenKind::Symbol);
    if table_name_result.is_err() {
        return Err(GQLError {
            message: "Expect `identifier` as a table name".to_owned(),
            location: tokens[*position].location,
        });
    }

    let table_name = &table_name_result.ok().unwrap().literal;
    if !TABLES_FIELDS_NAMES.contains_key(table_name.as_str()) {
        return Err(GQLError {
            message: "Invalid table name".to_owned(),
            location: tokens[*position].location,
        });
    }

    unsafe { current_table_fields.clear() };

    let valid_fields = TABLES_FIELDS_NAMES.get(table_name.as_str()).unwrap();
    for field in &fields {
        if !valid_fields.contains(&field.as_str()) {
            return Err(GQLError {
                message: "Invalid Field name".to_owned(),
                location: tokens[*position].location,
            });
        }

        unsafe { current_table_fields.push(field.to_string()) };
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
    if *position >= tokens.len() {
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
    return parse_logical_expression(tokens, position);
}

fn parse_logical_expression(
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, GQLError> {
    let expression = parse_comparison_expression(tokens, position);
    if expression.is_err() || *position >= tokens.len() {
        return expression;
    }

    let lhs = expression.ok().unwrap();
    let operator = &tokens[*position];

    if operator.kind == TokenKind::Or
        || operator.kind == TokenKind::And
        || operator.kind == TokenKind::Xor
    {
        *position += 1;

        let logical_operator = match operator.kind {
            TokenKind::Or => LogicalOperator::Or,
            TokenKind::And => LogicalOperator::And,
            _ => LogicalOperator::Xor,
        };

        let right_expr = parse_comparison_expression(tokens, position);
        if right_expr.is_err() {
            return Err(GQLError {
                message: "Can't parser right side of logical expression".to_owned(),
                location: tokens[*position].location,
            });
        }

        let rhs = right_expr.ok().unwrap();
        return Ok(Box::new(LogicalExpression {
            left: lhs,
            operator: logical_operator,
            right: rhs,
        }));
    }

    return Ok(lhs);
}

fn parse_comparison_expression(
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, GQLError> {
    let expression = parse_check_expression(tokens, position);
    if expression.is_err() || *position >= tokens.len() {
        return expression;
    }

    let lhs = expression.ok().unwrap();

    let operator = &tokens[*position];
    if operator.kind == TokenKind::Greater
        || operator.kind == TokenKind::GreaterEqual
        || operator.kind == TokenKind::Less
        || operator.kind == TokenKind::LessEqual
        || operator.kind == TokenKind::Equal
        || operator.kind == TokenKind::Bang
    {
        *position += 1;
        let comparison_operator = match operator.kind {
            TokenKind::Greater => ComparisonOperator::Greater,
            TokenKind::GreaterEqual => ComparisonOperator::GreaterEqual,
            TokenKind::Less => ComparisonOperator::Less,
            TokenKind::LessEqual => ComparisonOperator::LessEqual,
            TokenKind::Equal => ComparisonOperator::Equal,
            _ => ComparisonOperator::NotEqual,
        };

        let right_expr = parse_check_expression(tokens, position);

        if right_expr.is_err() {
            return Err(GQLError {
                message: "Can't right side of comparison expression".to_owned(),
                location: tokens[*position].location,
            });
        }

        let rhs = right_expr.ok().unwrap();
        return Ok(Box::new(ComparisonExpression {
            left: lhs,
            operator: comparison_operator,
            right: rhs,
        }));
    }

    return Ok(lhs);
}

fn parse_check_expression(
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, GQLError> {
    let expression = parse_unary_expression(tokens, position);
    if expression.is_err() || *position >= tokens.len() {
        return expression;
    }

    let lhs = expression.ok().unwrap();

    let operator = &tokens[*position];

    if operator.kind == TokenKind::Contains
        || operator.kind == TokenKind::StartsWith
        || operator.kind == TokenKind::EndsWith
        || operator.kind == TokenKind::Matches
    {
        *position += 1;

        let check_operator = match operator.kind {
            TokenKind::Contains => CheckOperator::Contains,
            TokenKind::StartsWith => CheckOperator::StartsWith,
            TokenKind::EndsWith => CheckOperator::EndsWith,
            _ => CheckOperator::Matches,
        };

        let right_expr = parse_unary_expression(tokens, position);
        if right_expr.is_err() {
            return Err(GQLError {
                message: "Can't parser right side of check expression".to_owned(),
                location: tokens[*position].location,
            });
        }

        let rhs = right_expr.ok().unwrap();
        return Ok(Box::new(CheckExpression {
            left: lhs,
            operator: check_operator,
            right: rhs,
        }));
    }

    return Ok(lhs);
}

fn parse_unary_expression(
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, GQLError> {
    if (&tokens[*position]).kind == TokenKind::Bang {
        *position += 1;
        let right_expr = parse_expression(tokens, position);
        if right_expr.is_err() {
            return right_expr;
        }
        let rhs = right_expr.ok().unwrap();
        return Ok(Box::new(NotExpression { right: rhs }));
    }

    return parse_call_expression(tokens, position);
}

fn parse_call_expression(
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, GQLError> {
    let mut expression = parse_primary_expression(tokens, position);
    if expression.is_err() || *position >= tokens.len() {
        return expression;
    }

    while (&tokens[*position]).kind == TokenKind::Dot {
        *position += 1;

        let function_name_result = consume_kind(&tokens[*position], TokenKind::Symbol);
        if function_name_result.is_err() {
            return Err(GQLError {
                message: "Expect `identifier` as a function name".to_owned(),
                location: tokens[*position].location,
            });
        }

        let function_name = function_name_result.ok().unwrap().literal.to_string();
        if !TRANSFORMATIONS.contains_key(function_name.as_str()) {
            return Err(GQLError {
                message: "Invalid GQL function name".to_owned(),
                location: tokens[*position].location,
            });
        }

        *position += 1;

        expression = Ok(Box::new(CallExpression {
            left: expression.ok().unwrap(),
            function_name: function_name,
        }));
    }

    return expression;
}

fn parse_primary_expression(
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, GQLError> {
    return match tokens[*position].kind {
        TokenKind::String => {
            *position += 1;
            Ok(Box::new(StringExpression {
                value: tokens[*position - 1].literal.to_string(),
            }))
        }
        TokenKind::Symbol => {
            *position += 1;

            let literal = &tokens[*position - 1].literal;
            if unsafe { !current_table_fields.contains(literal) } {
                return Err(GQLError {
                    message: "The current table contains no field with this name".to_owned(),
                    location: tokens[*position - 1].location,
                });
            }

            return Ok(Box::new(SymbolExpression {
                value: literal.to_string(),
            }));
        }
        TokenKind::LeftParen => {
            *position += 1;
            let expression = parse_expression(tokens, position);
            if tokens[*position].kind != TokenKind::RightParen {
                return Err(GQLError {
                    message: "Expect `)` to end group expression".to_owned(),
                    location: tokens[*position].location,
                });
            }
            *position += 1;
            expression
        }
        _ => Err(GQLError {
            message: "Can't parse primary expression".to_owned(),
            location: tokens[*position].location,
        }),
    };
}
