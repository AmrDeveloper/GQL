use gitql_ast::expression::CastExpr;
use gitql_ast::expression::ComparisonExpr;
use gitql_ast::expression::Expr;
use gitql_ast::expression::GroupComparisonExpr;
use gitql_ast::operator::ComparisonOperator;
use gitql_ast::operator::GroupComparisonOperator;
use gitql_core::environment::Environment;

use crate::context::ParserContext;
use crate::diagnostic::Diagnostic;
use crate::parser::consume_token_or_error;
use crate::parser::parse_contains_expression;
use crate::token::Token;
use crate::token::TokenKind;

pub(crate) fn parse_comparison_expression(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Expr>, Box<Diagnostic>> {
    let lhs = parse_contains_expression(context, env, tokens, position)?;

    if is_comparison_operator(tokens, position) {
        let operator = &tokens[*position];

        // Consume `=`, `!=`, `>`, `<`, `>=`, `<=` or `<>` operator
        *position += 1;

        // Parse and Consume optional `ALL | ANY | SOME`
        let potential_group_op_pos = *position;
        let optional_group_op = parse_optional_group_operator(tokens, position);
        let has_group_op = optional_group_op.is_some();

        // Consume `(` after Group operator if exists
        if has_group_op {
            consume_token_or_error(
                tokens,
                position,
                TokenKind::LeftParen,
                &format!(
                    "Expexts `(` after group operator keyword `{}`",
                    tokens[potential_group_op_pos]
                ),
            )?;
        }

        let rhs = parse_contains_expression(context, env, tokens, position)?;

        // Consume `)` after Group operator expression if exists
        if has_group_op {
            consume_token_or_error(
                tokens,
                position,
                TokenKind::RightParen,
                &format!(
                    "Expexts `)` after group operator  `{}` expression",
                    tokens[potential_group_op_pos]
                ),
            )?;
        }

        let lhs_type = lhs.expr_type();
        let rhs_type = rhs.expr_type();

        // Parse and Check sides for `=` operator
        if operator.kind == TokenKind::Equal {
            let expected_rhs_types = if has_group_op {
                lhs_type.can_perform_group_eq_op_with()
            } else {
                lhs_type.can_perform_eq_op_with()
            };

            // Can perform this operator between LHS and RHS
            if expected_rhs_types.contains(&rhs_type) {
                return Ok(create_comparison_expression(
                    lhs,
                    rhs,
                    ComparisonOperator::Equal,
                    optional_group_op,
                ));
            }

            // Check if RHS expr can be implicit casted to Expected LHS type to make this
            // Expression valid
            for expected_type in expected_rhs_types.iter() {
                if !expected_type.has_implicit_cast_from(&rhs) {
                    continue;
                }

                let casting = Box::new(CastExpr {
                    value: rhs,
                    result_type: expected_type.clone(),
                });

                return Ok(create_comparison_expression(
                    lhs,
                    casting,
                    ComparisonOperator::Equal,
                    optional_group_op,
                ));
            }

            // Check if LHS expr can be implicit casted to Expected RHS type to make this
            // Expression valid
            let expected_lhs_types = if has_group_op {
                rhs_type.can_perform_group_eq_op_with()
            } else {
                rhs_type.can_perform_eq_op_with()
            };

            for expected_type in expected_lhs_types.iter() {
                if !expected_type.has_implicit_cast_from(&lhs) {
                    continue;
                }

                let casting = Box::new(CastExpr {
                    value: lhs,
                    result_type: expected_type.clone(),
                });

                return Ok(create_comparison_expression(
                    casting,
                    rhs,
                    ComparisonOperator::Equal,
                    optional_group_op,
                ));
            }

            let mut diagnostic = Diagnostic::error(&format!(
                "Operator `=` can't be performed between types `{}` and `{}`",
                lhs_type, rhs_type
            ))
            .with_location(operator.location);

            if lhs_type.is_null() || rhs_type.is_null() {
                diagnostic = diagnostic
                    .add_note("Operator `=` can't used to check if value is null")
                    .add_help("Please use `IS NULL` expression");
            }

            // Return error if this operator can't be performed even with implicit cast
            return Err(diagnostic.as_boxed());
        }

        // Parse and Check sides for `!=` operator
        if operator.kind == TokenKind::BangEqual {
            let expected_rhs_types = if has_group_op {
                lhs_type.can_perform_group_bang_eq_op_with()
            } else {
                lhs_type.can_perform_bang_eq_op_with()
            };

            // Can perform this operator between LHS and RHS
            if expected_rhs_types.contains(&rhs_type) {
                return Ok(create_comparison_expression(
                    lhs,
                    rhs,
                    ComparisonOperator::NotEqual,
                    optional_group_op,
                ));
            }

            // Check if RHS expr can be implicit casted to Expected LHS type to make this
            // Expression valid
            for expected_type in expected_rhs_types.iter() {
                if !expected_type.has_implicit_cast_from(&rhs) {
                    continue;
                }

                let casting = Box::new(CastExpr {
                    value: rhs,
                    result_type: expected_type.clone(),
                });

                return Ok(create_comparison_expression(
                    lhs,
                    casting,
                    ComparisonOperator::NotEqual,
                    optional_group_op,
                ));
            }

            // Check if LHS expr can be implicit casted to Expected RHS type to make this
            // Expression valid
            let expected_lhs_types = if has_group_op {
                rhs_type.can_perform_group_bang_eq_op_with()
            } else {
                rhs_type.can_perform_bang_eq_op_with()
            };

            for expected_type in expected_lhs_types.iter() {
                if !expected_type.has_implicit_cast_from(&lhs) {
                    continue;
                }

                let casting = Box::new(CastExpr {
                    value: lhs,
                    result_type: expected_type.clone(),
                });

                return Ok(create_comparison_expression(
                    casting,
                    rhs,
                    ComparisonOperator::NotEqual,
                    optional_group_op,
                ));
            }

            let mut diagnostic = Diagnostic::error(&format!(
                "Operator `!=` can't be performed between types `{}` and `{}`",
                lhs_type, rhs_type
            ))
            .with_location(operator.location);

            // Handle special case if one of the type is null
            if lhs_type.is_null() || rhs_type.is_null() {
                diagnostic = diagnostic
                    .add_note("Operator `!=` can't used to check if value is null")
                    .add_help("Please use `IS NOT NULL` expression");
            }

            // Return error if this operator can't be performed even with implicit cast
            return Err(diagnostic.as_boxed());
        }

        // Parse and Check sides for `>` operator
        if operator.kind == TokenKind::Greater {
            let expected_rhs_types = if has_group_op {
                lhs_type.can_perform_group_gt_op_with()
            } else {
                lhs_type.can_perform_gt_op_with()
            };

            // Can perform this operator between LHS and RHS
            if expected_rhs_types.contains(&rhs_type) {
                return Ok(create_comparison_expression(
                    lhs,
                    rhs,
                    ComparisonOperator::Greater,
                    optional_group_op,
                ));
            }

            // Check if RHS expr can be implicit casted to Expected LHS type to make this
            // Expression valid
            for expected_type in expected_rhs_types.iter() {
                if !expected_type.has_implicit_cast_from(&rhs) {
                    continue;
                }

                let casting = Box::new(CastExpr {
                    value: rhs,
                    result_type: expected_type.clone(),
                });

                return Ok(create_comparison_expression(
                    lhs,
                    casting,
                    ComparisonOperator::Greater,
                    optional_group_op,
                ));
            }

            // Check if LHS expr can be implicit casted to Expected RHS type to make this
            // Expression valid
            let expected_lhs_types = if has_group_op {
                rhs_type.can_perform_group_gt_op_with()
            } else {
                rhs_type.can_perform_gt_op_with()
            };

            for expected_type in expected_lhs_types.iter() {
                if !expected_type.has_implicit_cast_from(&lhs) {
                    continue;
                }

                let casting = Box::new(CastExpr {
                    value: lhs,
                    result_type: expected_type.clone(),
                });

                return Ok(create_comparison_expression(
                    casting,
                    rhs,
                    ComparisonOperator::Greater,
                    optional_group_op,
                ));
            }

            // Return error if this operator can't be performed even with implicit cast
            return Err(Diagnostic::error(&format!(
                "Operator `>` can't be performed between types `{}` and `{}`",
                lhs_type, rhs_type
            ))
            .with_location(operator.location)
            .as_boxed());
        }

        // Parse and Check sides for `>=` operator
        if operator.kind == TokenKind::GreaterEqual {
            let expected_rhs_types = if has_group_op {
                lhs_type.can_perform_group_gte_op_with()
            } else {
                lhs_type.can_perform_gte_op_with()
            };

            // Can perform this operator between LHS and RHS
            if expected_rhs_types.contains(&rhs_type) {
                return Ok(create_comparison_expression(
                    lhs,
                    rhs,
                    ComparisonOperator::GreaterEqual,
                    optional_group_op,
                ));
            }

            // Check if RHS expr can be implicit casted to Expected LHS type to make this
            // Expression valid
            for expected_type in expected_rhs_types.iter() {
                if !expected_type.has_implicit_cast_from(&rhs) {
                    continue;
                }

                let casting = Box::new(CastExpr {
                    value: rhs,
                    result_type: expected_type.clone(),
                });

                return Ok(create_comparison_expression(
                    lhs,
                    casting,
                    ComparisonOperator::GreaterEqual,
                    optional_group_op,
                ));
            }

            // Check if LHS expr can be implicit casted to Expected RHS type to make this
            // Expression valid
            let expected_lhs_types = if has_group_op {
                rhs_type.can_perform_group_gte_op_with()
            } else {
                rhs_type.can_perform_gte_op_with()
            };

            for expected_type in expected_lhs_types.iter() {
                if !expected_type.has_implicit_cast_from(&lhs) {
                    continue;
                }

                let casting = Box::new(CastExpr {
                    value: lhs,
                    result_type: expected_type.clone(),
                });

                return Ok(create_comparison_expression(
                    casting,
                    rhs,
                    ComparisonOperator::GreaterEqual,
                    optional_group_op,
                ));
            }

            // Return error if this operator can't be performed even with implicit cast
            return Err(Diagnostic::error(&format!(
                "Operator `>=` can't be performed between types `{}` and `{}`",
                lhs_type, rhs_type
            ))
            .with_location(operator.location)
            .as_boxed());
        }

        // Parse and Check sides for `<` operator
        if operator.kind == TokenKind::Less {
            let expected_rhs_types = if has_group_op {
                lhs_type.can_perform_group_lt_op_with()
            } else {
                lhs_type.can_perform_lt_op_with()
            };

            // Can perform this operator between LHS and RHS
            if expected_rhs_types.contains(&rhs_type) {
                return Ok(create_comparison_expression(
                    lhs,
                    rhs,
                    ComparisonOperator::Less,
                    optional_group_op,
                ));
            }

            // Check if RHS expr can be implicit casted to Expected LHS type to make this
            // Expression valid
            for expected_type in expected_rhs_types.iter() {
                if !expected_type.has_implicit_cast_from(&rhs) {
                    continue;
                }

                let casting = Box::new(CastExpr {
                    value: rhs,
                    result_type: expected_type.clone(),
                });

                return Ok(create_comparison_expression(
                    lhs,
                    casting,
                    ComparisonOperator::Less,
                    optional_group_op,
                ));
            }

            // Check if LHS expr can be implicit casted to Expected RHS type to make this
            // Expression valid
            let expected_lhs_types = if has_group_op {
                rhs_type.can_perform_group_lt_op_with()
            } else {
                rhs_type.can_perform_lt_op_with()
            };

            for expected_type in expected_lhs_types.iter() {
                if !expected_type.has_implicit_cast_from(&lhs) {
                    continue;
                }

                let casting = Box::new(CastExpr {
                    value: lhs,
                    result_type: expected_type.clone(),
                });

                return Ok(create_comparison_expression(
                    casting,
                    rhs,
                    ComparisonOperator::Less,
                    optional_group_op,
                ));
            }

            // Return error if this operator can't be performed even with implicit cast
            return Err(Diagnostic::error(&format!(
                "Operator `<` can't be performed between types `{}` and `{}`",
                lhs_type, rhs_type
            ))
            .with_location(operator.location)
            .as_boxed());
        }

        // Parse and Check sides for `<=` operator
        if operator.kind == TokenKind::LessEqual {
            let expected_rhs_types = if has_group_op {
                lhs_type.can_perform_group_lt_op_with()
            } else {
                lhs_type.can_perform_lt_op_with()
            };

            // Can perform this operator between LHS and RHS
            if expected_rhs_types.contains(&rhs_type) {
                return Ok(create_comparison_expression(
                    lhs,
                    rhs,
                    ComparisonOperator::LessEqual,
                    optional_group_op,
                ));
            }

            // Check if RHS expr can be implicit casted to Expected LHS type to make this
            // Expression valid
            for expected_type in expected_rhs_types.iter() {
                if !expected_type.has_implicit_cast_from(&rhs) {
                    continue;
                }

                let casting = Box::new(CastExpr {
                    value: rhs,
                    result_type: expected_type.clone(),
                });

                return Ok(create_comparison_expression(
                    lhs,
                    casting,
                    ComparisonOperator::LessEqual,
                    optional_group_op,
                ));
            }

            // Check if LHS expr can be implicit casted to Expected RHS type to make this
            // Expression valid
            let expected_lhs_types = if has_group_op {
                rhs_type.can_perform_group_lt_op_with()
            } else {
                rhs_type.can_perform_lt_op_with()
            };

            for expected_type in expected_lhs_types.iter() {
                if !expected_type.has_implicit_cast_from(&lhs) {
                    continue;
                }

                let casting = Box::new(CastExpr {
                    value: lhs,
                    result_type: expected_type.clone(),
                });

                return Ok(create_comparison_expression(
                    casting,
                    rhs,
                    ComparisonOperator::LessEqual,
                    optional_group_op,
                ));
            }

            // Return error if this operator can't be performed even with implicit cast
            return Err(Diagnostic::error(&format!(
                "Operator `<=` can't be performed between types `{}` and `{}`",
                lhs_type, rhs_type
            ))
            .with_location(operator.location)
            .as_boxed());
        }

        // Parse and Check sides for `<=>` operator
        if operator.kind == TokenKind::NullSafeEqual {
            let expected_rhs_types = if has_group_op {
                lhs_type.can_perform_group_null_safe_eq_op_with()
            } else {
                lhs_type.can_perform_null_safe_eq_op_with()
            };

            // Can perform this operator between LHS and RHS
            if expected_rhs_types.contains(&rhs_type) {
                return Ok(create_comparison_expression(
                    lhs,
                    rhs,
                    ComparisonOperator::NullSafeEqual,
                    optional_group_op,
                ));
            }

            // Check if RHS expr can be implicit casted to Expected LHS type to make this
            // Expression valid
            for expected_type in expected_rhs_types.iter() {
                if !expected_type.has_implicit_cast_from(&rhs) {
                    continue;
                }

                let casting = Box::new(CastExpr {
                    value: rhs,
                    result_type: expected_type.clone(),
                });

                return Ok(create_comparison_expression(
                    lhs,
                    casting,
                    ComparisonOperator::NullSafeEqual,
                    optional_group_op,
                ));
            }

            // Check if LHS expr can be implicit casted to Expected RHS type to make this
            // Expression valid
            let expected_lhs_types = if has_group_op {
                rhs_type.can_perform_group_null_safe_eq_op_with()
            } else {
                rhs_type.can_perform_null_safe_eq_op_with()
            };

            for expected_type in expected_lhs_types.iter() {
                if !expected_type.has_implicit_cast_from(&lhs) {
                    continue;
                }

                let casting = Box::new(CastExpr {
                    value: lhs,
                    result_type: expected_type.clone(),
                });

                return Ok(create_comparison_expression(
                    casting,
                    rhs,
                    ComparisonOperator::NullSafeEqual,
                    optional_group_op,
                ));
            }

            // Return error if this operator can't be performed even with implicit cast
            return Err(Diagnostic::error(&format!(
                "Operator `<=>` can't be performed between types `{}` and `{}`",
                lhs_type, rhs_type
            ))
            .with_location(operator.location)
            .as_boxed());
        }
    }

    Ok(lhs)
}

fn parse_optional_group_operator(
    tokens: &[Token],
    position: &mut usize,
) -> Option<GroupComparisonOperator> {
    if *position < tokens.len() && is_group_operator(tokens, position) {
        let group_op = &tokens[*position].kind;

        // Consume Group Operator
        *position += 1;

        return match group_op {
            TokenKind::All => Some(GroupComparisonOperator::All),
            TokenKind::Any => Some(GroupComparisonOperator::Any),
            TokenKind::Some => Some(GroupComparisonOperator::Any),
            _ => unreachable!("Unreacable!"),
        };
    }
    None
}

#[inline(always)]
fn create_comparison_expression(
    left: Box<dyn Expr>,
    right: Box<dyn Expr>,
    comparison_operator: ComparisonOperator,
    optional_group_op: Option<GroupComparisonOperator>,
) -> Box<dyn Expr> {
    if let Some(group_operator) = optional_group_op {
        return Box::new(GroupComparisonExpr {
            left,
            comparison_operator,
            group_operator,
            right,
        });
    }

    Box::new(ComparisonExpr {
        left,
        operator: comparison_operator,
        right,
    })
}

#[inline(always)]
fn is_comparison_operator(tokens: &[Token], position: &usize) -> bool {
    *position < tokens.len()
        && matches!(
            tokens[*position].kind,
            TokenKind::Equal
                | TokenKind::BangEqual
                | TokenKind::Greater
                | TokenKind::GreaterEqual
                | TokenKind::Less
                | TokenKind::LessEqual
                | TokenKind::NullSafeEqual
        )
}

#[inline(always)]
fn is_group_operator(tokens: &[Token], position: &usize) -> bool {
    *position < tokens.len()
        && matches!(
            tokens[*position].kind,
            TokenKind::All | TokenKind::Any | TokenKind::Some
        )
}
