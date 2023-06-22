use lazy_static::lazy_static;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::diagnostic::GQLError;
use crate::expression::{BooleanExpression, Expression, StringExpression, SymbolExpression};
use crate::expression::{CallExpression, CheckOperator, ComparisonOperator, LogicalOperator};
use crate::expression::{CheckExpression, ComparisonExpression, LogicalExpression, NotExpression};
use crate::statement::{LimitStatement, OffsetStatement, OrderByStatement};
use crate::statement::{SelectStatement, Statement, WhereStatement};
use crate::tokenizer::{Token, TokenKind};

use crate::tokenizer::Location;
use crate::transformation::TRANSFORMATIONS;
use crate::transformation::TRANSFORMATIONS_PROTOS;
use crate::types::DataType;

lazy_static! {
    static ref TABLES_FIELDS_NAMES: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("refs", vec!["name", "full_name", "type"]);
        map.insert("commits", vec!["title", "message", "name", "email", "time"]);
        map.insert("branches", vec!["name", "is_head", "is_remote"]);
        map.insert("tags", vec!["name"]);
        map
    };
}

static mut CURRENT_TABLE_FIELDS: Vec<String> = Vec::new();

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
    let mut fields_set: HashSet<String> = HashSet::new();
    let mut alias_table: HashMap<String, String> = HashMap::new();

    if *position >= tokens.len() {
        return Err(GQLError {
            message: "Expect * or fields names after select keyword".to_owned(),
            location: tokens[*position].location,
        });
    }

    if tokens[*position].kind == TokenKind::Star {
        *position += 1;
    } else if tokens[*position].kind == TokenKind::Symbol {
        let mut fields_names: HashSet<String> = HashSet::new();

        while *position < tokens.len() {
            let field_name_result = consume_kind(&tokens[*position], TokenKind::Symbol);
            if field_name_result.is_err() {
                return Err(GQLError {
                    message: "Expect `identifier` as a field name".to_owned(),
                    location: tokens[*position].location,
                });
            }

            let field_name = field_name_result.ok().unwrap().literal.to_string();
            if !fields_names.insert(field_name.to_string()) {
                return Err(GQLError {
                    message: "Can't select the same field twice".to_owned(),
                    location: tokens[*position].location,
                });
            }

            fields.push(field_name.to_string());

            *position += 1;

            if tokens[*position].kind == TokenKind::As {
                *position += 1;
                let alias_name_result = consume_kind(&tokens[*position], TokenKind::Symbol);
                if alias_name_result.is_err() {
                    return Err(GQLError {
                        message: "Expect `identifier` as a field alias name".to_owned(),
                        location: tokens[*position].location,
                    });
                }

                let alias_name = alias_name_result.ok().unwrap().literal.to_string();
                if fields_set.contains(&alias_name) {
                    return Err(GQLError {
                        message: "There is already field or alias with the same name".to_owned(),
                        location: tokens[*position].location,
                    });
                }

                *position += 1;

                // Insert the alias name to used later in conditions
                fields_set.insert(alias_name.to_string());

                alias_table.insert(field_name, alias_name);
            } else {
                // Insert the origin name
                fields_set.insert(field_name.to_string());
            }

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

    unsafe { CURRENT_TABLE_FIELDS.clear() };

    let valid_fields = TABLES_FIELDS_NAMES.get(table_name.as_str()).unwrap();
    for field in &fields {
        if !valid_fields.contains(&field.as_str()) {
            return Err(GQLError {
                message: "Invalid Field name".to_owned(),
                location: tokens[*position].location,
            });
        }
    }

    // If fields set is empty that mean it selecting all fields,
    // else it should add all fields set
    if fields_set.is_empty() {
        for valid_field in valid_fields {
            unsafe { CURRENT_TABLE_FIELDS.push(valid_field.to_string()) };
        }
    } else {
        for field in fields_set.iter() {
            unsafe { CURRENT_TABLE_FIELDS.push(field.to_string()) };
        }
    }

    *position += 1;

    let statement = SelectStatement {
        table_name: table_name.to_string(),
        fields,
        alias_table,
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

    // Consume optional ordering ASC or DES
    let mut is_ascending = true;
    if *position < tokens.len()
        && (tokens[*position].kind == TokenKind::Ascending
            || tokens[*position].kind == TokenKind::Descending)
    {
        is_ascending = tokens[*position].kind == TokenKind::Ascending;
        *position += 1;
    }

    return Ok(Box::new(OrderByStatement {
        field_name,
        is_ascending,
    }));
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

        if lhs.expr_type() != DataType::Boolean {
            return Err(type_missmatch_error(
                tokens[*position - 2].location,
                DataType::Boolean,
                lhs.expr_type(),
            ));
        }

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
        if rhs.expr_type() != DataType::Boolean {
            return Err(type_missmatch_error(
                tokens[*position].location,
                DataType::Boolean,
                lhs.expr_type(),
            ));
        }

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
        if rhs.expr_type() != DataType::Boolean {
            return Err(type_missmatch_error(
                tokens[*position - 1].location,
                DataType::Boolean,
                rhs.expr_type(),
            ));
        }

        return Ok(Box::new(NotExpression { right: rhs }));
    }

    return parse_dot_expression(tokens, position);
}

fn parse_dot_expression(
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
        let function_name_location = tokens[*position].location;
        if !TRANSFORMATIONS.contains_key(function_name.as_str()) {
            return Err(GQLError {
                message: "Invalid GQL function name".to_owned(),
                location: function_name_location,
            });
        }

        *position += 1;

        let callee = expression.ok().unwrap();

        let arguments_result = parse_call_arguments_expressions(tokens, position);
        if arguments_result.is_err() {
            return Err(arguments_result.err().unwrap());
        }

        let arguments = arguments_result.ok().unwrap();

        let prototype = TRANSFORMATIONS_PROTOS.get(function_name.as_str()).unwrap();
        let parameters = &prototype.parameters;
        let callee_expected = parameters.first().unwrap();

        // Check Callee type
        if &callee.expr_type() != callee_expected {
            let message = format!(
                "Function `{}` must be called from type `{}` not `{}`",
                function_name,
                callee_expected.literal(),
                callee.expr_type().literal()
            );

            return Err(GQLError {
                message: message,
                location: function_name_location,
            });
        }

        // Check number of parameters and arguments
        if arguments.len() != (parameters.len() - 1) {
            let message = format!(
                "Function `{}` expect `{}` arguments but got `{}`",
                function_name,
                parameters.len() - 1,
                arguments.len()
            );

            return Err(GQLError {
                message: message,
                location: function_name_location,
            });
        }

        // Check arguments vs parameters
        for index in 0..arguments.len() {
            let parameter_type = parameters.get(index + 1).unwrap();
            let argument_type = arguments.get(index).unwrap().expr_type();

            if parameter_type != &argument_type {
                let message = format!(
                    "Function `{}` argument number {} with type `{}` don't match expected type `{}`",
                    function_name,
                    index,
                    parameters.len() - 1,
                    arguments.len()
                );

                return Err(GQLError {
                    message: message,
                    location: function_name_location,
                });
            }
        }

        expression = Ok(Box::new(CallExpression {
            function_name,
            callee,
            arguments,
        }));
    }

    return expression;
}

