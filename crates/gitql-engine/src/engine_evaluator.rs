use gitql_ast::date_utils::date_time_to_time_stamp;
use gitql_ast::date_utils::date_to_time_stamp;
use gitql_ast::environment::Environment;
use gitql_ast::expression::ArithmeticExpression;
use gitql_ast::expression::ArithmeticOperator;
use gitql_ast::expression::AssignmentExpression;
use gitql_ast::expression::BetweenExpression;
use gitql_ast::expression::BitwiseExpression;
use gitql_ast::expression::BitwiseOperator;
use gitql_ast::expression::BooleanExpression;
use gitql_ast::expression::CallExpression;
use gitql_ast::expression::CaseExpression;
use gitql_ast::expression::ComparisonExpression;
use gitql_ast::expression::ComparisonOperator;
use gitql_ast::expression::Expression;
use gitql_ast::expression::ExpressionKind::*;
use gitql_ast::expression::GlobExpression;
use gitql_ast::expression::GlobalVariableExpression;
use gitql_ast::expression::InExpression;
use gitql_ast::expression::IsNullExpression;
use gitql_ast::expression::LikeExpression;
use gitql_ast::expression::LogicalExpression;
use gitql_ast::expression::LogicalOperator;
use gitql_ast::expression::NumberExpression;
use gitql_ast::expression::PrefixUnary;
use gitql_ast::expression::PrefixUnaryOperator;
use gitql_ast::expression::StringExpression;
use gitql_ast::expression::StringValueType;
use gitql_ast::expression::SymbolExpression;
use gitql_ast::function::FUNCTIONS;
use gitql_ast::types::DataType;
use gitql_ast::value::Value;

use regex::Regex;
use std::collections::HashMap;
use std::string::String;

#[allow(clippy::borrowed_box)]
pub fn evaluate_expression(
    env: &mut Environment,
    expression: &Box<dyn Expression>,
    object: &HashMap<String, Value>,
) -> Result<Value, String> {
    match expression.kind() {
        Assignment => {
            let expr = expression
                .as_any()
                .downcast_ref::<AssignmentExpression>()
                .unwrap();
            evaluate_assignment(env, expr, object)
        }
        String => {
            let expr = expression
                .as_any()
                .downcast_ref::<StringExpression>()
                .unwrap();
            evaluate_string(expr)
        }
        Symbol => {
            let expr = expression
                .as_any()
                .downcast_ref::<SymbolExpression>()
                .unwrap();
            evaluate_symbol(expr, object)
        }
        GlobalVariable => {
            let expr = expression
                .as_any()
                .downcast_ref::<GlobalVariableExpression>()
                .unwrap();
            evaluate_global_variable(env, expr)
        }
        Number => {
            let expr = expression
                .as_any()
                .downcast_ref::<NumberExpression>()
                .unwrap();
            evaluate_number(expr)
        }
        Boolean => {
            let expr = expression
                .as_any()
                .downcast_ref::<BooleanExpression>()
                .unwrap();
            evaluate_boolean(expr)
        }
        PrefixUnary => {
            let expr = expression.as_any().downcast_ref::<PrefixUnary>().unwrap();
            evaluate_prefix_unary(env, expr, object)
        }
        Arithmetic => {
            let expr = expression
                .as_any()
                .downcast_ref::<ArithmeticExpression>()
                .unwrap();
            evaluate_arithmetic(env, expr, object)
        }
        Comparison => {
            let expr = expression
                .as_any()
                .downcast_ref::<ComparisonExpression>()
                .unwrap();
            evaluate_comparison(env, expr, object)
        }
        Like => {
            let expr = expression
                .as_any()
                .downcast_ref::<LikeExpression>()
                .unwrap();
            evaluate_like(env, expr, object)
        }
        Glob => {
            let expr = expression
                .as_any()
                .downcast_ref::<GlobExpression>()
                .unwrap();
            evaluate_glob(env, expr, object)
        }
        Logical => {
            let expr = expression
                .as_any()
                .downcast_ref::<LogicalExpression>()
                .unwrap();
            evaluate_logical(env, expr, object)
        }
        Bitwise => {
            let expr = expression
                .as_any()
                .downcast_ref::<BitwiseExpression>()
                .unwrap();
            evaluate_bitwise(env, expr, object)
        }
        Call => {
            let expr = expression
                .as_any()
                .downcast_ref::<CallExpression>()
                .unwrap();
            evaluate_call(env, expr, object)
        }
        Between => {
            let expr = expression
                .as_any()
                .downcast_ref::<BetweenExpression>()
                .unwrap();
            evaluate_between(env, expr, object)
        }
        Case => {
            let expr = expression
                .as_any()
                .downcast_ref::<CaseExpression>()
                .unwrap();
            evaluate_case(env, expr, object)
        }
        In => {
            let expr = expression.as_any().downcast_ref::<InExpression>().unwrap();
            evaluate_in(env, expr, object)
        }
        IsNull => {
            let expr = expression
                .as_any()
                .downcast_ref::<IsNullExpression>()
                .unwrap();
            evaluate_is_null(env, expr, object)
        }
        Null => Ok(Value::Null),
    }
}

