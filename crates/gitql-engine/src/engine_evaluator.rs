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
use gitql_ast::value::Value;

use regex::Regex;
use std::string::String;

#[allow(clippy::borrowed_box)]
pub fn evaluate_expression(
    env: &mut Environment,
    expression: &Box<dyn Expression>,
    titles: &[String],
    object: &Vec<Value>,
) -> Result<Value, String> {
    match expression.kind() {
        Assignment => {
            let expr = expression
                .as_any()
                .downcast_ref::<AssignmentExpression>()
                .unwrap();
            evaluate_assignment(env, expr, titles, object)
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
            evaluate_symbol(expr, titles, object)
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
            evaluate_prefix_unary(env, expr, titles, object)
        }
        Arithmetic => {
            let expr = expression
                .as_any()
                .downcast_ref::<ArithmeticExpression>()
                .unwrap();
            evaluate_arithmetic(env, expr, titles, object)
        }
        Comparison => {
            let expr = expression
                .as_any()
                .downcast_ref::<ComparisonExpression>()
                .unwrap();
            evaluate_comparison(env, expr, titles, object)
        }
        Like => {
            let expr = expression
                .as_any()
                .downcast_ref::<LikeExpression>()
                .unwrap();
            evaluate_like(env, expr, titles, object)
        }
        Glob => {
            let expr = expression
                .as_any()
                .downcast_ref::<GlobExpression>()
                .unwrap();
            evaluate_glob(env, expr, titles, object)
        }
        Logical => {
            let expr = expression
                .as_any()
                .downcast_ref::<LogicalExpression>()
                .unwrap();
            evaluate_logical(env, expr, titles, object)
        }
        Bitwise => {
            let expr = expression
                .as_any()
                .downcast_ref::<BitwiseExpression>()
                .unwrap();
            evaluate_bitwise(env, expr, titles, object)
        }
        Call => {
            let expr = expression
                .as_any()
                .downcast_ref::<CallExpression>()
                .unwrap();
            evaluate_call(env, expr, titles, object)
        }
        Between => {
            let expr = expression
                .as_any()
                .downcast_ref::<BetweenExpression>()
                .unwrap();
            evaluate_between(env, expr, titles, object)
        }
        Case => {
            let expr = expression
                .as_any()
                .downcast_ref::<CaseExpression>()
                .unwrap();
            evaluate_case(env, expr, titles, object)
        }
        In => {
            let expr = expression.as_any().downcast_ref::<InExpression>().unwrap();
            evaluate_in(env, expr, titles, object)
        }
        IsNull => {
            let expr = expression
                .as_any()
                .downcast_ref::<IsNullExpression>()
                .unwrap();
            evaluate_is_null(env, expr, titles, object)
        }
        Null => Ok(Value::Null),
    }
}

