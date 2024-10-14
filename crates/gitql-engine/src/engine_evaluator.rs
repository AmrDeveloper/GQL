use gitql_ast::expression::ArithmeticExpr;
use gitql_ast::expression::ArrayExpr;
use gitql_ast::expression::AssignmentExpr;
use gitql_ast::expression::BenchmarkCallExpr;
use gitql_ast::expression::BetweenExpr;
use gitql_ast::expression::BitwiseExpr;
use gitql_ast::expression::BooleanExpr;
use gitql_ast::expression::CallExpr;
use gitql_ast::expression::CaseExpr;
use gitql_ast::expression::CastExpr;
use gitql_ast::expression::ComparisonExpr;
use gitql_ast::expression::ContainedByExpr;
use gitql_ast::expression::ContainsExpr;
use gitql_ast::expression::Expr;
use gitql_ast::expression::ExprKind::*;
use gitql_ast::expression::GlobExpr;
use gitql_ast::expression::GlobalVariableExpr;
use gitql_ast::expression::InExpr;
use gitql_ast::expression::IndexExpr;
use gitql_ast::expression::IsNullExpr;
use gitql_ast::expression::LikeExpr;
use gitql_ast::expression::LogicalExpr;
use gitql_ast::expression::Number;
use gitql_ast::expression::NumberExpr;
use gitql_ast::expression::RegexExpr;
use gitql_ast::expression::SliceExpr;
use gitql_ast::expression::StringExpr;
use gitql_ast::expression::StringValueType;
use gitql_ast::expression::SymbolExpr;
use gitql_ast::expression::UnaryExpr;
use gitql_ast::operator::ArithmeticOperator;
use gitql_ast::operator::BinaryBitwiseOperator;
use gitql_ast::operator::BinaryLogicalOperator;
use gitql_ast::operator::ComparisonOperator;
use gitql_ast::operator::PrefixUnaryOperator;
use gitql_core::environment::Environment;
use gitql_core::values::array::ArrayValue;
use gitql_core::values::base::Value;
use gitql_core::values::boolean::BoolValue;
use gitql_core::values::date::DateValue;
use gitql_core::values::datetime::DateTimeValue;
use gitql_core::values::float::FloatValue;
use gitql_core::values::integer::IntValue;
use gitql_core::values::null::NullValue;
use gitql_core::values::text::TextValue;
use gitql_core::values::time::TimeValue;

use regex::Regex;
use std::string::String;