fn evaluate_assignment(
    env: &mut Environment,
    expr: &AssignmentExpression,
    object: &HashMap<String, Value>,
) -> Result<Value, String> {
    let value = evaluate_expression(env, &expr.value, object)?;
    env.globals.insert(expr.symbol.to_string(), value.clone());
    Ok(value)
}

fn evaluate_string(expr: &StringExpression) -> Result<Value, String> {
    match expr.value_type {
        StringValueType::Text => Ok(Value::Text(expr.value.to_owned())),
        StringValueType::Time => Ok(Value::Time(expr.value.to_owned())),
        StringValueType::Date => {
            let timestamp = date_to_time_stamp(&expr.value);
            Ok(Value::Date(timestamp))
        }
        StringValueType::DateTime => {
            let timestamp = date_time_to_time_stamp(&expr.value);
            Ok(Value::DateTime(timestamp))
        }
    }
}

fn evaluate_symbol(
    expr: &SymbolExpression,
    object: &HashMap<String, Value>,
) -> Result<Value, String> {
    if object.contains_key(&expr.value) {
        return Ok(object.get(&expr.value).unwrap().clone());
    }
    Err(format!("Invalid column name `{}`", &expr.value))
}

fn evaluate_global_variable(
    env: &mut Environment,
    expr: &GlobalVariableExpression,
) -> Result<Value, String> {
    let name = &expr.name;
    if env.globals.contains_key(name) {
        return Ok(env.globals[name].clone());
    }

    Err(format!(
        "The value of `{}` may be not exists or calculated yet",
        name
    ))
}

fn evaluate_number(expr: &NumberExpression) -> Result<Value, String> {
    Ok(expr.value.to_owned())
}

fn evaluate_boolean(expr: &BooleanExpression) -> Result<Value, String> {
    Ok(Value::Boolean(expr.is_true))
}

fn evaluate_prefix_unary(
    env: &mut Environment,
    expr: &PrefixUnary,
    object: &HashMap<String, Value>,
) -> Result<Value, String> {
    let rhs = evaluate_expression(env, &expr.right, object)?;
    if expr.op == PrefixUnaryOperator::Bang {
        Ok(Value::Boolean(!rhs.as_bool()))
    } else {
        Ok(Value::Integer(-rhs.as_int()))
    }
}

fn evaluate_arithmetic(
    env: &mut Environment,
    expr: &ArithmeticExpression,
    object: &HashMap<String, Value>,
) -> Result<Value, String> {
    let lhs = evaluate_expression(env, &expr.left, object)?;
    let rhs = evaluate_expression(env, &expr.right, object)?;

    match expr.operator {
        ArithmeticOperator::Plus => Ok(lhs.plus(&rhs)),
        ArithmeticOperator::Minus => Ok(lhs.minus(&rhs)),
        ArithmeticOperator::Star => lhs.mul(&rhs),
        ArithmeticOperator::Slash => lhs.div(&rhs),
        ArithmeticOperator::Modulus => lhs.modulus(&rhs),
    }
}

