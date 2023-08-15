use lazy_static::lazy_static;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::aggregation::AGGREGATIONS;
use crate::aggregation::AGGREGATIONS_PROTOS;
use crate::diagnostic::GQLError;
use crate::expression::{ArithmeticExpression, BetweenExpression, CallExpression, Expression};
use crate::expression::{ArithmeticOperator, CheckOperator, ComparisonOperator, LogicalOperator};
use crate::expression::{BitwiseExpression, BitwiseOperator};
use crate::expression::{BooleanExpression, NumberExpression, StringExpression, SymbolExpression};
use crate::expression::{CheckExpression, ComparisonExpression, LogicalExpression, NotExpression};
use crate::statement::{AggregateFunction, AggregationFunctionsStatement};
use crate::statement::{GQLQuery, HavingStatement, WhereStatement};
use crate::statement::{GroupByStatement, SelectStatement, Statement};
use crate::statement::{LimitStatement, OffsetStatement, OrderByStatement};
use crate::tokenizer::Location;
use crate::tokenizer::{Token, TokenKind};
use crate::transformation::TRANSFORMATIONS;
use crate::transformation::TRANSFORMATIONS_PROTOS;
use crate::types::DataType;
use crate::types::TABLES_FIELDS_TYPES;

lazy_static! {
    static ref TABLES_FIELDS_NAMES: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("refs", vec!["name", "full_name", "type"]);
        map.insert(
            "commits",
            vec!["commit_id", "title", "message", "name", "email", "time"],
        );
        map.insert(
            "branches",
            vec!["name", "commit_count", "is_head", "is_remote"],
        );
        map.insert(
            "diffs",
            vec![
                "commit_id",
                "name",
                "email",
                "insertions",
                "deletions",
                "files_changed",
            ],
        );
        map.insert("tags", vec!["name"]);
        map
    };
}

static mut CURRENT_TABLE_FIELDS: Vec<String> = Vec::new();