#[allow(clippy::borrowed_box)]
pub fn evaluate_expression(
    env: &mut Environment,
    expression: &Box<dyn Expr>,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    match expression.kind() {
        Assignment => {
            let expr = expression
                .as_any()
                .downcast_ref::<AssignmentExpr>()
                .unwrap();
            evaluate_assignment(env, expr, titles, object)
        }
        String => {
            let expr = expression.as_any().downcast_ref::<StringExpr>().unwrap();
            evaluate_string(expr)
        }
        Symbol => {
            let expr = expression.as_any().downcast_ref::<SymbolExpr>().unwrap();
            evaluate_symbol(expr, titles, object)
        }
        Array => {
            let expr = expression.as_any().downcast_ref::<ArrayExpr>().unwrap();
            evaluate_array(env, expr, titles, object)
        }
        GlobalVariable => {
            let expr = expression
                .as_any()
                .downcast_ref::<GlobalVariableExpr>()
                .unwrap();
            evaluate_global_variable(env, expr)
        }
        Number => {
            let expr = expression.as_any().downcast_ref::<NumberExpr>().unwrap();
            evaluate_number(expr)
        }
        Boolean => {
            let expr = expression.as_any().downcast_ref::<BooleanExpr>().unwrap();
            evaluate_boolean(expr)
        }
        PrefixUnary => {
            let expr = expression.as_any().downcast_ref::<UnaryExpr>().unwrap();
            evaluate_prefix_unary(env, expr, titles, object)
        }
        Index => {
            let expr = expression.as_any().downcast_ref::<IndexExpr>().unwrap();
            evaluate_collection_index(env, expr, titles, object)
        }
        Slice => {
            let expr = expression.as_any().downcast_ref::<SliceExpr>().unwrap();
            evaluate_collection_slice(env, expr, titles, object)
        }
        Arithmetic => {
            let expr = expression
                .as_any()
                .downcast_ref::<ArithmeticExpr>()
                .unwrap();
            evaluate_arithmetic(env, expr, titles, object)
        }
        Comparison => {
            let expr = expression
                .as_any()
                .downcast_ref::<ComparisonExpr>()
                .unwrap();
            evaluate_comparison(env, expr, titles, object)
        }
        Contains => {
            let expr = expression.as_any().downcast_ref::<ContainsExpr>().unwrap();
            evaluate_contains(env, expr, titles, object)
        }
        ContainedBy => {
            let expr = expression
                .as_any()
                .downcast_ref::<ContainedByExpr>()
                .unwrap();
            evaluate_contained_by(env, expr, titles, object)
        }
        Like => {
            let expr = expression.as_any().downcast_ref::<LikeExpr>().unwrap();
            evaluate_like(env, expr, titles, object)
        }
        Regex => {
            let expr = expression.as_any().downcast_ref::<RegexExpr>().unwrap();
            evaluate_regex(env, expr, titles, object)
        }
        Glob => {
            let expr = expression.as_any().downcast_ref::<GlobExpr>().unwrap();
            evaluate_glob(env, expr, titles, object)
        }
        Logical => {
            let expr = expression.as_any().downcast_ref::<LogicalExpr>().unwrap();
            evaluate_logical(env, expr, titles, object)
        }
        Bitwise => {
            let expr = expression.as_any().downcast_ref::<BitwiseExpr>().unwrap();
            evaluate_bitwise(env, expr, titles, object)
        }
        Call => {
            let expr = expression.as_any().downcast_ref::<CallExpr>().unwrap();
            evaluate_call(env, expr, titles, object)
        }
        BenchmarkCall => {
            let expr = expression
                .as_any()
                .downcast_ref::<BenchmarkCallExpr>()
                .unwrap();
            evaluate_benchmark_call(env, expr, titles, object)
        }
        Between => {
            let expr = expression.as_any().downcast_ref::<BetweenExpr>().unwrap();
            evaluate_between(env, expr, titles, object)
        }
        Case => {
            let expr = expression.as_any().downcast_ref::<CaseExpr>().unwrap();
            evaluate_case(env, expr, titles, object)
        }
        In => {
            let expr = expression.as_any().downcast_ref::<InExpr>().unwrap();
            evaluate_in(env, expr, titles, object)
        }
        IsNull => {
            let expr = expression.as_any().downcast_ref::<IsNullExpr>().unwrap();
            evaluate_is_null(env, expr, titles, object)
        }
        Cast => {
            let expr = expression.as_any().downcast_ref::<CastExpr>().unwrap();
            evaluate_cast(env, expr, titles, object)
        }
        Null => Ok(Box::new(NullValue)),
    }
}

fn evaluate_assignment(
    env: &mut Environment,
    expr: &AssignmentExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let value = evaluate_expression(env, &expr.value, titles, object)?;
    env.globals.insert(expr.symbol.to_string(), value.clone());
    Ok(value)
}

fn evaluate_string(expr: &StringExpr) -> Result<Box<dyn Value>, String> {
    match expr.value_type {
        StringValueType::Text => Ok(Box::new(TextValue {
            value: expr.value.to_owned(),
        })),
        StringValueType::Time => Ok(Box::new(TimeValue {
            value: expr.value.to_owned(),
        })),
        StringValueType::Date => Ok(string_literal_to_date(&expr.value)),
        StringValueType::DateTime => Ok(string_literal_to_date_time(&expr.value)),
        StringValueType::Boolean => Ok(string_literal_to_boolean(&expr.value)),
    }
}

fn string_literal_to_date(literal: &str) -> Box<dyn Value> {
    let date_time = chrono::NaiveDate::parse_from_str(literal, "%Y-%m-%d").ok();
    let timestamp = if let Some(date) = date_time {
        let zero_time = chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap();
        date.and_time(zero_time).and_utc().timestamp()
    } else {
        0
    };

    Box::new(DateValue { value: timestamp })
}

fn string_literal_to_date_time(literal: &str) -> Box<dyn Value> {
    let date_time_format = if literal.contains('.') {
        "%Y-%m-%d %H:%M:%S%.3f"
    } else {
        "%Y-%m-%d %H:%M:%S"
    };

    let date_time = chrono::NaiveDateTime::parse_from_str(literal, date_time_format);
    if date_time.is_err() {
        return Box::new(DateTimeValue { value: 0 });
    }

    let timestamp = date_time.ok().unwrap().and_utc().timestamp();
    Box::new(DateTimeValue { value: timestamp })
}

