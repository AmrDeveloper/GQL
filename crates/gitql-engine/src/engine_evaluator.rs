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

pub fn evaluate_expression(expression: &Box<dyn Expression>, object: &GQLObject) -> String {
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

fn evaluate_string(expr: &StringExpression) -> String {
    return expr.value.to_owned();
}

fn evaluate_symbol(expr: &SymbolExpression, object: &GQLObject) -> String {
    return object.attributes.get(&expr.value).unwrap().to_string();
}

fn evaluate_number(expr: &NumberExpression) -> String {
    return expr.value.to_string();
}

fn evaluate_boolean(expr: &BooleanExpression) -> String {
    return if expr.is_true {
        "true".to_owned()
    } else {
        "false".to_owned()
    };
}

fn evaluate_prefix_unary(expr: &NotExpression, object: &GQLObject) -> String {
    let value = evaluate_expression(&expr.right, object);
    return (!value.eq("true")).to_string();
}

fn evaluate_arithmetic(expr: &ArithmeticExpression, object: &GQLObject) -> String {
    let lhs = evaluate_expression(&expr.left, object)
        .parse::<i64>()
        .unwrap();

    let rhs = evaluate_expression(&expr.right, object)
        .parse::<i64>()
        .unwrap();

    return match expr.operator {
        ArithmeticOperator::Plus => lhs + rhs,
        ArithmeticOperator::Minus => lhs - rhs,
        ArithmeticOperator::Star => lhs * rhs,
        ArithmeticOperator::Slash => lhs / rhs,
        ArithmeticOperator::Modulus => lhs % rhs,
    }
    .to_string();
}

fn evaluate_comparison(expr: &ComparisonExpression, object: &GQLObject) -> String {
    let value = evaluate_expression(&expr.left, object);
    let expected = evaluate_expression(&expr.right, object);

    let is_numbers = expr.left.expr_type() == DataType::Text;
    let result = if is_numbers {
        value.cmp(&expected)
    } else {
        value
            .parse::<i64>()
            .unwrap()
            .cmp(&expected.parse::<i64>().unwrap())
    };

    return match expr.operator {
        ComparisonOperator::Greater => result.is_gt(),
        ComparisonOperator::GreaterEqual => result.is_ge(),
        ComparisonOperator::Less => result.is_lt(),
        ComparisonOperator::LessEqual => result.is_le(),
        ComparisonOperator::Equal => result.is_eq(),
        ComparisonOperator::NotEqual => !result.is_eq(),
    }
    .to_string();
}

fn evaluate_check(expr: &CheckExpression, object: &GQLObject) -> String {
    let value = evaluate_expression(&expr.left, object);
    let expected = evaluate_expression(&expr.right, object);
    return match expr.operator {
        CheckOperator::Contains => value.contains(&expected),
        CheckOperator::StartsWith => value.starts_with(&expected),
        CheckOperator::EndsWith => value.ends_with(&expected),
        CheckOperator::Matches => {
            let regex = Regex::new(&expected);
            if regex.is_err() {
                return "false".to_owned();
            }
            regex.unwrap().is_match(&value)
        }
    }
    .to_string();
}

fn evaluate_logical(expr: &LogicalExpression, object: &GQLObject) -> String {
    let lhs = evaluate_expression(&expr.left, object).eq("true");

    if expr.operator == LogicalOperator::And && !lhs {
        return "false".to_owned();
    }

    if expr.operator == LogicalOperator::Or && lhs {
        return "true".to_owned();
    }

    let rhs = evaluate_expression(&expr.right, object).eq("true");

    return match expr.operator {
        LogicalOperator::And => lhs && rhs,
        LogicalOperator::Or => lhs || rhs,
        LogicalOperator::Xor => lhs ^ rhs,
    }
    .to_string();
}

fn evaluate_bitwise(expr: &BitwiseExpression, object: &GQLObject) -> String {
    let lhs = evaluate_expression(&expr.left, object)
        .parse::<i64>()
        .unwrap();

    let rhs = evaluate_expression(&expr.right, object)
        .parse::<i64>()
        .unwrap();

    return match expr.operator {
        BitwiseOperator::Or => lhs | rhs,
        BitwiseOperator::And => lhs & rhs,
        BitwiseOperator::RightShift => lhs << rhs,
        BitwiseOperator::LeftShift => lhs >> rhs,
    }
    .to_string();
}

fn evaluate_call(expr: &CallExpression, object: &GQLObject) -> String {
    let lhs = evaluate_expression(&expr.callee, object);
    let transformation = TRANSFORMATIONS.get(expr.function_name.as_str()).unwrap();
    return transformation(lhs);
}

fn evaluate_between(expr: &BetweenExpression, object: &GQLObject) -> String {
    let value = evaluate_expression(&expr.value, object)
        .parse::<i64>()
        .unwrap();

    let range_start = evaluate_expression(&expr.range_start, object)
        .parse::<i64>()
        .unwrap();

    let range_end = evaluate_expression(&expr.range_end, object)
        .parse::<i64>()
        .unwrap();

    return (value >= range_start && value <= range_end).to_string();
}