fn evaluate_comparison(
    env: &mut Environment,
    expr: &ComparisonExpression,
    object: &HashMap<String, Value>,
) -> Result<Value, String> {
    let lhs = evaluate_expression(env, &expr.left, object)?;
    let rhs = evaluate_expression(env, &expr.right, object)?;

    let left_type = lhs.data_type();
    let comparison_result = if left_type == DataType::Integer {
        let ilhs = lhs.as_int();
        let irhs = rhs.as_int();
        ilhs.cmp(&irhs)
    } else if left_type == DataType::Float {
        let ilhs = lhs.as_float();
        let irhs = rhs.as_float();
        ilhs.total_cmp(&irhs)
    } else if left_type == DataType::Boolean {
        let ilhs = lhs.as_bool();
        let irhs = rhs.as_bool();
        ilhs.cmp(&irhs)
    } else {
        lhs.to_string().cmp(&rhs.to_string())
    };

    if expr.operator == ComparisonOperator::NullSafeEqual {
        return Ok(Value::Integer(
            // Return 1 of both sides are null
            if left_type == DataType::Null && rhs.data_type() == DataType::Null {
                1
            }
            // Return 0 if one side is null
            else if left_type == DataType::Null || rhs.data_type() == DataType::Null {
                0
            }
            // Return 1 if both non null sides are equals
            else if comparison_result.is_eq() {
                1
            }
            // Return 0 if both non null sides are not equals
            else {
                0
            },
        ));
    }

    Ok(Value::Boolean(match expr.operator {
        ComparisonOperator::Greater => comparison_result.is_gt(),
        ComparisonOperator::GreaterEqual => comparison_result.is_ge(),
        ComparisonOperator::Less => comparison_result.is_lt(),
        ComparisonOperator::LessEqual => comparison_result.is_le(),
        ComparisonOperator::Equal => comparison_result.is_eq(),
        ComparisonOperator::NotEqual => !comparison_result.is_eq(),
        ComparisonOperator::NullSafeEqual => false,
    }))
}

fn evaluate_like(
    env: &mut Environment,
    expr: &LikeExpression,
    object: &HashMap<String, Value>,
) -> Result<Value, String> {
    let rhs = evaluate_expression(env, &expr.pattern, object)?.as_text();
    let pattern = &format!(
        "^{}$",
        rhs.to_lowercase().replace('%', ".*").replace('_', ".")
    );
    let regex_result = Regex::new(pattern);
    if regex_result.is_err() {
        return Err(regex_result.err().unwrap().to_string());
    }
    let regex = regex_result.ok().unwrap();
    let lhs = evaluate_expression(env, &expr.input, object)?
        .as_text()
        .to_lowercase();
    Ok(Value::Boolean(regex.is_match(&lhs)))
}

fn evaluate_glob(
    env: &mut Environment,
    expr: &GlobExpression,
    object: &HashMap<String, Value>,
) -> Result<Value, String> {
    let rhs = evaluate_expression(env, &expr.pattern, object)?.as_text();
    let pattern = &format!(
        "^{}$",
        rhs.replace('.', "\\.").replace('*', ".*").replace('?', ".")
    );
    let regex_result = Regex::new(pattern);
    if regex_result.is_err() {
        return Err(regex_result.err().unwrap().to_string());
    }
    let regex = regex_result.ok().unwrap();
    let lhs = evaluate_expression(env, &expr.input, object)?.as_text();
    Ok(Value::Boolean(regex.is_match(&lhs)))
}