fn string_literal_to_boolean(literal: &str) -> Box<dyn Value> {
    match literal {
        // True values literal
        "t" => Box::new(BoolValue { value: true }),
        "true" => Box::new(BoolValue { value: true }),
        "y" => Box::new(BoolValue { value: true }),
        "yes" => Box::new(BoolValue { value: true }),
        "1" => Box::new(BoolValue { value: true }),
        // False values literal
        "f" => Box::new(BoolValue { value: false }),
        "false" => Box::new(BoolValue { value: false }),
        "n" => Box::new(BoolValue { value: false }),
        "no" => Box::new(BoolValue { value: false }),
        "0" => Box::new(BoolValue { value: false }),
        // Invalid value, must be unreachable
        _ => Box::new(NullValue),
    }
}

fn evaluate_symbol(
    expr: &SymbolExpr,
    titles: &[String],
    object: &[Box<dyn Value>],
) -> Result<Box<dyn Value>, String> {
    for (index, title) in titles.iter().enumerate() {
        if expr.value.eq(title) {
            return Ok(object[index].clone());
        }
    }
    Err(format!("Invalid column name `{}`", &expr.value))
}

fn evaluate_array(
    env: &mut Environment,
    expr: &ArrayExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let mut values: Vec<Box<dyn Value>> = Vec::with_capacity(expr.values.len());
    for value in &expr.values {
        values.push(evaluate_expression(env, value, titles, object)?);
    }

    Ok(Box::new(ArrayValue {
        values,
        base_type: expr.element_type.clone(),
    }))
}

fn evaluate_global_variable(
    env: &mut Environment,
    expr: &GlobalVariableExpr,
) -> Result<Box<dyn Value>, String> {
    let name = &expr.name;
    if env.globals.contains_key(name) {
        return Ok(env.globals[name].clone());
    }

    Err(format!(
        "The value of `{}` may be not exists or calculated yet",
        name
    ))
}

fn evaluate_number(expr: &NumberExpr) -> Result<Box<dyn Value>, String> {
    Ok(match expr.value {
        Number::Int(integer) => Box::new(IntValue { value: integer }),
        Number::Float(float) => Box::new(FloatValue { value: float }),
    })
}

fn evaluate_boolean(expr: &BooleanExpr) -> Result<Box<dyn Value>, String> {
    let value = expr.is_true;
    Ok(Box::new(BoolValue { value }))
}

fn evaluate_collection_index(
    env: &mut Environment,
    expr: &IndexExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let array = evaluate_expression(env, &expr.collection, titles, object)?;
    let index = evaluate_expression(env, &expr.index, titles, object)?;
    array.perform_index_op(&index)
}

fn evaluate_collection_slice(
    env: &mut Environment,
    expr: &SliceExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let array = evaluate_expression(env, &expr.collection, titles, object)?;

    let start = if let Some(start_expr) = &expr.start {
        Some(evaluate_expression(env, start_expr, titles, object)?)
    } else {
        None
    };

    let end = if let Some(end_expr) = &expr.end {
        Some(evaluate_expression(env, end_expr, titles, object)?)
    } else {
        None
    };

    array.perform_slice_op(&start, &end)
}

fn evaluate_prefix_unary(
    env: &mut Environment,
    expr: &UnaryExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let rhs = evaluate_expression(env, &expr.right, titles, object)?;
    match expr.operator {
        PrefixUnaryOperator::Minus => rhs.perform_neg_op(),
        PrefixUnaryOperator::Bang => rhs.perform_bang_op(),
        PrefixUnaryOperator::Not => rhs.perform_not_op(),
    }
}

fn evaluate_arithmetic(
    env: &mut Environment,
    expr: &ArithmeticExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let lhs = evaluate_expression(env, &expr.left, titles, object)?;
    let rhs = evaluate_expression(env, &expr.right, titles, object)?;

    match expr.operator {
        ArithmeticOperator::Plus => lhs.perform_add_op(&rhs),
        ArithmeticOperator::Minus => lhs.perform_sub_op(&rhs),
        ArithmeticOperator::Star => lhs.perform_mul_op(&rhs),
        ArithmeticOperator::Slash => lhs.perform_div_op(&rhs),
        ArithmeticOperator::Modulus => lhs.perform_rem_op(&rhs),
        ArithmeticOperator::Exponentiation => lhs.perform_caret_op(&rhs),
    }
}