fn parse_call_arguments_expressions(
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Vec<Box<dyn Expression>>, GQLError> {
    let mut arguments: Vec<Box<dyn Expression>> = vec![];
    if consume_kind(&tokens[*position], TokenKind::LeftParen).is_ok() {
        *position += 1;

        while tokens[*position].kind != TokenKind::RightParen {
            let argument_result = parse_expression(tokens, position);
            if argument_result.is_err() {
                return Err(argument_result.err().unwrap());
            }

            arguments.push(argument_result.ok().unwrap());

            if tokens[*position].kind == TokenKind::Comma {
                *position += 1;
            } else {
                break;
            }
        }

        if consume_kind(&tokens[*position], TokenKind::RightParen).is_ok() {
            *position += 1;
        } else {
            return Err(GQLError {
                message: "Expect `)` after function call arguments".to_owned(),
                location: tokens[*position].location,
            });
        }
    }
    return Ok(arguments);
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
            if unsafe { !CURRENT_TABLE_FIELDS.contains(literal) } {
                return Err(GQLError {
                    message: "The current table contains no selected field with this name"
                        .to_owned(),
                    location: tokens[*position - 1].location,
                });
            }

            return Ok(Box::new(SymbolExpression {
                value: literal.to_string(),
            }));
        }
        TokenKind::True => {
            *position += 1;
            return Ok(Box::new(BooleanExpression { is_true: true }));
        }
        TokenKind::False => {
            *position += 1;
            return Ok(Box::new(BooleanExpression { is_true: false }));
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

fn consume_kind(token: &Token, kind: TokenKind) -> Result<&Token, i32> {
    if token.kind == kind {
        return Ok(token);
    }
    return Err(0);
}

fn type_missmatch_error(location: Location, expected: DataType, actual: DataType) -> GQLError {
    let message = format!(
        "Type mismatch expected `{}`, got `{}`",
        expected.literal(),
        actual.literal()
    );
    return GQLError { message, location };
}