fn evaluate_logical(
    env: &mut Environment,
    expr: &LogicalExpression,
    object: &HashMap<String, Value>,
) -> Result<Value, String> {
    let lhs = evaluate_expression(env, &expr.left, object)?.as_bool();
    if expr.operator == LogicalOperator::And && !lhs {
        return Ok(Value::Boolean(false));
    }

    if expr.operator == LogicalOperator::Or && lhs {
        return Ok(Value::Boolean(true));
    }

    let rhs = evaluate_expression(env, &expr.right, object)?.as_bool();

    Ok(Value::Boolean(match expr.operator {
        LogicalOperator::And => lhs && rhs,
        LogicalOperator::Or => lhs || rhs,
        LogicalOperator::Xor => lhs ^ rhs,
    }))
}

fn evaluate_bitwise(
    env: &mut Environment,
    expr: &BitwiseExpression,
    object: &HashMap<String, Value>,
) -> Result<Value, String> {
    let lhs = evaluate_expression(env, &expr.left, object)?.as_int();
    let rhs = evaluate_expression(env, &expr.right, object)?.as_int();

    match expr.operator {
        BitwiseOperator::Or => Ok(Value::Integer(lhs | rhs)),
        BitwiseOperator::And => Ok(Value::Integer(lhs & rhs)),
        BitwiseOperator::RightShift => {
            if rhs >= 64 {
                Err("Attempt to shift right with overflow".to_string())
            } else {
                Ok(Value::Integer(lhs >> rhs))
            }
        }
        BitwiseOperator::LeftShift => {
            if rhs >= 64 {
                Err("Attempt to shift left with overflow".to_string())
            } else {
                Ok(Value::Integer(lhs << rhs))
            }
        }
    }
}

fn evaluate_call(
    env: &mut Environment,
    expr: &CallExpression,
    object: &HashMap<String, Value>,
) -> Result<Value, String> {
    let function_name = expr.function_name.as_str();
    let function = FUNCTIONS.get(function_name).unwrap();

    let mut arguments = vec![];
    for arg in expr.arguments.iter() {
        arguments.push(evaluate_expression(env, arg, object)?);
    }

    Ok(function(&arguments))
}

fn evaluate_between(
    env: &mut Environment,
    expr: &BetweenExpression,
    object: &HashMap<String, Value>,
) -> Result<Value, String> {
    let value_result = evaluate_expression(env, &expr.value, object)?;
    let range_start_result = evaluate_expression(env, &expr.range_start, object)?;
    let range_end_result = evaluate_expression(env, &expr.range_end, object)?;
    let value = value_result.as_int();
    let range_start = range_start_result.as_int();
    let range_end = range_end_result.as_int();
    Ok(Value::Boolean(value >= range_start && value <= range_end))
}

fn evaluate_case(
    env: &mut Environment,
    expr: &CaseExpression,
    object: &HashMap<String, Value>,
) -> Result<Value, String> {
    let conditions = &expr.conditions;
    let values = &expr.values;

    for i in 0..conditions.len() {
        let condition = evaluate_expression(env, &conditions[i], object)?;
        if condition.as_bool() {
            return evaluate_expression(env, &values[i], object);
        }
    }

    match &expr.default_value {
        Some(default_value) => evaluate_expression(env, default_value, object),
        _ => Err("Invalid case statement".to_owned()),
    }
}

fn evaluate_in(
    env: &mut Environment,
    expr: &InExpression,
    object: &HashMap<String, Value>,
) -> Result<Value, String> {
    let argument = evaluate_expression(env, &expr.argument, object)?;
    for value_expr in &expr.values {
        let value = evaluate_expression(env, value_expr, object)?;
        if argument.equals(&value) {
            return Ok(Value::Boolean(true));
        }
    }
    Ok(Value::Boolean(false))
}

fn evaluate_is_null(
    env: &mut Environment,
    expr: &IsNullExpression,
    object: &HashMap<String, Value>,
) -> Result<Value, String> {
    let argument = evaluate_expression(env, &expr.argument, object)?;
    let is_null = argument.data_type().is_null();
    Ok(Value::Boolean(if expr.has_not {
        !is_null
    } else {
        is_null
    }))
}
