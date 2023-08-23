use gitql_ast::expression::ArithmeticExpression;
use gitql_ast::expression::ArithmeticOperator;
use gitql_ast::expression::BetweenExpression;
use gitql_ast::expression::BitwiseExpression;
use gitql_ast::expression::BitwiseOperator;
use gitql_ast::expression::BooleanExpression;
use gitql_ast::expression::CallExpression;
use gitql_ast::expression::CheckExpression;
use gitql_ast::expression::CheckOperator;
use gitql_ast::expression::ComparisonExpression;
use gitql_ast::expression::ComparisonOperator;
use gitql_ast::expression::Expression;
use gitql_ast::expression::ExpressionKind::*;
use gitql_ast::expression::LogicalExpression;
use gitql_ast::expression::LogicalOperator;
use gitql_ast::expression::NotExpression;
use gitql_ast::expression::NumberExpression;
use gitql_ast::expression::StringExpression;
use gitql_ast::expression::SymbolExpression;
use gitql_ast::object::GQLObject;
use gitql_ast::transformation::TRANSFORMATIONS;
use gitql_ast::types::DataType;

use regex::Regex;
use std::string::String;

pub fn evaluate_expression(
    expression: &Box<dyn Expression>,
    object: &GQLObject,
) -> Result<String, String> {
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
            let expr = expression.as_any().downcast_ref::<NotExpression>().unwrap();
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
    };
}

fn evaluate_string(expr: &StringExpression) -> Result<String, String> {
    return Ok(expr.value.to_owned());
}

fn evaluate_symbol(expr: &SymbolExpression, object: &GQLObject) -> Result<String, String> {
    return Ok(object.attributes.get(&expr.value).unwrap().to_string());
}

fn evaluate_number(expr: &NumberExpression) -> Result<String, String> {
    return Ok(expr.value.to_string());
}

fn evaluate_boolean(expr: &BooleanExpression) -> Result<String, String> {
    return Ok(if expr.is_true {
        "true".to_owned()
    } else {
        "false".to_owned()
    });
}

fn evaluate_prefix_unary(expr: &NotExpression, object: &GQLObject) -> Result<String, String> {
    let value_result = evaluate_expression(&expr.right, object);
    if value_result.is_err() {
        return value_result;
    }
    return Ok((!value_result.ok().unwrap().eq("true")).to_string());
}

fn evaluate_arithmetic(expr: &ArithmeticExpression, object: &GQLObject) -> Result<String, String> {
    let lhs_result = evaluate_expression(&expr.left, object);
    if lhs_result.is_err() {
        return lhs_result;
    }

    let rhs_result = evaluate_expression(&expr.right, object);
    if rhs_result.is_err() {
        return lhs_result;
    }

    let lhs = lhs_result.ok().unwrap().parse::<i64>().unwrap();
    let rhs = rhs_result.ok().unwrap().parse::<i64>().unwrap();

    return match expr.operator {
        ArithmeticOperator::Plus => Ok((lhs + rhs).to_string()),
        ArithmeticOperator::Minus => Ok((lhs - rhs).to_string()),
        ArithmeticOperator::Star => Ok((lhs * rhs).to_string()),
        ArithmeticOperator::Slash => {
            if rhs == 0 {
                Err(format!("Attempt to divide `{}` by zero", lhs))
            } else {
                Ok((lhs / rhs).to_string())
            }
        }
        ArithmeticOperator::Modulus => {
            if rhs == 0 {
                Err(format!(
                    "Attempt to calculate the remainder of `{}` with a divisor of zero",
                    lhs
                ))
            } else {
                Ok((lhs % rhs).to_string())
            }
        }
    };
}

fn evaluate_comparison(expr: &ComparisonExpression, object: &GQLObject) -> Result<String, String> {
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

    let is_string_comparison = expr.left.expr_type() == DataType::Text;
    let result = if is_string_comparison {
        lhs.cmp(&rhs)
    } else {
        let ilhs = lhs.parse::<i64>().unwrap();
        let irhs = rhs.parse::<i64>().unwrap();
        ilhs.cmp(&irhs)
    };

    return Ok(match expr.operator {
        ComparisonOperator::Greater => result.is_gt(),
        ComparisonOperator::GreaterEqual => result.is_ge(),
        ComparisonOperator::Less => result.is_lt(),
        ComparisonOperator::LessEqual => result.is_le(),
        ComparisonOperator::Equal => result.is_eq(),
        ComparisonOperator::NotEqual => !result.is_eq(),
    }
    .to_string());
}