fn evaluate_assignment(
    env: &mut Environment,
    expr: &AssignmentExpression,
    titles: &[String],
    object: &Vec<Value>,
) -> Result<Value, String> {
    let value = evaluate_expression(env, &expr.value, titles, object)?;
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
    titles: &[String],
    object: &[Value],
) -> Result<Value, String> {
    for (index, title) in titles.iter().enumerate() {
        if expr.value.eq(title) {
            return Ok(object[index].clone());
        }
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
    titles: &[String],
    object: &Vec<Value>,
) -> Result<Value, String> {
    let rhs = evaluate_expression(env, &expr.right, titles, object)?;
    if expr.op == PrefixUnaryOperator::Bang {
        Ok(Value::Boolean(!rhs.as_bool()))
    } else {
        Ok(Value::Integer(-rhs.as_int()))
    }
}

fn evaluate_arithmetic(
    env: &mut Environment,
    expr: &ArithmeticExpression,
    titles: &[String],
    object: &Vec<Value>,
) -> Result<Value, String> {
    let lhs = evaluate_expression(env, &expr.left, titles, object)?;
    let rhs = evaluate_expression(env, &expr.right, titles, object)?;

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
    titles: &[String],
    object: &Vec<Value>,
) -> Result<Value, String> {
    let lhs = evaluate_expression(env, &expr.left, titles, object)?;
    let rhs = evaluate_expression(env, &expr.right, titles, object)?;

    let left_type = lhs.data_type();
    let comparison_result = if left_type.is_int() {
        lhs.as_int().cmp(&rhs.as_int())
    } else if left_type.is_float() {
        lhs.as_float().total_cmp(&rhs.as_float())
    } else if left_type.is_bool() {
        lhs.as_bool().cmp(&rhs.as_bool())
    } else {
        lhs.to_string().cmp(&rhs.to_string())
    };

    if expr.operator == ComparisonOperator::NullSafeEqual {
        return Ok(Value::Integer(
            // Return 1 of both sides are null
            if left_type.is_null() && rhs.data_type().is_null() {
                1
            }
            // Return 0 if one side is null
            else if left_type.is_null() || rhs.data_type().is_null() {
                0
            }
            // Return 1 if both non null sides are equals``
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
    titles: &[String],
    object: &Vec<Value>,
) -> Result<Value, String> {
    let rhs = evaluate_expression(env, &expr.pattern, titles, object)?.as_text();
    let pattern = &format!(
        "^{}$",
        rhs.to_lowercase().replace('%', ".*").replace('_', ".")
    );
    let regex_result = Regex::new(pattern);
    if regex_result.is_err() {
        return Err(regex_result.err().unwrap().to_string());
    }
    let regex = regex_result.ok().unwrap();
    let lhs = evaluate_expression(env, &expr.input, titles, object)?
        .as_text()
        .to_lowercase();
    Ok(Value::Boolean(regex.is_match(&lhs)))
}

fn evaluate_glob(
    env: &mut Environment,
    expr: &GlobExpression,
    titles: &[String],
    object: &Vec<Value>,
) -> Result<Value, String> {
    let rhs = evaluate_expression(env, &expr.pattern, titles, object)?.as_text();
    let pattern = &format!(
        "^{}$",
        rhs.replace('.', "\\.").replace('*', ".*").replace('?', ".")
    );
    let regex_result = Regex::new(pattern);
    if regex_result.is_err() {
        return Err(regex_result.err().unwrap().to_string());
    }
    let regex = regex_result.ok().unwrap();
    let lhs = evaluate_expression(env, &expr.input, titles, object)?.as_text();
    Ok(Value::Boolean(regex.is_match(&lhs)))
}

fn evaluate_logical(
    env: &mut Environment,
    expr: &LogicalExpression,
    titles: &[String],
    object: &Vec<Value>,
) -> Result<Value, String> {
    let lhs = evaluate_expression(env, &expr.left, titles, object)?.as_bool();
    if expr.operator == LogicalOperator::And && !lhs {
        return Ok(Value::Boolean(false));
    }

    if expr.operator == LogicalOperator::Or && lhs {
        return Ok(Value::Boolean(true));
    }

    let rhs = evaluate_expression(env, &expr.right, titles, object)?.as_bool();

    Ok(Value::Boolean(match expr.operator {
        LogicalOperator::And => lhs && rhs,
        LogicalOperator::Or => lhs || rhs,
        LogicalOperator::Xor => lhs ^ rhs,
    }))
}

fn evaluate_bitwise(
    env: &mut Environment,
    expr: &BitwiseExpression,
    titles: &[String],
    object: &Vec<Value>,
) -> Result<Value, String> {
    let lhs = evaluate_expression(env, &expr.left, titles, object)?.as_int();
    let rhs = evaluate_expression(env, &expr.right, titles, object)?.as_int();

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
    titles: &[String],
    object: &Vec<Value>,
) -> Result<Value, String> {
    let function_name = expr.function_name.as_str();
    let function = FUNCTIONS.get(function_name).unwrap();

    let mut arguments = Vec::with_capacity(expr.arguments.len());
    for arg in expr.arguments.iter() {
        arguments.push(evaluate_expression(env, arg, titles, object)?);
    }

    Ok(function(&arguments))
}

fn evaluate_between(
    env: &mut Environment,
    expr: &BetweenExpression,
    titles: &[String],
    object: &Vec<Value>,
) -> Result<Value, String> {
    let value = evaluate_expression(env, &expr.value, titles, object)?;
    let range_start = evaluate_expression(env, &expr.range_start, titles, object)?;
    let range_end = evaluate_expression(env, &expr.range_end, titles, object)?;
    Ok(Value::Boolean(
        value.compare(&range_start).is_le() && value.compare(&range_end).is_ge(),
    ))
}

fn evaluate_case(
    env: &mut Environment,
    expr: &CaseExpression,
    titles: &[String],
    object: &Vec<Value>,
) -> Result<Value, String> {
    let conditions = &expr.conditions;
    let values = &expr.values;

    for i in 0..conditions.len() {
        let condition = evaluate_expression(env, &conditions[i], titles, object)?;
        if condition.as_bool() {
            return evaluate_expression(env, &values[i], titles, object);
        }
    }

    match &expr.default_value {
        Some(default_value) => evaluate_expression(env, default_value, titles, object),
        _ => Err("Invalid case statement".to_owned()),
    }
}

fn evaluate_in(
    env: &mut Environment,
    expr: &InExpression,
    titles: &[String],
    object: &Vec<Value>,
) -> Result<Value, String> {
    let argument = evaluate_expression(env, &expr.argument, titles, object)?;

    for value_expr in &expr.values {
        let value = evaluate_expression(env, value_expr, titles, object)?;
        if argument.equals(&value) {
            return Ok(Value::Boolean(!expr.has_not_keyword));
        }
    }

    Ok(Value::Boolean(expr.has_not_keyword))
}

fn evaluate_is_null(
    env: &mut Environment,
    expr: &IsNullExpression,
    titles: &[String],
    object: &Vec<Value>,
) -> Result<Value, String> {
    let argument = evaluate_expression(env, &expr.argument, titles, object)?;
    let is_null = argument.data_type().is_null();
    Ok(Value::Boolean(if expr.has_not {
        !is_null
    } else {
        is_null
    }))
}
