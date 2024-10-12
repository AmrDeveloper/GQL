use gitql_ast::expression::ArithmeticExpression;
use gitql_ast::expression::ArrayExpression;
use gitql_ast::expression::AssignmentExpression;
use gitql_ast::expression::BenchmarkExpression;
use gitql_ast::expression::BetweenExpression;
use gitql_ast::expression::BitwiseExpression;
use gitql_ast::expression::BooleanExpression;
use gitql_ast::expression::CallExpression;
use gitql_ast::expression::CaseExpression;
use gitql_ast::expression::CastExpression;
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
use gitql_ast::expression::Number;
use gitql_ast::expression::NumberExpression;
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
    expression: &Box<dyn Expression>,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
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
        Cast => {
            let expr = expression
                .as_any()
                .downcast_ref::<CastExpression>()
                .unwrap();
            evaluate_cast(env, expr, titles, object)
        }
        Null => Ok(Box::new(NullValue)),
    }
}

fn evaluate_assignment(
    env: &mut Environment,
    expr: &AssignmentExpression,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let value = evaluate_expression(env, &expr.value, titles, object)?;
    env.globals.insert(expr.symbol.to_string(), value.clone());
    Ok(value)
}

fn evaluate_string(expr: &StringExpression) -> Result<Box<dyn Value>, String> {
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
    expr: &SymbolExpression,
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
    expr: &ArrayExpression,
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
    expr: &GlobalVariableExpression,
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

fn evaluate_number(expr: &NumberExpression) -> Result<Box<dyn Value>, String> {
    Ok(match expr.value {
        Number::Int(integer) => Box::new(IntValue { value: integer }),
        Number::Float(float) => Box::new(FloatValue { value: float }),
    })
}

fn evaluate_boolean(expr: &BooleanExpression) -> Result<Box<dyn Value>, String> {
    let value = expr.is_true;
    Ok(Box::new(BoolValue { value }))
}

fn evaluate_collection_index(
    env: &mut Environment,
    expr: &IndexExpression,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let array = evaluate_expression(env, &expr.collection, titles, object)?;
    let index = evaluate_expression(env, &expr.index, titles, object)?;
    array.perform_index_op(&index)
}

fn evaluate_collection_slice(
    env: &mut Environment,
    expr: &SliceExpression,
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
    expr: &UnaryExpression,
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
    expr: &ArithmeticExpression,
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
    expr: &ComparisonExpression,
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
    expr: &ContainsExpression,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let lhs = evaluate_expression(env, &expr.left, titles, object)?;
    let rhs = evaluate_expression(env, &expr.right, titles, object)?;
    lhs.perform_contains_op(&rhs)
}

fn evaluate_like(
    env: &mut Environment,
    expr: &LikeExpression,
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
    expr: &RegexExpression,
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
    expr: &GlobExpression,
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
    expr: &LogicalExpression,
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
    expr: &BitwiseExpression,
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
    expr: &CallExpression,
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
    expr: &BenchmarkExpression,
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
    expr: &BetweenExpression,
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
    expr: &CaseExpression,
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
    expr: &InExpression,
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
    expr: &IsNullExpression,
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
    expr: &CastExpression,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let value = evaluate_expression(env, &expr.value, titles, object)?;
    value.perform_cast_op(&expr.result_type)
}