pub fn parse_gql(tokens: Vec<Token>) -> Result<GQLQuery, GQLError> {
    let len = tokens.len();
    let mut position = 0;

    let mut statements: HashMap<String, Box<dyn Statement>> = HashMap::new();
    let mut aggregations: HashMap<String, AggregateFunction> = HashMap::new();
    let mut extra_type_table: HashMap<String, DataType> = HashMap::new();
    let mut hidden_selections: Vec<String> = Vec::new();

    let mut select_aggregations_only = false;

    while position < len {
        let token = &tokens[position];

        match &token.kind {
            TokenKind::Select => {
                if statements.contains_key("select") {
                    return Err(GQLError {
                        message: "you already used `select` statement ".to_owned(),
                        location: token.location,
                    });
                }

                let parse_result = parse_select_statement(
                    &tokens,
                    &mut position,
                    &mut aggregations,
                    &mut extra_type_table,
                    &mut hidden_selections,
                );

                if parse_result.is_err() {
                    return Err(parse_result.err().unwrap());
                }

                let select_info = parse_result.ok().unwrap();
                select_aggregations_only = select_info.1;
                statements.insert("select".to_string(), select_info.0);
            }
            TokenKind::Where => {
                if !statements.contains_key("select") {
                    return Err(GQLError {
                        message: "`WHERE` must be used after `SELECT` statement".to_owned(),
                        location: token.location,
                    });
                }

                if statements.contains_key("where") {
                    return Err(GQLError {
                        message: "you already used `where` statement".to_owned(),
                        location: token.location,
                    });
                }
                let parse_result = parse_where_statement(&tokens, &mut position);
                if parse_result.is_err() {
                    return Err(parse_result.err().unwrap());
                }
                statements.insert("where".to_string(), parse_result.ok().unwrap());
            }
            TokenKind::Group => {
                if !statements.contains_key("select") {
                    return Err(GQLError {
                        message: "`GROUP BY` must be used after `SELECT` statement".to_owned(),
                        location: token.location,
                    });
                }

                if statements.contains_key("group") {
                    return Err(GQLError {
                        message: "you already used `group by` statement".to_owned(),
                        location: token.location,
                    });
                }

                let parse_result = parse_group_by_statement(&tokens, &mut position);
                if parse_result.is_err() {
                    return Err(parse_result.err().unwrap());
                }

                statements.insert("group".to_string(), parse_result.ok().unwrap());
            }
            TokenKind::Having => {
                if statements.contains_key("having") {
                    return Err(GQLError {
                        message: "you already used `having` statement".to_owned(),
                        location: token.location,
                    });
                }

                if !statements.contains_key("group") {
                    return Err(GQLError {
                        message: "`HAVING` must be used after GROUP BY".to_owned(),
                        location: token.location,
                    });
                }

                let parse_result = parse_having_statement(&tokens, &mut position);
                if parse_result.is_err() {
                    return Err(parse_result.err().unwrap());
                }
                statements.insert("having".to_string(), parse_result.ok().unwrap());
            }
            TokenKind::Limit => {
                if !statements.contains_key("select") {
                    return Err(GQLError {
                        message: "`LIMIT` must be used after `SELECT` statement".to_owned(),
                        location: token.location,
                    });
                }

                if statements.contains_key("limit") {
                    return Err(GQLError {
                        message: "you already used `limit` statement".to_owned(),
                        location: token.location,
                    });
                }

                let parse_result = parse_limit_statement(&tokens, &mut position);
                if parse_result.is_err() {
                    return Err(parse_result.err().unwrap());
                }
                statements.insert("limit".to_string(), parse_result.ok().unwrap());
            }
            TokenKind::Offset => {
                if !statements.contains_key("select") {
                    return Err(GQLError {
                        message: "`OFFSET` must be used after `SELECT` statement".to_owned(),
                        location: token.location,
                    });
                }

                if statements.contains_key("offset") {
                    return Err(GQLError {
                        message: "you already used `offset` statement".to_owned(),
                        location: token.location,
                    });
                }

                let parse_result = parse_offset_statement(&tokens, &mut position);
                if parse_result.is_err() {
                    return Err(parse_result.err().unwrap());
                }
                statements.insert("offset".to_string(), parse_result.ok().unwrap());
            }
            TokenKind::Order => {
                if !statements.contains_key("select") {
                    return Err(GQLError {
                        message: "`ORDER BY` must be used after `SELECT` statement".to_owned(),
                        location: token.location,
                    });
                }

                if statements.contains_key("order") {
                    return Err(GQLError {
                        message: "you already used `order by` statement".to_owned(),
                        location: token.location,
                    });
                }

                let parse_result =
                    parse_order_by_statement(&tokens, &mut position, &mut extra_type_table);

                if parse_result.is_err() {
                    return Err(parse_result.err().unwrap());
                }
                statements.insert("order".to_string(), parse_result.ok().unwrap());
            }
            _ => {
                return Err(GQLError {
                    message: "Unexpected statement".to_owned(),
                    location: token.location,
                })
            }
        }
    }

    // If any aggregation function is used, add Aggregation Functions Node to the GQL Query
    if !aggregations.is_empty() {
        let aggregation_functions = AggregationFunctionsStatement { aggregations };
        statements.insert("aggregation".to_string(), Box::new(aggregation_functions));
    }

    return Ok(GQLQuery {
        statements,
        select_aggregations_only,
        hidden_selections,
    });
}

