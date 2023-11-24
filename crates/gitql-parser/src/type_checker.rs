use gitql_ast::{
    date_utils::is_valid_time_format,
    expression::{Expression, ExpressionKind, StringExpression, StringValueType},
    scope::Scope,
};

/// The return result after performing types checking
pub enum TypeCheckResult {
    /// Both right and left hand sides types are equals without implicit casting
    Equals,
    /// Both right and left hand sides types are not equals and can't perform implicit casting
    NotEqualAndCantImplicitCast,
    /// Right hand side type will match the left side after implciti casting
    RightSideCasted(Box<dyn Expression>),
    /// Left hand side type will match the right side after implciti casting
    LeftSideCasted(Box<dyn Expression>),
}

#[allow(clippy::borrowed_box)]
pub fn are_types_equals(
    scope: &Scope,
    lhs: &Box<dyn Expression>,
    rhs: &Box<dyn Expression>,
) -> TypeCheckResult {
    let lhs_type = lhs.expr_type(scope);
    let rhs_type = rhs.expr_type(scope);

    // Both types are already equals wihout need for implicit casting
    if lhs_type == rhs_type {
        return TypeCheckResult::Equals;
    }

    // Cast right hand side type from Text constants to time
    if lhs_type.is_time() && rhs_type.is_text() && rhs.expression_kind() == ExpressionKind::String {
        let expr = rhs.as_any().downcast_ref::<StringExpression>().unwrap();
        let string_literal_value = &expr.value;
        if !is_valid_time_format(string_literal_value) {
            return TypeCheckResult::NotEqualAndCantImplicitCast;
        }

        return TypeCheckResult::RightSideCasted(Box::new(StringExpression {
            value: string_literal_value.to_owned(),
            value_type: StringValueType::Time,
        }));
    }

    // Cast left hand side type from Text constants to time
    if lhs_type.is_text() && rhs_type.is_time() && lhs.expression_kind() == ExpressionKind::String {
        let expr = lhs.as_any().downcast_ref::<StringExpression>().unwrap();
        let string_literal_value = &expr.value;
        if is_valid_time_format(string_literal_value) {
            return TypeCheckResult::NotEqualAndCantImplicitCast;
        }

        return TypeCheckResult::LeftSideCasted(Box::new(StringExpression {
            value: string_literal_value.to_owned(),
            value_type: StringValueType::Time,
        }));
    }

    TypeCheckResult::NotEqualAndCantImplicitCast
}
