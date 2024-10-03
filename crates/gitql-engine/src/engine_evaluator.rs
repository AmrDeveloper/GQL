use gitql_ast::expression::ArithmeticExpression;
use gitql_ast::expression::ArrayExpression;
use gitql_ast::expression::AssignmentExpression;
use gitql_ast::expression::BenchmarkExpression;
use gitql_ast::expression::BetweenExpression;
use gitql_ast::expression::BitwiseExpression;
use gitql_ast::expression::BooleanExpression;
use gitql_ast::expression::CallExpression;
use gitql_ast::expression::CaseExpression;
use gitql_ast::expression::ComparisonExpression;
use gitql_ast::expression::ContainsExpression;
use gitql_ast::expression::Expression;
use gitql_ast::expression::ExpressionKind::*;
use gitql_ast::expression::GlobExpression;
use gitql_ast::expression::GlobalVariableExpression;
use gitql_ast::expression::InExpression;
use gitql_ast::expression::IndexExpression;
use gitql_ast::expression::IsNullExpression;
use gitql_ast::expression::LikeExpression;
use gitql_ast::expression::LogicalExpression;
use gitql_ast::expression::NumberExpression;
use gitql_ast::expression::OverlapExpression;
use gitql_ast::expression::RegexExpression;
use gitql_ast::expression::SliceExpression;
use gitql_ast::expression::StringExpression;
use gitql_ast::expression::StringValueType;
use gitql_ast::expression::SymbolExpression;
use gitql_ast::expression::UnaryExpression;
use gitql_ast::operator::ArithmeticOperator;
use gitql_ast::operator::BinaryBitwiseOperator;
use gitql_ast::operator::BinaryLogicalOperator;
use gitql_ast::operator::ComparisonOperator;
use gitql_ast::operator::ContainsOperator;
use gitql_ast::operator::OverlapOperator;
use gitql_ast::operator::PrefixUnaryOperator;
use gitql_core::environment::Environment;
use gitql_core::types::DataType;
use gitql_core::value::Value;