fn evaluate_comparison(
    env: &mut Environment,
    expr: &ComparisonExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let lhs = evaluate_expression(env, &expr.left, titles, object)?;
    let rhs = evaluate_expression(env, &expr.right, titles, object)?;

    match expr.operator {
        ComparisonOperator::Greater => lhs.perform_gt_op(&rhs),
        ComparisonOperator::GreaterEqual => lhs.perform_gte_op(&rhs),
        ComparisonOperator::Less => lhs.perform_lt_op(&rhs),
        ComparisonOperator::LessEqual => lhs.perform_lte_op(&rhs),
        ComparisonOperator::Equal => lhs.perform_eq_op(&rhs),
        ComparisonOperator::NotEqual => lhs.perform_bang_eq_op(&rhs),
        ComparisonOperator::NullSafeEqual => lhs.perform_null_safe_eq_op(&rhs),
    }
}

fn evaluate_contains(
    env: &mut Environment,
    expr: &ContainsExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let lhs = evaluate_expression(env, &expr.left, titles, object)?;
    let rhs = evaluate_expression(env, &expr.right, titles, object)?;
    lhs.perform_contains_op(&rhs)
}

fn evaluate_contained_by(
    env: &mut Environment,
    expr: &ContainedByExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let lhs = evaluate_expression(env, &expr.left, titles, object)?;
    let rhs = evaluate_expression(env, &expr.right, titles, object)?;
    lhs.perform_contained_by_op(&rhs)
}

fn evaluate_like(
    env: &mut Environment,
    expr: &LikeExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let pattern = evaluate_expression(env, &expr.pattern, titles, object)?;
    let input = evaluate_expression(env, &expr.input, titles, object)?;
    if let Some(pattern_text) = pattern.as_any().downcast_ref::<TextValue>() {
        if let Some(input_text) = input.as_any().downcast_ref::<TextValue>() {
            let pattern = &format!(
                "^{}$",
                pattern_text
                    .value
                    .to_lowercase()
                    .replace('%', ".*")
                    .replace('_', ".")
            );
            let regex_result = Regex::new(pattern);
            if regex_result.is_err() {
                return Err(regex_result.err().unwrap().to_string());
            }
            let regex = regex_result.ok().unwrap();
            let is_match = regex.is_match(&input_text.value.to_lowercase());
            return Ok(Box::new(BoolValue { value: is_match }));
        }
    }

    Err("Invalid Arguments for LIKE expression".to_string())
}

fn evaluate_regex(
    env: &mut Environment,
    expr: &RegexExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let pattern = evaluate_expression(env, &expr.pattern, titles, object)?;
    let input = evaluate_expression(env, &expr.input, titles, object)?;
    if let Some(pattern_text) = pattern.as_any().downcast_ref::<TextValue>() {
        if let Some(input_text) = input.as_any().downcast_ref::<TextValue>() {
            let pattern = &format!(
                "^{}$",
                pattern_text
                    .value
                    .to_lowercase()
                    .replace('%', ".*")
                    .replace('_', ".")
            );

            let regex_result = Regex::new(pattern);
            if regex_result.is_err() {
                return Err(regex_result.err().unwrap().to_string());
            }
            let regex = regex_result.ok().unwrap();
            let is_match = regex.is_match(&input_text.value.to_lowercase());
            return Ok(Box::new(BoolValue { value: is_match }));
        }
    }

    Err("Invalid Arguments for REGEX expression".to_string())
}

fn evaluate_glob(
    env: &mut Environment,
    expr: &GlobExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let rhs = evaluate_expression(env, &expr.pattern, titles, object)?;
    if let Some(rhs_text) = rhs.as_any().downcast_ref::<TextValue>() {
        let text = rhs_text.literal();
        let pattern = &format!(
            "^{}$",
            text.replace('.', "\\.")
                .replace('*', ".*")
                .replace('?', ".")
        );

        let regex_result = Regex::new(pattern);
        if regex_result.is_err() {
            return Err(regex_result.err().unwrap().to_string());
        }
        let regex = regex_result.ok().unwrap();
        let lhs = evaluate_expression(env, &expr.input, titles, object)?;
        if let Some(lhs_text) = lhs.as_any().downcast_ref::<TextValue>() {
            let is_match = regex.is_match(&lhs_text.value);
            return Ok(Box::new(BoolValue { value: is_match }));
        }
    }

    Err("Invalid Arguments for GLOB expression".to_string())
}