fn evaluate_check(expr: &CheckExpression, object: &GQLObject) -> Result<String, String> {
    let lhs_result = evaluate_expression(&expr.left, object);
    if lhs_result.is_err() {
        return lhs_result;
    }

    let rhs_result = evaluate_expression(&expr.right, object);
    if rhs_result.is_err() {
        return rhs_result;
    }

    let lhs = lhs_result.ok().unwrap();
    let rhs = rhs_result.ok().unwrap();

    return Ok(match expr.operator {
        CheckOperator::Contains => lhs.contains(&rhs),
        CheckOperator::StartsWith => lhs.starts_with(&rhs),
        CheckOperator::EndsWith => lhs.ends_with(&rhs),
        CheckOperator::Matches => {
            let regex = Regex::new(&rhs);
            if regex.is_err() {
                return Ok("false".to_owned());
            }
            regex.unwrap().is_match(&lhs)
        }
    }
    .to_string());
}

fn evaluate_logical(expr: &LogicalExpression, object: &GQLObject) -> Result<String, String> {
    let lhs_result = evaluate_expression(&expr.left, object);
    if lhs_result.is_err() {
        return lhs_result;
    }

    let lhs = lhs_result.ok().unwrap().eq("true");

    if expr.operator == LogicalOperator::And && !lhs {
        return Ok("false".to_owned());
    }

    if expr.operator == LogicalOperator::Or && lhs {
        return Ok("true".to_owned());
    }

    let rhs_result = evaluate_expression(&expr.right, object);
    if rhs_result.is_err() {
        return rhs_result;
    }

    let rhs = rhs_result.ok().unwrap().eq("true");

    return Ok(match expr.operator {
        LogicalOperator::And => lhs && rhs,
        LogicalOperator::Or => lhs || rhs,
        LogicalOperator::Xor => lhs ^ rhs,
    }
    .to_string());
}

fn evaluate_bitwise(expr: &BitwiseExpression, object: &GQLObject) -> Result<String, String> {
    let lhs_result = evaluate_expression(&expr.left, object);
    if lhs_result.is_err() {
        return lhs_result;
    }

    let rhs_result = evaluate_expression(&expr.right, object);
    if rhs_result.is_err() {
        return rhs_result;
    }

    let lhs = lhs_result.ok().unwrap().parse::<i64>().unwrap();
    let rhs = rhs_result.ok().unwrap().parse::<i64>().unwrap();

    return match expr.operator {
        BitwiseOperator::Or => Ok((lhs | rhs).to_string()),
        BitwiseOperator::And => Ok((lhs & rhs).to_string()),
        BitwiseOperator::RightShift => {
            if rhs >= 64 {
                Err("Attempt to shift right with overflow".to_string())
            } else {
                Ok((lhs >> rhs).to_string())
            }
        }
        BitwiseOperator::LeftShift => {
            if rhs >= 64 {
                Err("Attempt to shift left with overflow".to_string())
            } else {
                Ok((lhs << rhs).to_string())
            }
        }
    };
}

fn evaluate_call(expr: &CallExpression, object: &GQLObject) -> Result<String, String> {
    let lhs_result = evaluate_expression(&expr.callee, object);
    if lhs_result.is_err() {
        return lhs_result;
    }
    let lhs = lhs_result.ok().unwrap();
    let transformation = TRANSFORMATIONS.get(expr.function_name.as_str()).unwrap();
    return Ok(transformation(lhs));
}

fn evaluate_between(expr: &BetweenExpression, object: &GQLObject) -> Result<String, String> {
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

    let value = value_result.ok().unwrap().parse::<i64>().unwrap();
    let range_start = range_start_result.ok().unwrap().parse::<i64>().unwrap();
    let range_end = range_end_result.ok().unwrap().parse::<i64>().unwrap();

    return Ok((value >= range_start && value <= range_end).to_string());
}