use regex::Regex;
use std::cmp::Ordering;
use std::ops::Not;
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
        Array => {
            let expr = expression
                .as_any()
                .downcast_ref::<ArrayExpression>()
                .unwrap();
            evaluate_array(env, expr, titles, object)
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
            let expr = expression
                .as_any()
                .downcast_ref::<UnaryExpression>()
                .unwrap();
            evaluate_prefix_unary(env, expr, titles, object)
        }
        Index => {
            let expr = expression
                .as_any()
                .downcast_ref::<IndexExpression>()
                .unwrap();
            evaluate_collection_index(env, expr, titles, object)
        }
        Slice => {
            let expr = expression
                .as_any()
                .downcast_ref::<SliceExpression>()
                .unwrap();
            evaluate_collection_slice(env, expr, titles, object)
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
        Contains => {
            let expr = expression
                .as_any()
                .downcast_ref::<ContainsExpression>()
                .unwrap();
            evaluate_contains(env, expr, titles, object)
        }
        Overlap => {
            let expr = expression
                .as_any()
                .downcast_ref::<OverlapExpression>()
                .unwrap();
            evaluate_overlap(env, expr, titles, object)
        }
        Like => {
            let expr = expression
                .as_any()
                .downcast_ref::<LikeExpression>()
                .unwrap();
            evaluate_like(env, expr, titles, object)
        }
        Regex => {
            let expr = expression
                .as_any()
                .downcast_ref::<RegexExpression>()
                .unwrap();
            evaluate_regex(env, expr, titles, object)
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
        BenchmarkCall => {
            let expr = expression
                .as_any()
                .downcast_ref::<BenchmarkExpression>()
                .unwrap();
            evaluate_benchmark_call(env, expr, titles, object)
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
        StringValueType::Date => Ok(string_literal_to_date(&expr.value)),
        StringValueType::DateTime => Ok(string_literal_to_date_time(&expr.value)),
        StringValueType::Boolean => Ok(string_literal_to_boolean(&expr.value)),
    }
}

fn string_literal_to_date(literal: &str) -> Value {
    let date_time = chrono::NaiveDate::parse_from_str(literal, "%Y-%m-%d").ok();
    let timestamp = if let Some(date) = date_time {
        let zero_time = chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap();
        date.and_time(zero_time).and_utc().timestamp()
    } else {
        0
    };
    Value::Date(timestamp)
}

fn string_literal_to_date_time(literal: &str) -> Value {
    let date_time_format = if literal.contains('.') {
        "%Y-%m-%d %H:%M:%S%.3f"
    } else {
        "%Y-%m-%d %H:%M:%S"
    };

    let date_time = chrono::NaiveDateTime::parse_from_str(literal, date_time_format);
    if date_time.is_err() {
        return Value::DateTime(0);
    }

    Value::DateTime(date_time.ok().unwrap().and_utc().timestamp())
}

fn string_literal_to_boolean(literal: &str) -> Value {
    match literal {
        // True values literal
        "t" => Value::Boolean(true),
        "true" => Value::Boolean(true),
        "y" => Value::Boolean(true),
        "yes" => Value::Boolean(true),
        "1" => Value::Boolean(true),
        // False values literal
        "f" => Value::Boolean(false),
        "false" => Value::Boolean(false),
        "n" => Value::Boolean(false),
        "no" => Value::Boolean(false),
        "0" => Value::Boolean(false),
        // Invalid value, must be unreachable
        _ => Value::Null,
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

fn evaluate_array(
    env: &mut Environment,
    expr: &ArrayExpression,
    titles: &[String],
    object: &Vec<Value>,
) -> Result<Value, String> {
    let data_type = &expr.element_type;
    let mut values: Vec<Value> = Vec::with_capacity(expr.values.len());
    for value in &expr.values {
        values.push(evaluate_expression(env, value, titles, object)?);
    }
    Ok(Value::Array(data_type.clone(), values))
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

fn evaluate_collection_index(
    env: &mut Environment,
    expr: &IndexExpression,
    titles: &[String],
    object: &Vec<Value>,
) -> Result<Value, String> {
    let array = evaluate_expression(env, &expr.collection, titles, object)?;
    let index = evaluate_expression(env, &expr.index, titles, object)?;

    let elements = array.as_array();
    let position = index.as_int() - 1;

    if position < 0 {
        return Err("Array position must be larger than or equal 1".to_string());
    }

    if position as usize >= elements.len() {
        return Err(format!(
            "Array position is larger than array length {} and {}",
            position + 1,
            elements.len()
        ));
    }

    Ok(elements[position as usize].clone())
}

fn evaluate_collection_slice(
    env: &mut Environment,
    expr: &SliceExpression,
    titles: &[String],
    object: &Vec<Value>,
) -> Result<Value, String> {
    let array = evaluate_expression(env, &expr.collection, titles, object)?;
    let elements = array.as_array();
    let len = elements.len() as i64;

    let start = if let Some(start_expr) = &expr.start {
        evaluate_expression(env, start_expr, titles, object)?.as_int()
    } else {
        1
    };

    if start < 1 || start >= len {
        return Err("Slice start must be between 1 and length of collection".to_string());
    }

    let end = if let Some(end_expr) = &expr.end {
        evaluate_expression(env, end_expr, titles, object)?.as_int()
    } else {
        len
    };

    if end < 1 || end > len {
        return Err("Slice end must be between 1 and length of collection".to_string());
    }

    if start > end {
        return Err("Slice end must be larger then start".to_string());
    }

    let usize_start = (start - 1) as usize;
    let usize_end: usize = (end) as usize;
    let slice: Vec<Value> = elements[usize_start..usize_end].to_vec();
    let element_type = match expr.expr_type(env) {
        DataType::Array(element_type) => *element_type,
        _ => DataType::Any,
    };

    Ok(Value::Array(element_type, slice))
}

fn evaluate_prefix_unary(
    env: &mut Environment,
    expr: &UnaryExpression,
    titles: &[String],
    object: &Vec<Value>,
) -> Result<Value, String> {
    let rhs = evaluate_expression(env, &expr.right, titles, object)?;
    match expr.operator {
        PrefixUnaryOperator::Minus => {
            if rhs.data_type().is_int() {
                Ok(Value::Integer(-rhs.as_int()))
            } else {
                Ok(Value::Float(-rhs.as_float()))
            }
        }
        PrefixUnaryOperator::Bang => Ok(Value::Boolean(!rhs.as_bool())),
        PrefixUnaryOperator::Not => Ok(Value::Integer(rhs.as_int().not())),
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
        ArithmeticOperator::Plus => lhs.plus(&rhs),
        ArithmeticOperator::Minus => lhs.minus(&rhs),
        ArithmeticOperator::Star => lhs.mul(&rhs),
        ArithmeticOperator::Slash => lhs.div(&rhs),
        ArithmeticOperator::Modulus => lhs.modulus(&rhs),
        ArithmeticOperator::Exponentiation => lhs.pow(&rhs),
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

fn evaluate_contains(
    env: &mut Environment,
    expr: &ContainsExpression,
    titles: &[String],
    object: &Vec<Value>,
) -> Result<Value, String> {
    let lhs = evaluate_expression(env, &expr.left, titles, object)?;
    let rhs = evaluate_expression(env, &expr.right, titles, object)?;

    match expr.operator {
        ContainsOperator::RangeContainsElement => {
            let collection_range = lhs.as_range();
            let is_in_range = Ordering::is_ge(collection_range.0.compare(&rhs))
                && Ordering::is_le(collection_range.1.compare(&rhs));
            Ok(Value::Boolean(is_in_range))
        }
        ContainsOperator::RangeContainsRange => {
            let lhs_range = lhs.as_range();
            let rhs_range = rhs.as_range();
            let is_in_range = Ordering::is_ge(lhs_range.0.compare(&rhs_range.0))
                && Ordering::is_le(lhs_range.1.compare(&rhs_range.1));
            Ok(Value::Boolean(is_in_range))
        }
    }
}

fn evaluate_overlap(
    env: &mut Environment,
    expr: &OverlapExpression,
    titles: &[String],
    object: &Vec<Value>,
) -> Result<Value, String> {
    let lhs = evaluate_expression(env, &expr.left, titles, object)?;
    let rhs = evaluate_expression(env, &expr.right, titles, object)?;

    match expr.operator {
        OverlapOperator::RangeOverlap => {
            let lhs_range = lhs.as_range();
            let rhs_range = rhs.as_range();
            let max_start = if lhs_range.0.compare(&rhs_range.0).is_le() {
                lhs_range.0
            } else {
                rhs_range.0
            };
            let max_end = if lhs_range.1.compare(&rhs_range.1).is_gt() {
                lhs_range.1
            } else {
                rhs_range.1
            };
            // has_overlap = min(r1.1, r2.1) > max(r1.0, r2.0)
            let is_overlap = max_end.compare(&max_start).is_le();
            return Ok(Value::Boolean(is_overlap));
        }
        OverlapOperator::ArrayOverlap => {
            let lhs_array = lhs.as_array();
            let rhs_array = rhs.as_array();
            for lhs_element in lhs_array {
                for rhs_element in rhs_array.iter() {
                    if lhs_element.equals(rhs_element) {
                        return Ok(Value::Boolean(true));
                    }
                }
            }
        }
    }
    Ok(Value::Boolean(false))
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

fn evaluate_regex(
    env: &mut Environment,
    expr: &RegexExpression,
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
    let input = evaluate_expression(env, &expr.input, titles, object)?
        .as_text()
        .to_lowercase();
    Ok(Value::Boolean(regex.is_match(&input)))
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
    if expr.operator == BinaryLogicalOperator::And && !lhs {
        return Ok(Value::Boolean(false));
    }

    if expr.operator == BinaryLogicalOperator::Or && lhs {
        return Ok(Value::Boolean(true));
    }

    let rhs = evaluate_expression(env, &expr.right, titles, object)?.as_bool();

    Ok(Value::Boolean(match expr.operator {
        BinaryLogicalOperator::And => lhs && rhs,
        BinaryLogicalOperator::Or => lhs || rhs,
        BinaryLogicalOperator::Xor => lhs ^ rhs,
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
        BinaryBitwiseOperator::Or => Ok(Value::Integer(lhs | rhs)),
        BinaryBitwiseOperator::And => Ok(Value::Integer(lhs & rhs)),
        BinaryBitwiseOperator::RightShift => {
            if rhs >= 64 {
                Err("Attempt to shift right with overflow".to_string())
            } else {
                Ok(Value::Integer(lhs >> rhs))
            }
        }
        BinaryBitwiseOperator::LeftShift => {
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
    let mut arguments = Vec::with_capacity(expr.arguments.len());
    for arg in expr.arguments.iter() {
        arguments.push(evaluate_expression(env, arg, titles, object)?);
    }

    let function = env.std_function(function_name).unwrap();
    Ok(function(&arguments))
}

fn evaluate_benchmark_call(
    env: &mut Environment,
    expr: &BenchmarkExpression,
    titles: &[String],
    object: &Vec<Value>,
) -> Result<Value, String> {
    let number_of_execution = evaluate_expression(env, &expr.count, titles, object)?;
    for _ in 0..number_of_execution.as_int() {
        evaluate_expression(env, &expr.expression, titles, object)?;
    }
    Ok(Value::Integer(0))
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