fn evaluate_logical(
    env: &mut Environment,
    expr: &LogicalExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let lhs = evaluate_expression(env, &expr.left, titles, object)?;
    let rhs = evaluate_expression(env, &expr.right, titles, object)?;
    match expr.operator {
        BinaryLogicalOperator::And => lhs.perform_logical_and_op(&rhs),
        BinaryLogicalOperator::Or => lhs.perform_logical_or_op(&rhs),
        BinaryLogicalOperator::Xor => lhs.perform_logical_xor_op(&rhs),
    }
}

fn evaluate_bitwise(
    env: &mut Environment,
    expr: &BitwiseExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let lhs = evaluate_expression(env, &expr.left, titles, object)?;
    let rhs = evaluate_expression(env, &expr.right, titles, object)?;

    match expr.operator {
        BinaryBitwiseOperator::Or => lhs.perform_or_op(&rhs),
        BinaryBitwiseOperator::And => lhs.perform_and_op(&rhs),
        BinaryBitwiseOperator::Xor => lhs.perform_xor_op(&rhs),
        BinaryBitwiseOperator::RightShift => lhs.perform_shr_op(&rhs),
        BinaryBitwiseOperator::LeftShift => lhs.perform_shl_op(&rhs),
    }
}

fn evaluate_call(
    env: &mut Environment,
    expr: &CallExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
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
    expr: &BenchmarkCallExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let number_of_execution = evaluate_expression(env, &expr.count, titles, object)?;
    if let Some(number) = number_of_execution.as_any().downcast_ref::<IntValue>() {
        for _ in 0..number.value {
            evaluate_expression(env, &expr.expression, titles, object)?;
        }
    }

    Ok(Box::new(IntValue { value: 0 }))
}

fn evaluate_between(
    env: &mut Environment,
    expr: &BetweenExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let value = evaluate_expression(env, &expr.value, titles, object)?;
    let range_start = evaluate_expression(env, &expr.range_start, titles, object)?;
    let range_end = evaluate_expression(env, &expr.range_end, titles, object)?;
    let result =
        value.compare(&range_start).unwrap().is_le() && value.compare(&range_end).unwrap().is_ge();
    Ok(Box::new(BoolValue { value: result }))
}

fn evaluate_case(
    env: &mut Environment,
    expr: &CaseExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let conditions = &expr.conditions;
    let values = &expr.values;

    for i in 0..conditions.len() {
        let condition = evaluate_expression(env, &conditions[i], titles, object)?;
        if let Some(bool_value) = condition.as_any().downcast_ref::<BoolValue>() {
            if bool_value.value {
                return evaluate_expression(env, &values[i], titles, object);
            }
        }
    }

    match &expr.default_value {
        Some(default_value) => evaluate_expression(env, default_value, titles, object),
        _ => Err("Invalid case statement".to_owned()),
    }
}

fn evaluate_in(
    env: &mut Environment,
    expr: &InExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let argument = evaluate_expression(env, &expr.argument, titles, object)?;

    for value_expr in &expr.values {
        let value = evaluate_expression(env, value_expr, titles, object)?;
        if argument.equals(&value) {
            return Ok(Box::new(BoolValue {
                value: !expr.has_not_keyword,
            }));
        }
    }

    Ok(Box::new(BoolValue {
        value: expr.has_not_keyword,
    }))
}

fn evaluate_is_null(
    env: &mut Environment,
    expr: &IsNullExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let argument = evaluate_expression(env, &expr.argument, titles, object)?;
    let is_null = argument.as_any().downcast_ref::<NullValue>().is_some();
    Ok(Box::new(BoolValue {
        value: if expr.has_not { !is_null } else { is_null },
    }))
}

fn evaluate_cast(
    env: &mut Environment,
    expr: &CastExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let value = evaluate_expression(env, &expr.value, titles, object)?;
    value.perform_cast_op(&expr.result_type)
}
