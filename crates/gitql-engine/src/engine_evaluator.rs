use gitql_ast::expression::ArithmeticExpression;
use gitql_ast::expression::ArithmeticOperator;
use gitql_ast::expression::BetweenExpression;
use gitql_ast::expression::BitwiseExpression;
use gitql_ast::expression::BitwiseOperator;
use gitql_ast::expression::BooleanExpression;
use gitql_ast::expression::CallExpression;
use gitql_ast::expression::CaseExpression;
use gitql_ast::expression::CheckExpression;
use gitql_ast::expression::CheckOperator;
use gitql_ast::expression::ComparisonExpression;
use gitql_ast::expression::ComparisonOperator;
use gitql_ast::expression::Expression;
use gitql_ast::expression::ExpressionKind::*;
use gitql_ast::expression::LogicalExpression;
use gitql_ast::expression::LogicalOperator;
use gitql_ast::expression::NumberExpression;
use gitql_ast::expression::PrefixUnary;
use gitql_ast::expression::PrefixUnaryOperator;
use gitql_ast::expression::StringExpression;
use gitql_ast::expression::SymbolExpression;
use gitql_ast::object::GQLObject;
use gitql_ast::transformation::TRANSFORMATIONS;
use gitql_ast::types::DataType;
use gitql_ast::value::Value;

use regex::Regex;
use std::string::String;

pub fn evaluate_expression(
    expression: &Box<dyn Expression>,
    object: &GQLObject,
) -> Result<Value, String> {
    match expression.get_expression_kind() {
        String => {
            let expr = expression
                .as_any()
                .downcast_ref::<StringExpression>()
                .unwrap();
            return evaluate_string(expr);
        }
        Symbol => {
            let expr = expression
                .as_any()
                .downcast_ref::<SymbolExpression>()
                .unwrap();
            return evaluate_symbol(expr, object);
        }
        Number => {
            let expr = expression
                .as_any()
                .downcast_ref::<NumberExpression>()
                .unwrap();
            return evaluate_number(expr);
        }
        Boolean => {
            let expr = expression
                .as_any()
                .downcast_ref::<BooleanExpression>()
                .unwrap();
            return evaluate_boolean(expr);
        }
        PrefixUnary => {
            let expr = expression.as_any().downcast_ref::<PrefixUnary>().unwrap();
            return evaluate_prefix_unary(expr, object);
        }
        Arithmetic => {
            let expr = expression
                .as_any()
                .downcast_ref::<ArithmeticExpression>()
                .unwrap();
            return evaluate_arithmetic(expr, object);
        }
        Comparison => {
            let expr = expression
                .as_any()
                .downcast_ref::<ComparisonExpression>()
                .unwrap();
            return evaluate_comparison(expr, object);
        }
        Check => {
            let expr = expression
                .as_any()
                .downcast_ref::<CheckExpression>()
                .unwrap();
            return evaluate_check(expr, object);
        }
        Logical => {
            let expr = expression
                .as_any()
                .downcast_ref::<LogicalExpression>()
                .unwrap();
            return evaluate_logical(expr, object);
        }
        Bitwise => {
            let expr = expression
                .as_any()
                .downcast_ref::<BitwiseExpression>()
                .unwrap();
            return evaluate_bitwise(expr, object);
        }
        Call => {
            let expr = expression
                .as_any()
                .downcast_ref::<CallExpression>()
                .unwrap();
            return evaluate_call(expr, object);
        }
        Between => {
            let expr = expression
                .as_any()
                .downcast_ref::<BetweenExpression>()
                .unwrap();
            return evaluate_between(expr, object);
        }
        Case => {
            let expr = expression
                .as_any()
                .downcast_ref::<CaseExpression>()
                .unwrap();
            return evaluate_case(expr, object);
        }
    };
}

fn evaluate_string(expr: &StringExpression) -> Result<Value, String> {
    return Ok(Value::Text(expr.value.to_owned()));
}

fn evaluate_symbol(expr: &SymbolExpression, object: &GQLObject) -> Result<Value, String> {
    return Ok(object.attributes.get(&expr.value).unwrap().clone());
}

fn evaluate_number(expr: &NumberExpression) -> Result<Value, String> {
    return Ok(Value::Number(expr.value));
}