fn parse_select_statement(
    tokens: &Vec<Token>,
    position: &mut usize,
    aggregations: &mut HashMap<String, AggregateFunction>,
    extra_type_table: &mut HashMap<String, DataType>,
    hidden_selections: &mut Vec<String>,
) -> Result<(Box<dyn Statement>, bool), GQLError> {
    *position += 1;
    let mut selected_fields: Vec<String> = Vec::new();
    let mut fields_set: HashSet<String> = HashSet::new();
    let mut alias_table: HashMap<String, String> = HashMap::new();

    let mut select_aggregations_only = true;

    if *position >= tokens.len() {
        return Err(GQLError {
            message: "Expect * or fields names after the select keyword".to_owned(),
            location: tokens[*position - 1].location,
        });
    }

    if tokens[*position].kind == TokenKind::Star {
        *position += 1;
        select_aggregations_only = false;
    } else if tokens[*position].kind == TokenKind::Symbol {
        let mut fields_names: HashSet<String> = HashSet::new();
        let mut aggregation_function_index = 0;

        while *position < tokens.len() && tokens[*position].kind == TokenKind::Symbol {
            let field_name_result = consume_kind(&tokens[*position], TokenKind::Symbol);
            if field_name_result.is_err() {
                return Err(GQLError {
                    message: "Expect `identifier` as a field name".to_owned(),
                    location: tokens[*position].location,
                });
            }

            let field_name_token = field_name_result.ok().unwrap();
            let field_name_location = field_name_token.location;
            let field_name = field_name_token.literal.to_string();

            // Consume identifier as field or aggregation function name
            *position += 1;

            // Parse aggregation function
            if *position < tokens.len() && tokens[*position].kind == TokenKind::LeftParen {
                *position += 1;
                let argument_result = consume_kind(&tokens[*position], TokenKind::Symbol);
                if argument_result.is_err() {
                    return Err(GQLError {
                        message: "Expect `identifier` as aggregation function name".to_owned(),
                        location: tokens[*position].location,
                    });
                }

                let argument = argument_result.ok().unwrap();
                if !TABLES_FIELDS_TYPES.contains_key(argument.literal.as_str()) {
                    return Err(GQLError {
                        message: format!("No table has field with name `{}`", argument.literal),
                        location: tokens[*position].location,
                    });
                }

                if !fields_set.contains(&argument.literal) {
                    selected_fields.push(argument.literal.to_string());
                    hidden_selections.push(argument.literal.to_string());
                }

                // Consume argument
                *position += 1;

                // Consume `)`
                if *position < tokens.len() && tokens[*position].kind == TokenKind::RightParen {
                    *position += 1;
                } else {
                    return Err(GQLError {
                        message: "Expect `)` at the end of aggregation function".to_owned(),
                        location: tokens[*position].location,
                    });
                }

                // Check if aggregation function name is valid
                let function_name = field_name.to_lowercase();
                if !AGGREGATIONS.contains_key(function_name.as_str()) {
                    return Err(GQLError {
                        message: "Invalid GQL aggregation function name".to_owned(),
                        location: field_name_location,
                    });
                }

                // Type check aggregation function argument type
                let prototype = AGGREGATIONS_PROTOS.get(function_name.as_str()).unwrap();
                let field_type = TABLES_FIELDS_TYPES.get(argument.literal.as_str()).unwrap();
                if prototype.parameter != DataType::Any && field_type != &prototype.parameter {
                    let message = format!(
                        "Aggregation Function `{}` expect parameter type `{}` but got type `{}`",
                        function_name,
                        &prototype.parameter.literal(),
                        field_type.literal()
                    );
                    return Err(GQLError {
                        message: message,
                        location: argument.location,
                    });
                }

                let column_name =
                    if *position < tokens.len() && tokens[*position].kind == TokenKind::As {
                        *position += 1;

                        if *position >= tokens.len() {
                            return Err(GQLError {
                                message: "Expect `identifier` as a field alias name".to_owned(),
                                location: tokens[*position - 1].location,
                            });
                        }

                        let alias_name_result = consume_kind(&tokens[*position], TokenKind::Symbol);
                        if alias_name_result.is_err() {
                            return Err(GQLError {
                                message: "Expect `identifier` as a field alias name".to_owned(),
                                location: tokens[*position].location,
                            });
                        }

                        let alias_name = alias_name_result.ok().unwrap().literal.to_string();
                        if TABLES_FIELDS_TYPES.contains_key(&alias_name.as_str()) {
                            return Err(GQLError {
                                message: "You can't use column name as alias name".to_owned(),
                                location: tokens[*position].location,
                            });
                        }

                        if fields_set.contains(&alias_name) {
                            return Err(GQLError {
                                message: "There is already field or alias with the same name"
                                    .to_owned(),
                                location: tokens[*position].location,
                            });
                        }

                        *position += 1;

                        // Insert the alias name to used later in conditions
                        fields_set.insert(alias_name.to_string());

                        alias_name
                    } else {
                        aggregation_function_index += 1;
                        format!("{}_{}", "field", aggregation_function_index)
                    };

                extra_type_table.insert(column_name.to_string(), prototype.result.clone());

                aggregations.insert(
                    column_name.to_string(),
                    AggregateFunction {
                        function_name: function_name,
                        argument: argument.literal.to_string(),
                    },
                );

                if *position < tokens.len() && tokens[*position].kind == TokenKind::Comma {
                    *position += 1;
                }

                continue;
            }

            select_aggregations_only = false;
            if !fields_names.insert(field_name.to_string()) {
                return Err(GQLError {
                    message: "Can't select the same field twice".to_owned(),
                    location: tokens[*position - 1].location,
                });
            }

            let index = hidden_selections.iter().position(|r| r == &field_name);
            if let Some(position) = index {
                hidden_selections.remove(position);
            }

            selected_fields.push(field_name.to_string());

            if *position < tokens.len() && tokens[*position].kind == TokenKind::As {
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

                if TABLES_FIELDS_TYPES.contains_key(&alias_name.as_str()) {
                    return Err(GQLError {
                        message: "You can't use column name as alias name".to_owned(),
                        location: tokens[*position].location,
                    });
                }

                // Make sure there is a field with this name before alias
                if !TABLES_FIELDS_TYPES.contains_key(field_name.as_str()) {
                    return Err(GQLError {
                        message: format!("No table has field with name `{}`", field_name),
                        location: field_name_location,
                    });
                }

                // Update extra type table for this alias
                let field_type = TABLES_FIELDS_TYPES.get(field_name.as_str()).unwrap();
                extra_type_table.insert(alias_name.to_string(), field_type.clone());

                // Insert the alias name to used later in conditions
                fields_set.insert(alias_name.to_string());

                alias_table.insert(field_name, alias_name);
            } else {
                // Insert the origin name
                fields_set.insert(field_name.to_string());
            }

            if *position < tokens.len() && tokens[*position].kind == TokenKind::Comma {
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

    if *position >= tokens.len() || tokens[*position].kind != TokenKind::From {
        return Err(GQLError {
            message: "Expect `from` keyword after attributes".to_owned(),
            location: tokens[*position - 1].location,
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
    for field in &selected_fields {
        if !valid_fields.contains(&field.as_str()) {
            return Err(GQLError {
                message: format!("Table {} has no field with name {}", table_name, field),
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
        fields: selected_fields,
        alias_table,
    };

    return Ok((Box::new(statement), select_aggregations_only));
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

fn parse_group_by_statement(
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Statement>, GQLError> {
    *position += 1;
    if *position >= tokens.len() || tokens[*position].kind != TokenKind::By {
        return Err(GQLError {
            message: "Expect keyword `by` after keyword `group`".to_owned(),
            location: tokens[*position - 1].location,
        });
    }
    *position += 1;
    if *position >= tokens.len() || tokens[*position].kind != TokenKind::Symbol {
        return Err(GQLError {
            message: "Expect field name after `group by`".to_owned(),
            location: tokens[*position - 1].location,
        });
    }

    let field_name = tokens[*position].literal.to_string();
    *position += 1;

    return Ok(Box::new(GroupByStatement { field_name }));
}

fn parse_having_statement(
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

    return Ok(Box::new(HavingStatement {
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
    extra_type_table: &mut HashMap<String, DataType>,
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

    let field_type: DataType;
    if TABLES_FIELDS_TYPES.contains_key(field_name.as_str()) {
        field_type = TABLES_FIELDS_TYPES
            .get(field_name.as_str())
            .unwrap()
            .clone();
    } else if extra_type_table.contains_key(field_name.as_str()) {
        field_type = extra_type_table.get(field_name.as_str()).unwrap().clone();
    } else {
        return Err(GQLError {
            message: "Un resolved field name".to_owned(),
            location: tokens[*position].location,
        });
    }

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
        field_type,
    }));
}

fn parse_expression(
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, GQLError> {
    return parse_between_expression(tokens, position);
}

fn parse_between_expression(
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, GQLError> {
    let expression = parse_logical_or_expression(tokens, position);
    if expression.is_err() {
        return expression;
    }

    if *position < tokens.len() && tokens[*position].kind == TokenKind::Between {
        let between_location = tokens[*position].location;

        // Consume Between keyword
        *position += 1;

        let value = expression.ok().unwrap();
        if value.expr_type() != DataType::Number {
            return Err(GQLError {
                message: format!(
                    "BETWEEN value must to be Number type but got {}",
                    value.expr_type().literal()
                ),
                location: between_location,
            });
        }

        if *position >= tokens.len() {
            return Err(GQLError {
                message: "Between keyword expects two range after it".to_owned(),
                location: between_location,
            });
        }

        let range_start_result = parse_logical_or_expression(tokens, position);
        if range_start_result.is_err() {
            return range_start_result;
        }

        let range_start = range_start_result.ok().unwrap();
        if range_start.expr_type() != DataType::Number {
            return Err(GQLError {
                message: format!(
                    "Expect range start to be Number type but got {}",
                    range_start.expr_type().literal()
                ),
                location: between_location,
            });
        }

        if *position >= tokens.len() || tokens[*position].kind != TokenKind::DotDot {
            return Err(GQLError {
                message: "Expect `..` after BETWEEN range start".to_owned(),
                location: between_location,
            });
        }

        // Consume AND keyword
        *position += 1;

        let range_end_result = parse_logical_or_expression(tokens, position);
        if range_end_result.is_err() {
            return range_end_result;
        }

        let range_end = range_end_result.ok().unwrap();
        if range_end.expr_type() != DataType::Number {
            return Err(GQLError {
                message: format!(
                    "Expect range end to be Number type but got {}",
                    range_end.expr_type().literal()
                ),
                location: between_location,
            });
        }

        return Ok(Box::new(BetweenExpression {
            value,
            range_start,
            range_end,
        }));
    }
    return expression;
}

fn parse_logical_or_expression(
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, GQLError> {
    let expression = parse_logical_and_expression(tokens, position);
    if expression.is_err() || *position >= tokens.len() {
        return expression;
    }

    let lhs = expression.ok().unwrap();

    let operator = &tokens[*position];

    if operator.kind == TokenKind::LogicalOr {
        *position += 1;

        if lhs.expr_type() != DataType::Boolean {
            return Err(type_missmatch_error(
                tokens[*position - 2].location,
                DataType::Boolean,
                lhs.expr_type(),
            ));
        }

        let right_expr = parse_logical_and_expression(tokens, position);
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
            operator: LogicalOperator::Or,
            right: rhs,
        }));
    }

    return Ok(lhs);
}

fn parse_logical_and_expression(
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, GQLError> {
    let expression = parse_bitwise_or_expression(tokens, position);
    if expression.is_err() || *position >= tokens.len() {
        return expression;
    }

    let lhs = expression.ok().unwrap();

    let operator = &tokens[*position];

    if operator.kind == TokenKind::LogicalAnd {
        *position += 1;

        if lhs.expr_type() != DataType::Boolean {
            return Err(type_missmatch_error(
                tokens[*position - 2].location,
                DataType::Boolean,
                lhs.expr_type(),
            ));
        }

        let right_expr = parse_bitwise_or_expression(tokens, position);
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
            operator: LogicalOperator::And,
            right: rhs,
        }));
    }

    return Ok(lhs);
}

fn parse_bitwise_or_expression(
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, GQLError> {
    let expression = parse_logical_xor_expression(tokens, position);
    if expression.is_err() || *position >= tokens.len() {
        return expression;
    }

    let lhs = expression.ok().unwrap();

    let operator = &tokens[*position];

    if operator.kind == TokenKind::BitwiseOr {
        *position += 1;

        if lhs.expr_type() != DataType::Boolean {
            return Err(type_missmatch_error(
                tokens[*position - 2].location,
                DataType::Boolean,
                lhs.expr_type(),
            ));
        }

        let right_expr = parse_logical_xor_expression(tokens, position);
        if right_expr.is_err() {
            return Err(GQLError {
                message: "Can't parser right side of bitwise or expression".to_owned(),
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

        return Ok(Box::new(BitwiseExpression {
            left: lhs,
            operator: BitwiseOperator::Or,
            right: rhs,
        }));
    }

    return Ok(lhs);
}

fn parse_logical_xor_expression(
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, GQLError> {
    let expression = parse_bitwise_and_expression(tokens, position);
    if expression.is_err() || *position >= tokens.len() {
        return expression;
    }

    let lhs = expression.ok().unwrap();

    let operator = &tokens[*position];

    if operator.kind == TokenKind::LogicalXor {
        *position += 1;

        if lhs.expr_type() != DataType::Boolean {
            return Err(type_missmatch_error(
                tokens[*position - 2].location,
                DataType::Boolean,
                lhs.expr_type(),
            ));
        }

        let right_expr = parse_bitwise_and_expression(tokens, position);
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
            operator: LogicalOperator::Xor,
            right: rhs,
        }));
    }

    return Ok(lhs);
}

fn parse_bitwise_and_expression(
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, GQLError> {
    let expression = parse_equality_expression(tokens, position);
    if expression.is_err() || *position >= tokens.len() {
        return expression;
    }

    let lhs = expression.ok().unwrap();

    let operator = &tokens[*position];

    if operator.kind == TokenKind::BitwiseAnd {
        *position += 1;

        if lhs.expr_type() != DataType::Boolean {
            return Err(type_missmatch_error(
                tokens[*position - 2].location,
                DataType::Boolean,
                lhs.expr_type(),
            ));
        }

        let right_expr = parse_equality_expression(tokens, position);
        if right_expr.is_err() {
            return Err(GQLError {
                message: "Can't parser right side of bitwise and expression".to_owned(),
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

        return Ok(Box::new(BitwiseExpression {
            left: lhs,
            operator: BitwiseOperator::And,
            right: rhs,
        }));
    }

    return Ok(lhs);
}

fn parse_equality_expression(
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, GQLError> {
    let expression = parse_comparison_expression(tokens, position);
    if expression.is_err() || *position >= tokens.len() {
        return expression;
    }

    let lhs = expression.ok().unwrap();

    let operator = &tokens[*position];
    if operator.kind == TokenKind::Equal || operator.kind == TokenKind::Bang {
        *position += 1;
        let comparison_operator = if operator.kind == TokenKind::Equal {
            ComparisonOperator::Equal
        } else {
            ComparisonOperator::NotEqual
        };

        let right_expr = parse_comparison_expression(tokens, position);
        if right_expr.is_err() {
            return Err(right_expr.err().unwrap());
        }

        let rhs = right_expr.ok().unwrap();

        // Make sure right and left hand side types are the same
        if rhs.expr_type() != lhs.expr_type() {
            let message = format!(
                "Can't compare values of different types `{}` and `{}`",
                lhs.expr_type().literal(),
                rhs.expr_type().literal()
            );
            return Err(GQLError {
                message: message,
                location: tokens[*position - 2].location,
            });
        }

        return Ok(Box::new(ComparisonExpression {
            left: lhs,
            operator: comparison_operator,
            right: rhs,
        }));
    }

    return Ok(lhs);
}

fn parse_comparison_expression(
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, GQLError> {
    let expression = parse_bitwise_shift_expression(tokens, position);
    if expression.is_err() || *position >= tokens.len() {
        return expression;
    }

    let lhs = expression.ok().unwrap();
    if is_comparison_operator(&tokens[*position]) {
        let operator = &tokens[*position];
        *position += 1;
        let comparison_operator = match operator.kind {
            TokenKind::Greater => ComparisonOperator::Greater,
            TokenKind::GreaterEqual => ComparisonOperator::GreaterEqual,
            TokenKind::Less => ComparisonOperator::Less,
            _ => ComparisonOperator::LessEqual,
        };

        let right_expr = parse_bitwise_shift_expression(tokens, position);
        if right_expr.is_err() {
            return Err(right_expr.err().unwrap());
        }

        let rhs = right_expr.ok().unwrap();

        // Make sure right and left hand side types are the same
        if rhs.expr_type() != lhs.expr_type() {
            let message = format!(
                "Can't compare values of different types `{}` and `{}`",
                lhs.expr_type().literal(),
                rhs.expr_type().literal()
            );
            return Err(GQLError {
                message: message,
                location: tokens[*position - 2].location,
            });
        }

        return Ok(Box::new(ComparisonExpression {
            left: lhs,
            operator: comparison_operator,
            right: rhs,
        }));
    }

    return Ok(lhs);
}

fn parse_bitwise_shift_expression(
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, GQLError> {
    let expression = parse_term_expression(tokens, position);
    if expression.is_err() || *position >= tokens.len() {
        return expression;
    }

    let mut lhs = expression.ok().unwrap();

    while *position < tokens.len()
        && (&tokens[*position].kind == &TokenKind::BitwiseRightShift
            || &tokens[*position].kind == &TokenKind::BitwiseLeftShift)
    {
        let operator = &tokens[*position];
        *position += 1;
        let bitwise_operator = if operator.kind == TokenKind::BitwiseRightShift {
            BitwiseOperator::RightShift
        } else {
            BitwiseOperator::LeftShift
        };

        let right_expr = parse_term_expression(tokens, position);
        if right_expr.is_err() {
            return Err(right_expr.err().unwrap());
        }

        let rhs = right_expr.ok().unwrap();

        // Make sure right and left hand side types are numbers
        if rhs.expr_type() == DataType::Number && rhs.expr_type() != lhs.expr_type() {
            let message = format!(
                "Bitwise operators require number types but got `{}` and `{}`",
                lhs.expr_type().literal(),
                rhs.expr_type().literal()
            );
            return Err(GQLError {
                message: message,
                location: tokens[*position - 2].location,
            });
        }

        lhs = Box::new(BitwiseExpression {
            left: lhs,
            operator: bitwise_operator,
            right: rhs,
        });
    }

    return Ok(lhs);
}

fn parse_term_expression(
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, GQLError> {
    let expression = parse_factor_expression(tokens, position);
    if expression.is_err() || *position >= tokens.len() {
        return expression;
    }

    let mut lhs = expression.ok().unwrap();

    while *position < tokens.len()
        && (&tokens[*position].kind == &TokenKind::Plus
            || &tokens[*position].kind == &TokenKind::Minus)
    {
        let operator = &tokens[*position];
        *position += 1;
        let math_operator = if operator.kind == TokenKind::Plus {
            ArithmeticOperator::Plus
        } else {
            ArithmeticOperator::Minus
        };

        let right_expr = parse_factor_expression(tokens, position);
        if right_expr.is_err() {
            return Err(right_expr.err().unwrap());
        }

        let rhs = right_expr.ok().unwrap();

        // Make sure right and left hand side types are numbers
        if rhs.expr_type() == DataType::Number && rhs.expr_type() != lhs.expr_type() {
            let message = format!(
                "Math operators require number types but got `{}` and `{}`",
                lhs.expr_type().literal(),
                rhs.expr_type().literal()
            );
            return Err(GQLError {
                message: message,
                location: tokens[*position - 2].location,
            });
        }

        lhs = Box::new(ArithmeticExpression {
            left: lhs,
            operator: math_operator,
            right: rhs,
        });
    }

    return Ok(lhs);
}

fn parse_factor_expression(
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, GQLError> {
    let expression = parse_check_expression(tokens, position);
    if expression.is_err() || *position >= tokens.len() {
        return expression;
    }

    let mut lhs = expression.ok().unwrap();
    while *position < tokens.len() && is_factor_operator(&tokens[*position]) {
        let operator = &tokens[*position];
        *position += 1;

        let factor_operator = match operator.kind {
            TokenKind::Star => ArithmeticOperator::Star,
            TokenKind::Slash => ArithmeticOperator::Slash,
            _ => ArithmeticOperator::Modulus,
        };

        let right_expr = parse_check_expression(tokens, position);
        if right_expr.is_err() {
            return Err(right_expr.err().unwrap());
        }

        let rhs = right_expr.ok().unwrap();

        // Make sure right and left hand side types are numbers
        if rhs.expr_type() == DataType::Number && rhs.expr_type() != lhs.expr_type() {
            let message = format!(
                "Math operators require number types but got `{}` and `{}`",
                lhs.expr_type().literal(),
                rhs.expr_type().literal()
            );
            return Err(GQLError {
                message: message,
                location: tokens[*position - 2].location,
            });
        }

        lhs = Box::new(ArithmeticExpression {
            left: lhs,
            operator: factor_operator,
            right: rhs,
        });
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
        TokenKind::Number => {
            *position += 1;
            let value = tokens[*position - 1].literal.parse::<i64>().unwrap();
            return Ok(Box::new(NumberExpression { value }));
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

#[inline(always)]
fn consume_kind(token: &Token, kind: TokenKind) -> Result<&Token, i32> {
    if token.kind == kind {
        return Ok(token);
    }
    return Err(0);
}

#[inline(always)]
fn is_comparison_operator(token: &Token) -> bool {
    return token.kind == TokenKind::Greater
        || token.kind == TokenKind::GreaterEqual
        || token.kind == TokenKind::Less
        || token.kind == TokenKind::LessEqual;
}

#[inline(always)]
fn is_factor_operator(token: &Token) -> bool {
    return token.kind == TokenKind::Star
        || token.kind == TokenKind::Slash
        || token.kind == TokenKind::Percentage;
}

#[inline(always)]
fn type_missmatch_error(location: Location, expected: DataType, actual: DataType) -> GQLError {
    let message = format!(
        "Type mismatch expected `{}`, got `{}`",
        expected.literal(),
        actual.literal()
    );
    return GQLError { message, location };
}
