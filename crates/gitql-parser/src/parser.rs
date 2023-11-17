use gitql_ast::scope::Scope;
use gitql_ast::scope::TABLES_FIELDS_NAMES;
use gitql_ast::value::Value;
use std::collections::HashMap;
use std::vec;

use crate::context::ParserContext;
use crate::diagnostic::GQLError;
use crate::tokenizer::Location;
use crate::tokenizer::Token;
use crate::tokenizer::TokenKind;

use gitql_ast::aggregation::AGGREGATIONS;
use gitql_ast::aggregation::AGGREGATIONS_PROTOS;
use gitql_ast::expression::*;
use gitql_ast::function::FUNCTIONS;
use gitql_ast::function::PROTOTYPES;
use gitql_ast::statement::*;
use gitql_ast::types::DataType;
use gitql_ast::types::TABLES_FIELDS_TYPES;

pub fn parse_gql(tokens: Vec<Token>) -> Result<GQLQuery, GQLError> {
    let len = tokens.len();
    let mut position = 0;

    let mut context = ParserContext::default();
    let mut statements: HashMap<String, Box<dyn Statement>> = HashMap::new();

    while position < len {
        let token = &tokens[position];

        match &token.kind {
            TokenKind::Select => {
                if statements.contains_key("select") {
                    return Err(GQLError {
                        message: "You already used `select` statement ".to_owned(),
                        location: token.location,
                    });
                }
                let statement = parse_select_statement(&mut context, &tokens, &mut position)?;
                statements.insert("select".to_string(), statement);
                context.is_single_value_query = !context.aggregations.is_empty();
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
                        message: "You already used `where` statement".to_owned(),
                        location: token.location,
                    });
                }

                let statement = parse_where_statement(&mut context, &tokens, &mut position)?;
                statements.insert("where".to_string(), statement);
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
                        message: "You already used `group by` statement".to_owned(),
                        location: token.location,
                    });
                }

                let statement = parse_group_by_statement(&mut context, &tokens, &mut position)?;
                statements.insert("group".to_string(), statement);
            }
            TokenKind::Having => {
                if statements.contains_key("having") {
                    return Err(GQLError {
                        message: "You already used `having` statement".to_owned(),
                        location: token.location,
                    });
                }

                if !statements.contains_key("group") {
                    return Err(GQLError {
                        message: "`HAVING` must be used after GROUP BY".to_owned(),
                        location: token.location,
                    });
                }

                let statement = parse_having_statement(&mut context, &tokens, &mut position)?;
                statements.insert("having".to_string(), statement);
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

                let statement = parse_limit_statement(&tokens, &mut position)?;
                statements.insert("limit".to_string(), statement);
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

                let statement = parse_offset_statement(&tokens, &mut position)?;
                statements.insert("offset".to_string(), statement);
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

                let statement = parse_order_by_statement(&mut context, &tokens, &mut position)?;
                statements.insert("order".to_string(), statement);
            }
            _ => return Err(un_expected_statement_error(&tokens, &mut position)),
        }
    }

    // If any aggregation function is used, add Aggregation Functions Node to the GQL Query
    if !context.aggregations.is_empty() {
        let aggregation_functions = AggregationFunctionsStatement {
            aggregations: context.aggregations,
        };
        statements.insert("aggregation".to_string(), Box::new(aggregation_functions));
    }

    // Remove all selected fields from hidden selection
    let hidden_selections: Vec<String> = context
        .hidden_selections
        .iter()
        .filter(|n| !context.selected_fields.contains(n))
        .cloned()
        .collect();

    Ok(GQLQuery {
        statements,
        has_aggregation_function: context.is_single_value_query,
        has_group_by_statement: context.has_group_by_statement,
        hidden_selections,
    })
}