fn evaluate_boolean(expr: &BooleanExpression) -> Result<Value, String> {
    return Ok(Value::Boolean(expr.is_true));
}

fn evaluate_prefix_unary(expr: &PrefixUnary, object: &GQLObject) -> Result<Value, String> {
    let value_result = evaluate_expression(&expr.right, object);
    if value_result.is_err() {
        return value_result;
    }

    let rhs = value_result.ok().unwrap();
    return if expr.op == PrefixUnaryOperator::Bang {
        Ok(Value::Boolean(!rhs.as_bool()))
    } else {
        Ok(Value::Number(-rhs.as_number()))
    };
}

fn evaluate_arithmetic(expr: &ArithmeticExpression, object: &GQLObject) -> Result<Value, String> {
    let lhs_result = evaluate_expression(&expr.left, object);
    if lhs_result.is_err() {
        return lhs_result;
    }

    let rhs_result = evaluate_expression(&expr.right, object);
    if rhs_result.is_err() {
        return lhs_result;
    }

    let lhs = lhs_result.ok().unwrap().as_number();
    let rhs = rhs_result.ok().unwrap().as_number();

    return match expr.operator {
        ArithmeticOperator::Plus => Ok(Value::Number(lhs + rhs)),
        ArithmeticOperator::Minus => Ok(Value::Number(lhs - rhs)),
        ArithmeticOperator::Star => {
            let mul_result = lhs.overflowing_mul(rhs);
            if mul_result.1 {
                Err(format!(
                    "Attempt to compute `{} * {}`, which would overflow",
                    lhs, rhs
                ))
            } else {
                Ok(Value::Number(mul_result.0))
            }
        }
        ArithmeticOperator::Slash => {
            if rhs == 0 {
                Err(format!("Attempt to divide `{}` by zero", lhs))
            } else {
                Ok(Value::Number(lhs / rhs))
            }
        }
        ArithmeticOperator::Modulus => {
            if rhs == 0 {
                Err(format!(
                    "Attempt to calculate the remainder of `{}` with a divisor of zero",
                    lhs
                ))
            } else {
                Ok(Value::Number(lhs % rhs))
            }
        }
    };
}

fn evaluate_comparison(expr: &ComparisonExpression, object: &GQLObject) -> Result<Value, String> {
    let lhs_result = evaluate_expression(&expr.left, object);
    if lhs_result.is_err() {
        return lhs_result;
    }

    let rhs_result = evaluate_expression(&expr.right, object);
    if rhs_result.is_err() {
        return lhs_result;
    }

    let lhs = lhs_result.ok().unwrap();
    let rhs = rhs_result.ok().unwrap();

    let left_type = expr.left.expr_type();
    let comparison_result = if left_type == DataType::Number {
        let ilhs = lhs.as_number();
        let irhs = rhs.as_number();
        ilhs.cmp(&irhs)
    } else if left_type == DataType::Boolean {
        let ilhs = lhs.as_bool();
        let irhs = rhs.as_bool();
        ilhs.cmp(&irhs)
    } else {
        lhs.as_text().cmp(&rhs.as_text())
    };

    return Ok(Value::Boolean(match expr.operator {
        ComparisonOperator::Greater => comparison_result.is_gt(),
        ComparisonOperator::GreaterEqual => comparison_result.is_ge(),
        ComparisonOperator::Less => comparison_result.is_lt(),
        ComparisonOperator::LessEqual => comparison_result.is_le(),
        ComparisonOperator::Equal => comparison_result.is_eq(),
        ComparisonOperator::NotEqual => !comparison_result.is_eq(),
    }));
}

fn evaluate_check(expr: &CheckExpression, object: &GQLObject) -> Result<Value, String> {
    let lhs_result = evaluate_expression(&expr.left, object);
    if lhs_result.is_err() {
        return lhs_result;
    }

    let rhs_result = evaluate_expression(&expr.right, object);
    if rhs_result.is_err() {
        return rhs_result;
    }

    let lhs = lhs_result.ok().unwrap().as_text();
    let rhs = rhs_result.ok().unwrap().as_text();

    return Ok(match expr.operator {
        CheckOperator::Contains => Value::Boolean(lhs.contains(&rhs)),
        CheckOperator::StartsWith => Value::Boolean(lhs.starts_with(&rhs)),
        CheckOperator::EndsWith => Value::Boolean(lhs.ends_with(&rhs)),
        CheckOperator::Matches => {
            let regex = Regex::new(&rhs);
            if regex.is_err() {
                return Ok(Value::Boolean(false));
            }
            Value::Boolean(regex.unwrap().is_match(&lhs))
        }
    });
}