fn parse_select_statement(
    context: &mut ParserContext,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Statement>, GQLError> {
    // Consume select keyword
    *position += 1;

    if *position >= tokens.len() {
        return Err(GQLError {
            message: "Incomplete input for select statement".to_owned(),
            location: get_safe_location(tokens, *position - 1),
        });
    }

    let mut table_name = "";
    let mut fields_names: Vec<String> = Vec::new();
    let mut fields_values: Vec<Box<dyn Expression>> = Vec::new();
    let mut alias_table: HashMap<String, String> = HashMap::new();
    let mut is_select_all = false;
    let mut is_distinct = false;

    // Check if select has distinct keyword after it
    if tokens[*position].kind == TokenKind::Distinct {
        is_distinct = true;
        *position += 1;
    }

    // Select all option
    if *position < tokens.len() && tokens[*position].kind == TokenKind::Star {
        // Consume `*`
        *position += 1;
        is_select_all = true;
    } else {
        while *position < tokens.len() && tokens[*position].kind != TokenKind::From {
            let expression = parse_expression(context, tokens, position)?;
            let expr_type = expression.expr_type(&context.symbol_table).clone();
            let expression_name = get_expression_name(&expression);
            let field_name = if expression_name.is_ok() {
                expression_name.ok().unwrap()
            } else {
                context.generate_column_name()
            };

            // Assert that each selected field is unique
            if fields_names.contains(&field_name) {
                return Err(GQLError {
                    message: "Can't select the same field twice".to_owned(),
                    location: get_safe_location(tokens, *position - 1),
                });
            }

            // Check for Field name alias
            if *position < tokens.len() && tokens[*position].kind == TokenKind::As {
                // Consume `as` keyword
                *position += 1;
                let alias_name_token = consume_kind(tokens, *position, TokenKind::Symbol);
                if alias_name_token.is_err() {
                    return Err(GQLError {
                        message: "Expect `identifier` as field alias name".to_owned(),
                        location: get_safe_location(tokens, *position),
                    });
                }

                // Register alias name
                let alias_name = alias_name_token.ok().unwrap().literal.to_string();
                if context.selected_fields.contains(&alias_name)
                    || alias_table.contains_key(&alias_name)
                {
                    return Err(GQLError {
                        message: "You already have field with the same name".to_owned(),
                        location: get_safe_location(tokens, *position),
                    });
                }

                // Consume alias name
                *position += 1;

                // Register alias name type
                context
                    .symbol_table
                    .define(alias_name.to_string(), expr_type.clone());

                context.selected_fields.push(alias_name.clone());
                alias_table.insert(field_name.to_string(), alias_name);
            }

            // Register field type
            context
                .symbol_table
                .define(field_name.to_string(), expr_type);

            fields_names.push(field_name.to_owned());
            context.selected_fields.push(field_name.to_owned());
            fields_values.push(expression);

            // Consume `,` or break
            if *position < tokens.len() && tokens[*position].kind == TokenKind::Comma {
                *position += 1;
            } else {
                break;
            }
        }
    }

    // Parse optional Form statement
    if *position < tokens.len() && tokens[*position].kind == TokenKind::From {
        // Consume `from` keyword
        *position += 1;

        let table_name_token = consume_kind(tokens, *position, TokenKind::Symbol);
        if table_name_token.is_err() {
            return Err(GQLError {
                message: "Expect `identifier` as a table name".to_owned(),
                location: get_safe_location(tokens, *position),
            });
        }

        // Consume table name
        *position += 1;

        table_name = &table_name_token.ok().unwrap().literal;
        if !TABLES_FIELDS_NAMES.contains_key(table_name) {
            return Err(GQLError {
                message: "Unresolved table name".to_owned(),
                location: get_safe_location(tokens, *position),
            });
        }

        register_current_table_fields_types(table_name, &mut context.symbol_table);
    }

    // Make sure `SELECT *` used with specific table
    if is_select_all && table_name.is_empty() {
        return Err(GQLError {
            message: "Expect `FROM` and table name after `SELECT *`".to_owned(),
            location: get_safe_location(tokens, *position),
        });
    }

    // Select input validations
    if !is_select_all && fields_names.is_empty() {
        return Err(GQLError {
            message: "Incomplete input for select statement".to_owned(),
            location: get_safe_location(tokens, *position - 1),
        });
    }

    // If it `select *` make all table fields selectable
    if is_select_all {
        select_all_table_fields(
            table_name,
            &mut context.selected_fields,
            &mut fields_names,
            &mut fields_values,
        );
    }

    // Type check all selected fields has type regsited in type table
    type_check_selected_fields(
        &context.symbol_table,
        table_name,
        &fields_names,
        tokens,
        *position,
    )?;

    Ok(Box::new(SelectStatement {
        table_name: table_name.to_string(),
        fields_names,
        fields_values,
        alias_table,
        is_distinct,
    }))
}

fn parse_where_statement(
    context: &mut ParserContext,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Statement>, GQLError> {
    *position += 1;
    if *position >= tokens.len() {
        return Err(GQLError {
            message: "Expect expression after `WHERE` keyword".to_owned(),
            location: get_safe_location(tokens, *position - 1),
        });
    }

    let aggregations_count_before = context.aggregations.len();

    // Make sure WHERE condition expression has boolean type
    let condition_location = tokens[*position].location;
    let condition = parse_expression(context, tokens, position)?;
    let condition_type = condition.expr_type(&context.symbol_table);
    if condition_type != DataType::Boolean {
        return Err(GQLError {
            message: format!(
                "Expect `WHERE` condition bo be type {} but got {}",
                DataType::Boolean.literal(),
                condition_type.literal()
            ),
            location: condition_location,
        });
    }

    let aggregations_count_after = context.aggregations.len();
    if aggregations_count_before != aggregations_count_after {
        return Err(GQLError {
            message: String::from("Can't use Aggregation functions in `WHERE` statement"),
            location: condition_location,
        });
    }

    Ok(Box::new(WhereStatement { condition }))
}

fn parse_group_by_statement(
    context: &mut ParserContext,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Statement>, GQLError> {
    *position += 1;
    if *position >= tokens.len() || tokens[*position].kind != TokenKind::By {
        return Err(GQLError {
            message: "Expect keyword `by` after keyword `group`".to_owned(),
            location: get_safe_location(tokens, *position - 1),
        });
    }
    *position += 1;
    if *position >= tokens.len() || tokens[*position].kind != TokenKind::Symbol {
        return Err(GQLError {
            message: "Expect field name after `group by`".to_owned(),
            location: get_safe_location(tokens, *position - 1),
        });
    }

    let field_name = tokens[*position].literal.to_string();
    *position += 1;

    if !context.symbol_table.contains(&field_name) {
        return Err(GQLError {
            message: "Current table not contains field with this name".to_owned(),
            location: get_safe_location(tokens, *position - 1),
        });
    }

    context.has_group_by_statement = true;
    Ok(Box::new(GroupByStatement { field_name }))
}

fn parse_having_statement(
    context: &mut ParserContext,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Statement>, GQLError> {
    *position += 1;
    if *position >= tokens.len() {
        return Err(GQLError {
            message: "Expect expression after `where` keyword".to_owned(),
            location: get_safe_location(tokens, *position - 1),
        });
    }

    // Make sure HAVING condition expression has boolean type
    let condition_location = tokens[*position].location;
    let condition = parse_expression(context, tokens, position)?;
    let condition_type = condition.expr_type(&context.symbol_table);
    if condition_type != DataType::Boolean {
        return Err(GQLError {
            message: format!(
                "Expect `HAVING` condition bo be type {} but got {}",
                DataType::Boolean.literal(),
                condition_type.literal()
            ),
            location: condition_location,
        });
    }

    Ok(Box::new(HavingStatement { condition }))
}

fn parse_limit_statement(
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Statement>, GQLError> {
    *position += 1;
    if *position >= tokens.len() || tokens[*position].kind != TokenKind::Integer {
        return Err(GQLError {
            message: "Expect number after `LIMIT` keyword".to_owned(),
            location: get_safe_location(tokens, *position - 1),
        });
    }

    let count_str = tokens[*position].literal.to_string();
    let count: usize = count_str.parse().unwrap();
    *position += 1;
    Ok(Box::new(LimitStatement { count }))
}

fn parse_offset_statement(
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Statement>, GQLError> {
    *position += 1;
    if *position >= tokens.len() || tokens[*position].kind != TokenKind::Integer {
        return Err(GQLError {
            message: "Expect number after `OFFSET` keyword".to_owned(),
            location: get_safe_location(tokens, *position - 1),
        });
    }

    let count_str = tokens[*position].literal.to_string();
    let count: usize = count_str.parse().unwrap();
    *position += 1;
    Ok(Box::new(OffsetStatement { count }))
}

fn parse_order_by_statement(
    context: &mut ParserContext,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Statement>, GQLError> {
    // Consume `ORDER` keyword
    *position += 1;

    if *position >= tokens.len() || tokens[*position].kind != TokenKind::By {
        return Err(GQLError {
            message: "Expect keyword `BY` after keyword `ORDER`".to_owned(),
            location: get_safe_location(tokens, *position - 1),
        });
    }

    // Consume `BY` keyword
    *position += 1;

    let mut arguments: Vec<Box<dyn Expression>> = vec![];
    let mut sorting_orders: Vec<SortingOrder> = vec![];

    loop {
        let argument = parse_expression(context, tokens, position)?;
        arguments.push(argument);

        let mut order = SortingOrder::Ascending;
        if *position < tokens.len() && is_asc_or_desc(&tokens[*position]) {
            if tokens[*position].kind == TokenKind::Descending {
                order = SortingOrder::Descending;
            }

            // Consume `ASC or DESC` keyword
            *position += 1;
        }

        sorting_orders.push(order);
        if *position < tokens.len() && tokens[*position].kind == TokenKind::Comma {
            // Consume `,` keyword
            *position += 1;
        } else {
            break;
        }
    }

    Ok(Box::new(OrderByStatement {
        arguments,
        sorting_orders,
    }))
}

fn parse_expression(
    context: &mut ParserContext,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, GQLError> {
    parse_is_null_expression(context, tokens, position)
}

fn parse_is_null_expression(
    context: &mut ParserContext,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, GQLError> {
    let expression = parse_in_expression(context, tokens, position)?;
    if *position < tokens.len() && tokens[*position].kind == TokenKind::Is {
        let is_location = tokens[*position].location;

        // Consume `IS` keyword
        *position += 1;

        let has_not_keyword =
            if *position < tokens.len() && tokens[*position].kind == TokenKind::Not {
                // Consume `NOT` keyword
                *position += 1;
                true
            } else {
                false
            };

        if *position < tokens.len() && tokens[*position].kind == TokenKind::Null {
            // Consume `Null` keyword
            *position += 1;
        } else {
            return Err(GQLError {
                message: "Expects `NULL` Keyword after `IS` or `IS NOT`".to_owned(),
                location: is_location,
            });
        }

        return Ok(Box::new(IsNullExpression {
            argument: expression,
            has_not: has_not_keyword,
        }));
    }
    Ok(expression)
}

fn parse_in_expression(
    context: &mut ParserContext,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, GQLError> {
    let expression = parse_between_expression(context, tokens, position)?;
    if *position < tokens.len() && tokens[*position].kind == TokenKind::In {
        let in_location = tokens[*position].location;

        // Consume `IN` keyword
        *position += 1;

        if consume_kind(tokens, *position, TokenKind::LeftParen).is_err() {
            return Err(GQLError {
                message: "Expects values between `(` and `)` after `IN` keyword".to_owned(),
                location: in_location,
            });
        }

        let values = parse_arguments_expressions(context, tokens, position)?;
        let values_type_result = check_all_values_are_same_type(context, &values);
        if values_type_result.is_err() {
            return Err(GQLError {
                message: "Expects values between `(` and `)` to have the same type".to_owned(),
                location: in_location,
            });
        }

        // Check that argument and values has the same type
        let values_type = values_type_result.ok().unwrap();
        if values_type != DataType::Any
            && expression.expr_type(&context.symbol_table) != values_type
        {
            return Err(GQLError {
                message: "Argument and Values of In Expression must have the same type".to_owned(),
                location: in_location,
            });
        }

        return Ok(Box::new(InExpression {
            argument: expression,
            values,
            values_type,
        }));
    }
    Ok(expression)
}

fn parse_between_expression(
    context: &mut ParserContext,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, GQLError> {
    let expression = parse_logical_or_expression(context, tokens, position)?;

    if *position < tokens.len() && tokens[*position].kind == TokenKind::Between {
        let between_location = tokens[*position].location;

        // Consume `BETWEEN` keyword
        *position += 1;

        if expression.expr_type(&context.symbol_table) != DataType::Integer {
            return Err(GQLError {
                message: format!(
                    "BETWEEN value must to be Number type but got {}",
                    expression.expr_type(&context.symbol_table).literal()
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

        let range_start = parse_logical_or_expression(context, tokens, position)?;
        if range_start.expr_type(&context.symbol_table) != DataType::Integer {
            return Err(GQLError {
                message: format!(
                    "Expect range start to be Number type but got {}",
                    range_start.expr_type(&context.symbol_table).literal()
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

        // Consume `..` keyword
        *position += 1;

        let range_end = parse_logical_or_expression(context, tokens, position)?;
        if range_end.expr_type(&context.symbol_table) != DataType::Integer {
            return Err(GQLError {
                message: format!(
                    "Expect range end to be Number type but got {}",
                    range_end.expr_type(&context.symbol_table).literal()
                ),
                location: between_location,
            });
        }

        return Ok(Box::new(BetweenExpression {
            value: expression,
            range_start,
            range_end,
        }));
    }

    Ok(expression)
}

fn parse_logical_or_expression(
    context: &mut ParserContext,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, GQLError> {
    let expression = parse_logical_and_expression(context, tokens, position);
    if expression.is_err() || *position >= tokens.len() {
        return expression;
    }

    let lhs = expression.ok().unwrap();

    let operator = &tokens[*position];

    if operator.kind == TokenKind::LogicalOr {
        *position += 1;

        if lhs.expr_type(&context.symbol_table) != DataType::Boolean {
            return Err(type_missmatch_error(
                tokens[*position - 2].location,
                DataType::Boolean,
                lhs.expr_type(&context.symbol_table),
            ));
        }

        let rhs = parse_logical_and_expression(context, tokens, position)?;
        if rhs.expr_type(&context.symbol_table) != DataType::Boolean {
            return Err(type_missmatch_error(
                tokens[*position].location,
                DataType::Boolean,
                lhs.expr_type(&context.symbol_table),
            ));
        }

        return Ok(Box::new(LogicalExpression {
            left: lhs,
            operator: LogicalOperator::Or,
            right: rhs,
        }));
    }

    Ok(lhs)
}

fn parse_logical_and_expression(
    context: &mut ParserContext,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, GQLError> {
    let expression = parse_bitwise_or_expression(context, tokens, position);
    if expression.is_err() || *position >= tokens.len() {
        return expression;
    }

    let lhs = expression.ok().unwrap();

    let operator = &tokens[*position];

    if operator.kind == TokenKind::LogicalAnd {
        *position += 1;

        if lhs.expr_type(&context.symbol_table) != DataType::Boolean {
            return Err(type_missmatch_error(
                tokens[*position - 2].location,
                DataType::Boolean,
                lhs.expr_type(&context.symbol_table),
            ));
        }

        let rhs = parse_bitwise_or_expression(context, tokens, position)?;
        if rhs.expr_type(&context.symbol_table) != DataType::Boolean {
            return Err(type_missmatch_error(
                tokens[*position].location,
                DataType::Boolean,
                lhs.expr_type(&context.symbol_table),
            ));
        }

        return Ok(Box::new(LogicalExpression {
            left: lhs,
            operator: LogicalOperator::And,
            right: rhs,
        }));
    }

    Ok(lhs)
}

fn parse_bitwise_or_expression(
    context: &mut ParserContext,

    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, GQLError> {
    let expression = parse_logical_xor_expression(context, tokens, position);
    if expression.is_err() || *position >= tokens.len() {
        return expression;
    }

    let lhs = expression.ok().unwrap();

    let operator = &tokens[*position];

    if operator.kind == TokenKind::BitwiseOr {
        *position += 1;

        if lhs.expr_type(&context.symbol_table) != DataType::Boolean {
            return Err(type_missmatch_error(
                tokens[*position - 2].location,
                DataType::Boolean,
                lhs.expr_type(&context.symbol_table),
            ));
        }

        let rhs = parse_logical_xor_expression(context, tokens, position)?;
        if rhs.expr_type(&context.symbol_table) != DataType::Boolean {
            return Err(type_missmatch_error(
                tokens[*position].location,
                DataType::Boolean,
                lhs.expr_type(&context.symbol_table),
            ));
        }

        return Ok(Box::new(BitwiseExpression {
            left: lhs,
            operator: BitwiseOperator::Or,
            right: rhs,
        }));
    }

    Ok(lhs)
}

fn parse_logical_xor_expression(
    context: &mut ParserContext,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, GQLError> {
    let expression = parse_bitwise_and_expression(context, tokens, position);
    if expression.is_err() || *position >= tokens.len() {
        return expression;
    }

    let lhs = expression.ok().unwrap();

    let operator = &tokens[*position];

    if operator.kind == TokenKind::LogicalXor {
        *position += 1;

        if lhs.expr_type(&context.symbol_table) != DataType::Boolean {
            return Err(type_missmatch_error(
                tokens[*position - 2].location,
                DataType::Boolean,
                lhs.expr_type(&context.symbol_table),
            ));
        }

        let rhs = parse_bitwise_and_expression(context, tokens, position)?;
        if rhs.expr_type(&context.symbol_table) != DataType::Boolean {
            return Err(type_missmatch_error(
                tokens[*position].location,
                DataType::Boolean,
                lhs.expr_type(&context.symbol_table),
            ));
        }

        return Ok(Box::new(LogicalExpression {
            left: lhs,
            operator: LogicalOperator::Xor,
            right: rhs,
        }));
    }

    Ok(lhs)
}

fn parse_bitwise_and_expression(
    context: &mut ParserContext,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, GQLError> {
    let expression = parse_equality_expression(context, tokens, position);
    if expression.is_err() || *position >= tokens.len() {
        return expression;
    }

    let lhs = expression.ok().unwrap();

    let operator = &tokens[*position];

    if operator.kind == TokenKind::BitwiseAnd {
        *position += 1;

        if lhs.expr_type(&context.symbol_table) != DataType::Boolean {
            return Err(type_missmatch_error(
                tokens[*position - 2].location,
                DataType::Boolean,
                lhs.expr_type(&context.symbol_table),
            ));
        }

        let right_expr = parse_equality_expression(context, tokens, position);
        if right_expr.is_err() {
            return Err(GQLError {
                message: "Can't parser right side of bitwise and expression".to_owned(),
                location: get_safe_location(tokens, *position),
            });
        }

        let rhs = right_expr.ok().unwrap();
        if rhs.expr_type(&context.symbol_table) != DataType::Boolean {
            return Err(type_missmatch_error(
                tokens[*position].location,
                DataType::Boolean,
                lhs.expr_type(&context.symbol_table),
            ));
        }

        return Ok(Box::new(BitwiseExpression {
            left: lhs,
            operator: BitwiseOperator::And,
            right: rhs,
        }));
    }

    Ok(lhs)
}

fn parse_equality_expression(
    context: &mut ParserContext,

    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, GQLError> {
    let expression = parse_comparison_expression(context, tokens, position);
    if expression.is_err() || *position >= tokens.len() {
        return expression;
    }

    let lhs = expression.ok().unwrap();

    let operator = &tokens[*position];
    if operator.kind == TokenKind::Equal || operator.kind == TokenKind::BangEqual {
        *position += 1;
        let comparison_operator = if operator.kind == TokenKind::Equal {
            ComparisonOperator::Equal
        } else {
            ComparisonOperator::NotEqual
        };

        let rhs = parse_comparison_expression(context, tokens, position)?;

        // Make sure right and left hand side types are the same
        if rhs.expr_type(&context.symbol_table) != lhs.expr_type(&context.symbol_table) {
            let message = format!(
                "Can't compare values of different types `{}` and `{}`",
                lhs.expr_type(&context.symbol_table).literal(),
                rhs.expr_type(&context.symbol_table).literal()
            );
            return Err(GQLError {
                message,
                location: get_safe_location(tokens, *position - 2),
            });
        }

        return Ok(Box::new(ComparisonExpression {
            left: lhs,
            operator: comparison_operator,
            right: rhs,
        }));
    }

    Ok(lhs)
}

fn parse_comparison_expression(
    context: &mut ParserContext,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, GQLError> {
    let expression = parse_bitwise_shift_expression(context, tokens, position);
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

        let rhs = parse_bitwise_shift_expression(context, tokens, position)?;
        let rhs_type = rhs.expr_type(&context.symbol_table);

        // Make sure right and left hand side types are the same
        if rhs_type == DataType::Integer && rhs_type != lhs.expr_type(&context.symbol_table) {
            let message = format!(
                "Can't compare values of different types `{}` and `{}`",
                lhs.expr_type(&context.symbol_table).literal(),
                rhs_type.literal()
            );
            return Err(GQLError {
                message,
                location: get_safe_location(tokens, *position - 2),
            });
        }

        return Ok(Box::new(ComparisonExpression {
            left: lhs,
            operator: comparison_operator,
            right: rhs,
        }));
    }

    Ok(lhs)
}

fn parse_bitwise_shift_expression(
    context: &mut ParserContext,

    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, GQLError> {
    let mut lhs = parse_term_expression(context, tokens, position)?;

    while *position < tokens.len() && is_bitwise_shift_operator(&tokens[*position]) {
        let operator = &tokens[*position];
        *position += 1;
        let bitwise_operator = if operator.kind == TokenKind::BitwiseRightShift {
            BitwiseOperator::RightShift
        } else {
            BitwiseOperator::LeftShift
        };

        let rhs = parse_term_expression(context, tokens, position)?;

        // Make sure right and left hand side types are numbers
        if rhs.expr_type(&context.symbol_table) == DataType::Integer
            && rhs.expr_type(&context.symbol_table) != lhs.expr_type(&context.symbol_table)
        {
            let message = format!(
                "Bitwise operators require number types but got `{}` and `{}`",
                lhs.expr_type(&context.symbol_table).literal(),
                rhs.expr_type(&context.symbol_table).literal()
            );
            return Err(GQLError {
                message,
                location: get_safe_location(tokens, *position - 2),
            });
        }

        lhs = Box::new(BitwiseExpression {
            left: lhs,
            operator: bitwise_operator,
            right: rhs,
        });
    }

    Ok(lhs)
}

fn parse_term_expression(
    context: &mut ParserContext,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, GQLError> {
    let mut lhs = parse_factor_expression(context, tokens, position)?;

    while *position < tokens.len() && is_term_operator(&tokens[*position]) {
        let operator = &tokens[*position];
        *position += 1;
        let math_operator = if operator.kind == TokenKind::Plus {
            ArithmeticOperator::Plus
        } else {
            ArithmeticOperator::Minus
        };

        let rhs = parse_factor_expression(context, tokens, position)?;

        let lhs_type = lhs.expr_type(&context.symbol_table);
        let rhs_type = rhs.expr_type(&context.symbol_table);

        // Make sure right and left hand side types are numbers
        if lhs_type.is_number() && rhs_type.is_number() {
            lhs = Box::new(ArithmeticExpression {
                left: lhs,
                operator: math_operator,
                right: rhs,
            });

            continue;
        }

        let message = format!(
            "Math operators require number types but got `{}` and `{}`",
            lhs_type.literal(),
            rhs_type.literal()
        );

        return Err(GQLError {
            message,
            location: get_safe_location(tokens, *position - 2),
        });
    }

    Ok(lhs)
}

fn parse_factor_expression(
    context: &mut ParserContext,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, GQLError> {
    let expression = parse_like_expression(context, tokens, position);
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

        let rhs = parse_like_expression(context, tokens, position)?;

        let lhs_type = lhs.expr_type(&context.symbol_table);
        let rhs_type = rhs.expr_type(&context.symbol_table);

        // Make sure right and left hand side types are numbers
        if lhs_type.is_number() && rhs_type.is_number() {
            lhs = Box::new(ArithmeticExpression {
                left: lhs,
                operator: factor_operator,
                right: rhs,
            });
            continue;
        }

        let message = format!(
            "Math operators require number types but got `{}` and `{}`",
            lhs_type.literal(),
            rhs_type.literal()
        );

        return Err(GQLError {
            message,
            location: get_safe_location(tokens, *position - 2),
        });
    }

    Ok(lhs)
}

fn parse_like_expression(
    context: &mut ParserContext,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, GQLError> {
    let expression = parse_glob_expression(context, tokens, position);
    if expression.is_err() || *position >= tokens.len() {
        return expression;
    }

    let lhs = expression.ok().unwrap();
    if tokens[*position].kind == TokenKind::Like {
        let location = tokens[*position].location;
        *position += 1;

        if !lhs.expr_type(&context.symbol_table).is_text() {
            let message = format!(
                "Expect `LIKE` left hand side to be `TEXT` but got {}",
                lhs.expr_type(&context.symbol_table).literal()
            );
            return Err(GQLError { message, location });
        }

        let pattern = parse_glob_expression(context, tokens, position)?;
        if !pattern.expr_type(&context.symbol_table).is_text() {
            let message = format!(
                "Expect `LIKE` right hand side to be `TEXT` but got {}",
                pattern.expr_type(&context.symbol_table).literal()
            );
            return Err(GQLError { message, location });
        }

        return Ok(Box::new(LikeExpression {
            input: lhs,
            pattern,
        }));
    }

    Ok(lhs)
}

fn parse_glob_expression(
    context: &mut ParserContext,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, GQLError> {
    let expression = parse_unary_expression(context, tokens, position);
    if expression.is_err() || *position >= tokens.len() {
        return expression;
    }

    let lhs = expression.ok().unwrap();
    if tokens[*position].kind == TokenKind::Glob {
        let location = tokens[*position].location;
        *position += 1;

        if !lhs.expr_type(&context.symbol_table).is_text() {
            let message = format!(
                "Expect `GLOB` left hand side to be `TEXT` but got {}",
                lhs.expr_type(&context.symbol_table).literal()
            );
            return Err(GQLError { message, location });
        }

        let pattern = parse_unary_expression(context, tokens, position)?;
        if !pattern.expr_type(&context.symbol_table).is_text() {
            let message = format!(
                "Expect `GLOB` right hand side to be `TEXT` but got {}",
                pattern.expr_type(&context.symbol_table).literal()
            );
            return Err(GQLError { message, location });
        }

        return Ok(Box::new(GlobExpression {
            input: lhs,
            pattern,
        }));
    }

    Ok(lhs)
}

fn parse_unary_expression(
    context: &mut ParserContext,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, GQLError> {
    if *position < tokens.len() && is_prefix_unary_operator(&tokens[*position]) {
        let op = if tokens[*position].kind == TokenKind::Bang {
            PrefixUnaryOperator::Bang
        } else {
            PrefixUnaryOperator::Minus
        };

        *position += 1;

        let rhs = parse_expression(context, tokens, position)?;
        let rhs_type = rhs.expr_type(&context.symbol_table);
        if op == PrefixUnaryOperator::Bang && rhs_type != DataType::Boolean {
            return Err(type_missmatch_error(
                get_safe_location(tokens, *position - 1),
                DataType::Boolean,
                rhs_type,
            ));
        } else if op == PrefixUnaryOperator::Minus && rhs_type != DataType::Integer {
            return Err(type_missmatch_error(
                get_safe_location(tokens, *position - 1),
                DataType::Integer,
                rhs_type,
            ));
        }

        return Ok(Box::new(PrefixUnary { right: rhs, op }));
    }

    parse_function_call_expression(context, tokens, position)
}

fn parse_function_call_expression(
    context: &mut ParserContext,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, GQLError> {
    let expression = parse_primary_expression(context, tokens, position)?;
    if *position < tokens.len() && tokens[*position].kind == TokenKind::LeftParen {
        let symbol_expression = expression.as_any().downcast_ref::<SymbolExpression>();
        let function_name_location = get_safe_location(tokens, *position);

        // Make sure function name is SymbolExpression
        if symbol_expression.is_none() {
            return Err(GQLError {
                message: "Function name must be identifier".to_owned(),
                location: function_name_location,
            });
        }

        // Make sure it's valid function name
        let function_name = &symbol_expression.unwrap().value;
        if FUNCTIONS.contains_key(function_name.as_str()) {
            let arguments = parse_arguments_expressions(context, tokens, position)?;
            let prototype = PROTOTYPES.get(function_name.as_str()).unwrap();
            let parameters = &prototype.parameters;
            let return_type = prototype.result.clone();

            check_function_call_arguments(
                context,
                &arguments,
                parameters,
                function_name.to_string(),
                function_name_location,
            )?;

            // Register function name with return type
            context
                .symbol_table
                .define(function_name.to_string(), return_type);

            return Ok(Box::new(CallExpression {
                function_name: function_name.to_string(),
                arguments,
                is_aggregation: false,
            }));
        } else if AGGREGATIONS.contains_key(function_name.as_str()) {
            let arguments = parse_arguments_expressions(context, tokens, position)?;
            let prototype = AGGREGATIONS_PROTOS.get(function_name.as_str()).unwrap();
            let parameters = &vec![prototype.parameter.clone()];
            let return_type = prototype.result.clone();

            check_function_call_arguments(
                context,
                &arguments,
                parameters,
                function_name.to_string(),
                function_name_location,
            )?;

            let argument_str = get_expression_name(&arguments[0]);
            let argument = argument_str.ok().unwrap();
            let column_name = context.generate_column_name();

            context.hidden_selections.push(column_name.to_string());

            // Register aggregation generated name with return type
            context
                .symbol_table
                .define(column_name.to_string(), return_type);

            context.aggregations.insert(
                column_name.clone(),
                AggregateFunction {
                    function_name: function_name.to_string(),
                    argument,
                },
            );

            return Ok(Box::new(SymbolExpression { value: column_name }));
        } else {
            return Err(GQLError {
                message: "No such function name".to_owned(),
                location: function_name_location,
            });
        }
    }
    Ok(expression)
}

fn parse_arguments_expressions(
    context: &mut ParserContext,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Vec<Box<dyn Expression>>, GQLError> {
    let mut arguments: Vec<Box<dyn Expression>> = vec![];
    if consume_kind(tokens, *position, TokenKind::LeftParen).is_ok() {
        *position += 1;

        while tokens[*position].kind != TokenKind::RightParen {
            let argument = parse_expression(context, tokens, position)?;
            let argument_literal = get_expression_name(&argument);
            if argument_literal.is_ok() {
                let literal = argument_literal.ok().unwrap();
                context.hidden_selections.push(literal);
            }

            arguments.push(argument);

            if tokens[*position].kind == TokenKind::Comma {
                *position += 1;
            } else {
                break;
            }
        }

        if consume_kind(tokens, *position, TokenKind::RightParen).is_ok() {
            *position += 1;
        } else {
            return Err(GQLError {
                message: "Expect `)` after function call arguments".to_owned(),
                location: get_safe_location(tokens, *position),
            });
        }
    }
    Ok(arguments)
}

fn parse_primary_expression(
    context: &mut ParserContext,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, GQLError> {
    if *position >= tokens.len() {
        return Err(un_expected_expression_error(tokens, position));
    }

    match tokens[*position].kind {
        TokenKind::String => {
            *position += 1;
            Ok(Box::new(StringExpression {
                value: tokens[*position - 1].literal.to_string(),
            }))
        }
        TokenKind::Symbol => {
            *position += 1;
            let value = tokens[*position - 1].literal.to_string();
            if !context.selected_fields.contains(&value) {
                context.hidden_selections.push(value.to_string());
            }
            Ok(Box::new(SymbolExpression { value }))
        }
        TokenKind::Integer => {
            *position += 1;
            let integer = tokens[*position - 1].literal.parse::<i64>().unwrap();
            let value = Value::Integer(integer);
            Ok(Box::new(NumberExpression { value }))
        }
        TokenKind::Float => {
            *position += 1;
            let float = tokens[*position - 1].literal.parse::<f64>().unwrap();
            let value = Value::Float(float);
            Ok(Box::new(NumberExpression { value }))
        }
        TokenKind::True => {
            *position += 1;
            Ok(Box::new(BooleanExpression { is_true: true }))
        }
        TokenKind::False => {
            *position += 1;
            Ok(Box::new(BooleanExpression { is_true: false }))
        }
        TokenKind::Null => {
            *position += 1;
            Ok(Box::new(NullExpression {}))
        }
        TokenKind::LeftParen => parse_group_expression(context, tokens, position),
        TokenKind::Case => parse_case_expression(context, tokens, position),
        _ => Err(un_expected_expression_error(tokens, position)),
    }
}

fn parse_group_expression(
    context: &mut ParserContext,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, GQLError> {
    *position += 1;
    let expression = parse_expression(context, tokens, position)?;
    if tokens[*position].kind != TokenKind::RightParen {
        return Err(GQLError {
            message: "Expect `)` to end group expression".to_owned(),
            location: get_safe_location(tokens, *position),
        });
    }
    *position += 1;
    Ok(expression)
}

fn parse_case_expression(
    context: &mut ParserContext,
    tokens: &Vec<Token>,
    position: &mut usize,
) -> Result<Box<dyn Expression>, GQLError> {
    let mut conditions: Vec<Box<dyn Expression>> = vec![];
    let mut values: Vec<Box<dyn Expression>> = vec![];
    let mut default_value: Option<Box<dyn Expression>> = None;

    // Consume `case` keyword
    let case_location = tokens[*position].location;
    *position += 1;

    let mut has_else_branch = false;

    while *position < tokens.len() && tokens[*position].kind != TokenKind::End {
        // Else branch
        if tokens[*position].kind == TokenKind::Else {
            if has_else_branch {
                return Err(GQLError {
                    message: "This case expression already has else branch".to_owned(),
                    location: get_safe_location(tokens, *position),
                });
            }

            // consume else keyword
            *position += 1;

            let default_value_expr = parse_expression(context, tokens, position)?;
            default_value = Some(default_value_expr);
            has_else_branch = true;
            continue;
        }

        // When
        let when_result = consume_kind(tokens, *position, TokenKind::When);
        if when_result.is_err() {
            return Err(GQLError {
                message: "Expect `when` before case condition".to_owned(),
                location: get_safe_location(tokens, *position),
            });
        }

        // Consume when keyword
        *position += 1;

        let condition = parse_expression(context, tokens, position)?;
        if condition.expr_type(&context.symbol_table) != DataType::Boolean {
            return Err(GQLError {
                message: "Case condition must be a boolean type".to_owned(),
                location: get_safe_location(tokens, *position - 1),
            });
        }

        conditions.push(condition);

        let then_result = consume_kind(tokens, *position, TokenKind::Then);
        if then_result.is_err() {
            return Err(GQLError {
                message: "Expect `then` after case condition".to_owned(),
                location: get_safe_location(tokens, *position),
            });
        }

        // Consume then keyword
        *position += 1;

        let expression = parse_expression(context, tokens, position)?;
        values.push(expression);
    }

    // Make sure case expression has at least else branch
    if conditions.is_empty() && !has_else_branch {
        return Err(GQLError {
            message: "Case expression must has at least else branch".to_owned(),
            location: get_safe_location(tokens, *position),
        });
    }

    // Make sure case expression end with END keyword
    if *position >= tokens.len() || tokens[*position].kind != TokenKind::End {
        return Err(GQLError {
            message: "Expect `end` after case branches".to_owned(),
            location: get_safe_location(tokens, *position),
        });
    }

    // Consume end
    *position += 1;

    // Make sure this case expression has else branch
    if !has_else_branch {
        return Err(GQLError {
            message: "Case expression must has else branch".to_owned(),
            location: get_safe_location(tokens, *position),
        });
    }

    // Assert that all values has the same type
    let values_type: DataType = values[0].expr_type(&context.symbol_table);
    for (i, value) in values.iter().enumerate().skip(1) {
        if values_type != value.expr_type(&context.symbol_table) {
            return Err(GQLError {
                message: format!(
                    "Case value in branch {} has different type than the last branch",
                    i + 1
                ),
                location: case_location,
            });
        }
    }

    Ok(Box::new(CaseExpression {
        conditions,
        values,
        default_value,
        values_type,
    }))
}

fn check_function_call_arguments(
    context: &mut ParserContext,
    arguments: &Vec<Box<dyn Expression>>,
    parameters: &Vec<DataType>,
    function_name: String,
    location: Location,
) -> Result<(), GQLError> {
    let arguments_len = arguments.len();
    let parameters_len = parameters.len();

    // Make sure number of arguments and parameters are the same
    if arguments_len != parameters_len {
        let message = format!(
            "Function `{}` expects `{}` arguments but got `{}`",
            function_name, parameters_len, arguments_len
        );
        return Err(GQLError { message, location });
    }

    // Check each argument vs parameter type
    for index in 0..arguments_len {
        let argument_type = arguments
            .get(index)
            .unwrap()
            .expr_type(&context.symbol_table);

        let parameter_type = parameters.get(index).unwrap();

        if argument_type == DataType::Any || *parameter_type == DataType::Any {
            continue;
        }

        if argument_type != *parameter_type {
            let message = format!(
                "Function `{}` argument number {} with type `{}` don't match expected type `{}`",
                function_name,
                index,
                argument_type.literal(),
                parameter_type.literal()
            );
            return Err(GQLError { message, location });
        }
    }

    Ok(())
}

fn check_all_values_are_same_type(
    context: &ParserContext,
    arguments: &Vec<Box<dyn Expression>>,
) -> Result<DataType, ()> {
    let arguments_count = arguments.len();
    if arguments_count == 0 {
        return Ok(DataType::Any);
    }

    let data_type = arguments[0].expr_type(&context.symbol_table);
    for argument in arguments.iter().take(arguments_count).skip(1) {
        let expr_type = argument.expr_type(&context.symbol_table);
        if data_type != expr_type {
            return Err(());
        }
    }

    Ok(data_type)
}

fn type_check_selected_fields(
    symbol_table: &Scope,
    table_name: &str,
    fields_names: &Vec<String>,
    tokens: &Vec<Token>,
    position: usize,
) -> Result<(), GQLError> {
    for field_name in fields_names {
        if symbol_table.contains(field_name) {
            let field_type = symbol_table.env.get(field_name);
            if field_type.is_none() || *field_type.unwrap() == DataType::Undefined {
                return Err(GQLError {
                    message: format!("No field with name `{}`", field_name),
                    location: get_safe_location(tokens, position),
                });
            }
            continue;
        }

        let message = format!(
            "Table `{}` has no field with name `{}`",
            table_name, field_name
        );

        return Err(GQLError {
            message,
            location: get_safe_location(tokens, position),
        });
    }
    Ok(())
}

fn un_expected_statement_error(tokens: &Vec<Token>, position: &mut usize) -> GQLError {
    let location = get_safe_location(tokens, *position);
    GQLError {
        message: "Unexpected statement".to_owned(),
        location,
    }
}

fn un_expected_expression_error(tokens: &Vec<Token>, position: &usize) -> GQLError {
    let location = get_safe_location(tokens, *position);

    if *position == 0 || *position >= tokens.len() {
        return GQLError {
            message: "Can't complete parsing this expression".to_owned(),
            location,
        };
    }

    let current = &tokens[*position];
    let previous = &tokens[*position - 1];

    // Make sure `ASC` and `DESC` are used in ORDER BY statement
    if current.kind == TokenKind::Ascending || current.kind == TokenKind::Descending {
        return GQLError {
            message: "`ASC` and `DESC` must be used in `ORDER BY` statement".to_owned(),
            location,
        };
    }

    // Similar to SQL just `=` is used for equality comparisons
    if previous.kind == TokenKind::Equal && current.kind == TokenKind::Equal {
        return GQLError {
            message: "Unexpected `==`, Just use `=` to check equality".to_owned(),
            location,
        };
    }

    // `< =` the user may mean to write `<=`
    if previous.kind == TokenKind::Greater && current.kind == TokenKind::Equal {
        return GQLError {
            message: "Unexpected `> =`, do you mean `>=`?".to_owned(),
            location,
        };
    }

    // `> =` the user may mean to write `>=`
    if previous.kind == TokenKind::Less && current.kind == TokenKind::Equal {
        return GQLError {
            message: "Unexpected `< =`, do you mean `<=`?".to_owned(),
            location,
        };
    }

    // `> >` the user may mean to write '>>'
    if previous.kind == TokenKind::Greater && current.kind == TokenKind::Greater {
        return GQLError {
            message: "Unexpected `> >`, do you mean `>>`?".to_owned(),
            location,
        };
    }

    // `< <` the user may mean to write `<<`
    if previous.kind == TokenKind::Less && current.kind == TokenKind::Less {
        return GQLError {
            message: "Unexpected `< <`, do you mean `<<`?".to_owned(),
            location,
        };
    }

    // `< >` the user may mean to write `<>`
    if previous.kind == TokenKind::Less && current.kind == TokenKind::Greater {
        return GQLError {
            message: "Unexpected `< >`, do you mean `<>`?".to_owned(),
            location,
        };
    }

    // Default error message
    GQLError {
        message: "Can't complete parsing this expression".to_owned(),
        location,
    }
}

#[allow(clippy::borrowed_box)]
fn get_expression_name(expression: &Box<dyn Expression>) -> Result<String, ()> {
    let expr = expression.as_any().downcast_ref::<SymbolExpression>();
    if let Some(symbol) = expr {
        Ok(symbol.value.to_string())
    } else {
        Err(())
    }
}

#[inline(always)]
fn register_current_table_fields_types(table_name: &str, symbol_table: &mut Scope) {
    let table_fields_names = &TABLES_FIELDS_NAMES[table_name];
    for field_name in table_fields_names {
        let field_type = TABLES_FIELDS_TYPES[field_name].clone();
        symbol_table.define(field_name.to_string(), field_type);
    }
}

#[inline(always)]
fn select_all_table_fields(
    table_name: &str,
    selected_fields: &mut Vec<String>,
    fields_names: &mut Vec<String>,
    fields_values: &mut Vec<Box<dyn Expression>>,
) {
    if TABLES_FIELDS_NAMES.contains_key(table_name) {
        let table_fields = &TABLES_FIELDS_NAMES[table_name];

        for field in table_fields {
            if !fields_names.contains(&field.to_string()) {
                fields_names.push(field.to_string());
                selected_fields.push(field.to_string());

                let literal_expr = Box::new(SymbolExpression {
                    value: field.to_string(),
                });

                fields_values.push(literal_expr);
            }
        }
    }
}

#[inline(always)]
fn consume_kind(tokens: &Vec<Token>, position: usize, kind: TokenKind) -> Result<&Token, ()> {
    if position < tokens.len() && tokens[position].kind == kind {
        return Ok(&tokens[position]);
    }
    Err(())
}

#[inline(always)]
fn get_safe_location(tokens: &Vec<Token>, position: usize) -> Location {
    if position < tokens.len() {
        return tokens[position].location;
    }
    tokens[tokens.len() - 1].location
}

#[inline(always)]
fn is_term_operator(token: &Token) -> bool {
    token.kind == TokenKind::Plus || token.kind == TokenKind::Minus
}

#[inline(always)]
fn is_bitwise_shift_operator(token: &Token) -> bool {
    token.kind == TokenKind::BitwiseLeftShift || token.kind == TokenKind::BitwiseRightShift
}

#[inline(always)]
fn is_prefix_unary_operator(token: &Token) -> bool {
    token.kind == TokenKind::Bang || token.kind == TokenKind::Minus
}

#[inline(always)]
fn is_comparison_operator(token: &Token) -> bool {
    token.kind == TokenKind::Greater
        || token.kind == TokenKind::GreaterEqual
        || token.kind == TokenKind::Less
        || token.kind == TokenKind::LessEqual
}

#[inline(always)]
fn is_factor_operator(token: &Token) -> bool {
    token.kind == TokenKind::Star
        || token.kind == TokenKind::Slash
        || token.kind == TokenKind::Percentage
}

#[inline(always)]
fn is_asc_or_desc(token: &Token) -> bool {
    token.kind == TokenKind::Ascending || token.kind == TokenKind::Descending
}

#[inline(always)]
fn type_missmatch_error(location: Location, expected: DataType, actual: DataType) -> GQLError {
    let message = format!(
        "Type mismatch expected `{}`, got `{}`",
        expected.literal(),
        actual.literal()
    );
    GQLError { message, location }
}