fn evaluate_logical(expr: &LogicalExpression, object: &GQLObject) -> Result<Value, String> {
    let lhs_result = evaluate_expression(&expr.left, object);
    if lhs_result.is_err() {
        return lhs_result;
    }

    let lhs = lhs_result.ok().unwrap().as_bool();

    if expr.operator == LogicalOperator::And && !lhs {
        return Ok(Value::Boolean(false));
    }

    if expr.operator == LogicalOperator::Or && lhs {
        return Ok(Value::Boolean(true));
    }

    let rhs_result = evaluate_expression(&expr.right, object);
    if rhs_result.is_err() {
        return rhs_result;
    }

    let rhs = rhs_result.ok().unwrap().as_bool();
    return Ok(Value::Boolean(match expr.operator {
        LogicalOperator::And => lhs && rhs,
        LogicalOperator::Or => lhs || rhs,
        LogicalOperator::Xor => lhs ^ rhs,
    }));
}

fn evaluate_bitwise(expr: &BitwiseExpression, object: &GQLObject) -> Result<Value, String> {
    let lhs_result = evaluate_expression(&expr.left, object);
    if lhs_result.is_err() {
        return lhs_result;
    }

    let rhs_result = evaluate_expression(&expr.right, object);
    if rhs_result.is_err() {
        return rhs_result;
    }

    let lhs = lhs_result.ok().unwrap().as_number();
    let rhs = rhs_result.ok().unwrap().as_number();

    return match expr.operator {
        BitwiseOperator::Or => Ok(Value::Number(lhs | rhs)),
        BitwiseOperator::And => Ok(Value::Number(lhs & rhs)),
        BitwiseOperator::RightShift => {
            if rhs >= 64 {
                Err("Attempt to shift right with overflow".to_string())
            } else {
                Ok(Value::Number(lhs >> rhs))
            }
        }
        BitwiseOperator::LeftShift => {
            if rhs >= 64 {
                Err("Attempt to shift left with overflow".to_string())
            } else {
                Ok(Value::Number(lhs << rhs))
            }
        }
    };
}

fn evaluate_call(expr: &CallExpression, object: &GQLObject) -> Result<Value, String> {
    let lhs_result = evaluate_expression(&expr.callee, object);
    if lhs_result.is_err() {
        return lhs_result;
    }
    let lhs = lhs_result.ok().unwrap();
    let transformation = TRANSFORMATIONS.get(expr.function_name.as_str()).unwrap();
    return Ok(transformation(lhs));
}

fn evaluate_between(expr: &BetweenExpression, object: &GQLObject) -> Result<Value, String> {
    let value_result = evaluate_expression(&expr.value, object);
    if value_result.is_err() {
        return value_result;
    }

    let range_start_result = evaluate_expression(&expr.range_start, object);
    if range_start_result.is_err() {
        return range_start_result;
    }

    let range_end_result = evaluate_expression(&expr.range_end, object);
    if range_end_result.is_err() {
        return range_end_result;
    }

    let value = value_result.ok().unwrap().as_number();
    let range_start = range_start_result.ok().unwrap().as_number();
    let range_end = range_end_result.ok().unwrap().as_number();
    return Ok(Value::Boolean(value >= range_start && value <= range_end));
}

fn evaluate_case(expr: &CaseExpression, object: &GQLObject) -> Result<Value, String> {
    let conditions = &expr.conditions;
    let values = &expr.values;

    for i in 0..conditions.len() {
        let condition_result = evaluate_expression(&conditions[i], object);
        if condition_result.is_err() {
            return condition_result;
        }

        let condition = condition_result.ok().unwrap();
        if condition.as_bool() {
            return evaluate_expression(&values[i], object);
        }
    }

    return match &expr.default_value {
        Some(default_value) => evaluate_expression(default_value, object),
        _ => Err("Invalid case statement".to_owned()),
    };
}
