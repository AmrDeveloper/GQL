use gitql_ast::expression::Expression;
use gitql_ast::expression::ExpressionKind;
use gitql_ast::expression::StringExpression;
use gitql_ast::expression::StringValueType;
use gitql_ast::operator::PrefixUnaryOperator;
use gitql_core::environment::Environment;
use gitql_core::signature::Signature;
use gitql_core::types::DataType;

use crate::diagnostic::Diagnostic;
use crate::format_checker::is_valid_date_format;
use crate::format_checker::is_valid_datetime_format;
use crate::format_checker::is_valid_time_format;
use crate::tokenizer::Location;

/// The return result after performing types checking with implicit casting option
pub enum TypeCheckResult {
    /// Both right and left hand sides types are equals without implicit casting
    Equals,
    /// Both right and left hand sides types are not equals and can't perform implicit casting
    NotEqualAndCantImplicitCast,
    /// Not Equals and can't perform implicit casting with error message provided
    Error(Box<Diagnostic>),
    /// Right hand side type will match the left side after implicit casting
    RightSideCasted(Box<dyn Expression>),
    /// Left hand side type will match the right side after implicit casting
    LeftSideCasted(Box<dyn Expression>),
}

/// List of valid boolean values
const BOOLEANS_VALUES_LITERAL: [&str; 10] =
    ["t", "true", "y", "yes", "1", "f", "false", "n", "no", "0"];

/// The return result after performing types checking with implicit casting option
pub enum ExprTypeCheckResult {
    /// Both right and left hand sides types are equals without implicit casting
    Equals,
    /// Both right and left hand sides types are not equals and can't perform implicit casting
    NotEqualAndCantImplicitCast,
    /// Not Equals and can't perform implicit casting with error message provided
    Error(Box<Diagnostic>),
    /// Left hand side type will match the right side after implicit casting
    ImplicitCasted(Box<dyn Expression>),
}

/// Check if expression type and data type are equals
/// If not then check if one can be implicit casted to the other
///
/// Supported Implicit casting:
/// - String to Time.
/// - String to Date.
/// - String to DateTime
/// - String to Boolean
///
#[allow(clippy::borrowed_box)]
pub fn is_expression_type_equals(
    scope: &Environment,
    expr: &Box<dyn Expression>,
    data_type: &DataType,
) -> ExprTypeCheckResult {
    let expr_type = expr.expr_type(scope);

    // Both types are already equals without need for implicit casting
    if expr_type == *data_type {
        return ExprTypeCheckResult::Equals;
    }

    // Current implicit casting require expression kind to be string literal
    if expr.kind() != ExpressionKind::String || !expr_type.is_text() {
        return ExprTypeCheckResult::NotEqualAndCantImplicitCast;
    }

    // Implicit Casting expression type from Text literal to time
    if data_type.is_time() || data_type.is_variant_with(&DataType::Time) {
        let literal = expr.as_any().downcast_ref::<StringExpression>().unwrap();
        let string_literal_value = &literal.value;
        if !is_valid_time_format(string_literal_value) {
            return ExprTypeCheckResult::Error(
                Diagnostic::error(&format!(
                    "Can't compare Time and Text `{}` because it can't be implicitly casted to Time",
                    string_literal_value
                )).add_help("A valid Time format must match `HH:MM:SS` or `HH:MM:SS.SSS`")
                .add_help("You can use `MAKETIME(hour, minute, second)` function to create date value")
                .as_boxed(),
            );
        }

        return ExprTypeCheckResult::ImplicitCasted(Box::new(StringExpression {
            value: string_literal_value.to_owned(),
            value_type: StringValueType::Time,
        }));
    }

    // Implicit Casting expression type from Text literal to Date
    if data_type.is_date() || data_type.is_variant_with(&DataType::Date) {
        let literal = expr.as_any().downcast_ref::<StringExpression>().unwrap();
        let string_literal_value = &literal.value;
        if !is_valid_date_format(string_literal_value) {
            return ExprTypeCheckResult::Error(
                Diagnostic::error(&format!(
                    "Can't compare Date and Text `{}` because it can't be implicitly casted to Date",
                    string_literal_value
                )).add_help("A valid Date format must match `YYYY-MM-DD`")
                .add_help("You can use `MAKEDATE(year, dayOfYear)` function to a create date value")
                .as_boxed(),
            );
        }

        return ExprTypeCheckResult::ImplicitCasted(Box::new(StringExpression {
            value: string_literal_value.to_owned(),
            value_type: StringValueType::Date,
        }));
    }

    // Implicit Casting expression type from Text literal to DateTime
    if data_type.is_datetime() || data_type.is_variant_with(&DataType::DateTime) {
        let literal = expr.as_any().downcast_ref::<StringExpression>().unwrap();
        let string_literal_value = &literal.value;
        if !is_valid_datetime_format(string_literal_value) {
            return ExprTypeCheckResult::Error(
                Diagnostic::error(&format!(
                    "Can't compare DateTime and Text `{}` because it can't be implicitly casted to DateTime",
                    string_literal_value
                )).add_help("A valid DateTime format must match one of the values `YYYY-MM-DD HH:MM:SS` or `YYYY-MM-DD HH:MM:SS.SSS`")
                .as_boxed(),
            );
        }

        return ExprTypeCheckResult::ImplicitCasted(Box::new(StringExpression {
            value: string_literal_value.to_owned(),
            value_type: StringValueType::DateTime,
        }));
    }

    // Implicit Casting expression type from Text literal to Boolean
    if data_type.is_bool() || data_type.is_variant_with(&DataType::Boolean) {
        let literal = expr.as_any().downcast_ref::<StringExpression>().unwrap();
        let string_literal_value = &literal.value;
        if !BOOLEANS_VALUES_LITERAL.contains(&string_literal_value.as_str()) {
            return ExprTypeCheckResult::Error(
                Diagnostic::error(&format!(
                    "Can't compare Boolean and Text `{}` because it can't be implicitly casted to Boolean",
                    string_literal_value
                )).add_help("A valid Boolean value must match `t, true, y, yes, 1, f, false, n, no, 0`")
                .as_boxed(),
            );
        }

        return ExprTypeCheckResult::ImplicitCasted(Box::new(StringExpression {
            value: string_literal_value.to_owned(),
            value_type: StringValueType::Boolean,
        }));
    }

    ExprTypeCheckResult::NotEqualAndCantImplicitCast
}

/// Check if two expressions types are equals
/// If not then check if one can be implicit casted to the other
#[allow(clippy::borrowed_box)]
pub fn are_types_equals(
    scope: &Environment,
    lhs: &Box<dyn Expression>,
    rhs: &Box<dyn Expression>,
) -> TypeCheckResult {
    let lhs_type = lhs.expr_type(scope);
    let rhs_type = rhs.expr_type(scope);

    // Both types are already equals without need for implicit casting
    if lhs_type == rhs_type {
        return TypeCheckResult::Equals;
    }

    // Check if can cast right hand side to left hand side type
    match is_expression_type_equals(scope, rhs, &lhs_type) {
        ExprTypeCheckResult::ImplicitCasted(expr) => {
            return TypeCheckResult::RightSideCasted(expr);
        }
        ExprTypeCheckResult::Error(diagnostic) => {
            return TypeCheckResult::Error(diagnostic);
        }
        _ => {}
    }

    // Check if can cast left hand side to right hand side type
    match is_expression_type_equals(scope, lhs, &rhs_type) {
        ExprTypeCheckResult::ImplicitCasted(expr) => {
            return TypeCheckResult::LeftSideCasted(expr);
        }
        ExprTypeCheckResult::Error(diagnostic) => {
            return TypeCheckResult::Error(diagnostic);
        }
        _ => {}
    }

    TypeCheckResult::NotEqualAndCantImplicitCast
}

/// Checks if all values has the same type
/// If they have the same type, return it or return None
pub fn check_all_values_are_same_type(
    env: &mut Environment,
    arguments: &[Box<dyn Expression>],
) -> Option<DataType> {
    let arguments_count = arguments.len();
    if arguments_count == 0 {
        return Some(DataType::Any);
    }

    let data_type = arguments[0].expr_type(env);
    for argument in arguments.iter().take(arguments_count).skip(1) {
        let expr_type = argument.expr_type(env);
        if data_type != expr_type {
            return None;
        }
    }

    Some(data_type)
}

/// Check That function call arguments types are matches the parameter types
/// Return a Diagnostic Error if anything is wrong
pub fn check_function_call_arguments(
    env: &Environment,
    arguments: &mut [Box<dyn Expression>],
    parameters: &[DataType],
    function_name: String,
    location: Location,
) -> Result<(), Box<Diagnostic>> {
    let parameters_count = parameters.len();
    let arguments_count = arguments.len();

    let mut has_varargs_parameter = false;
    let mut optional_parameters_count = 0;
    if parameters_count != 0 {
        let last_parameter = parameters.last().unwrap();
        has_varargs_parameter = last_parameter.is_varargs();

        // Count number of optional parameters
        for parameter_type in parameters.iter().take(parameters_count) {
            if parameter_type.is_optional() {
                optional_parameters_count += 1;
            }
        }
    }

    let mut min_arguments_count = parameters_count - optional_parameters_count;
    if has_varargs_parameter {
        min_arguments_count -= 1;
    }

    if arguments_count < min_arguments_count {
        return Err(Diagnostic::error(&format!(
            "Function `{}` expects at least `{}` arguments but got `{}`",
            function_name, min_arguments_count, arguments_count
        ))
        .with_location(location)
        .as_boxed());
    }

    if !has_varargs_parameter && arguments_count > parameters_count {
        return Err(Diagnostic::error(&format!(
            "Function `{}` expects `{}` arguments but got `{}`",
            function_name, arguments_count, parameters_count
        ))
        .with_location(location)
        .as_boxed());
    }

    // Type check the min required arguments
    for index in 0..min_arguments_count {
        let parameter_type = parameters.get(index).unwrap();
        let argument = arguments.get(index).unwrap();

        // Catch undefined arguments
        if argument.expr_type(env).is_undefined() {
            return Err(Diagnostic::error(&format!(
                "Function `{}` argument number {} has Undefined type",
                function_name, index,
            ))
            .add_help("Make sure you used a correct field name")
            .add_help("Check column names for each table from docs website")
            .with_location(location)
            .as_boxed());
        }

        match is_expression_type_equals(env, argument, parameter_type) {
            ExprTypeCheckResult::ImplicitCasted(new_expr) => {
                arguments[index] = new_expr;
            }
            ExprTypeCheckResult::NotEqualAndCantImplicitCast => {
                let argument_type = argument.expr_type(env);
                return Err(Diagnostic::error(&format!(
                    "Function `{}` argument number {} with type `{}` don't match expected type `{}`",
                    function_name, index, argument_type, parameter_type
                ))
                .with_location(location).as_boxed());
            }
            ExprTypeCheckResult::Error(error) => return Err(error),
            ExprTypeCheckResult::Equals => {}
        }
    }

    // Type check the optional parameters
    let last_optional_param_index = min_arguments_count + optional_parameters_count;
    for index in min_arguments_count..last_optional_param_index {
        if index >= arguments_count {
            return Ok(());
        }

        let parameter_type = parameters.get(index).unwrap();
        let argument = arguments.get(index).unwrap();

        // Catch undefined arguments
        if argument.expr_type(env).is_undefined() {
            return Err(Diagnostic::error(&format!(
                "Function `{}` argument number {} has Undefined type",
                function_name, index,
            ))
            .add_help("Make sure you used a correct field name")
            .add_help("Check column names for each table from docs website")
            .with_location(location)
            .as_boxed());
        }

        match is_expression_type_equals(env, argument, parameter_type) {
            ExprTypeCheckResult::ImplicitCasted(new_expr) => {
                arguments[index] = new_expr;
            }
            ExprTypeCheckResult::NotEqualAndCantImplicitCast => {
                let argument_type = argument.expr_type(env);
                return Err(Diagnostic::error(&format!(
                    "Function `{}` argument number {} with type `{}` don't match expected type `{}`",
                    function_name, index, argument_type, parameter_type
                ))
                .with_location(location).as_boxed());
            }
            ExprTypeCheckResult::Error(error) => return Err(error),
            ExprTypeCheckResult::Equals => {}
        }
    }

    // Type check the variable parameters if exists
    if has_varargs_parameter {
        let varargs_type = parameters.last().unwrap();
        for index in last_optional_param_index..arguments_count {
            let argument = arguments.get(index).unwrap();

            // Catch undefined arguments
            if argument.expr_type(env).is_undefined() {
                return Err(Diagnostic::error(&format!(
                    "Function `{}` argument number {} has Undefined type",
                    function_name, index,
                ))
                .add_help("Make sure you used a correct field name")
                .add_help("Check column names for each table from docs website")
                .with_location(location)
                .as_boxed());
            }

            match is_expression_type_equals(env, argument, varargs_type) {
                ExprTypeCheckResult::ImplicitCasted(new_expr) => {
                    arguments[index] = new_expr;
                }
                ExprTypeCheckResult::NotEqualAndCantImplicitCast => {
                    let argument_type = argument.expr_type(env);
                    return Err(Diagnostic::error(&format!(
                        "Function `{}` argument number {} with type `{}` don't match expected type `{}`",
                        function_name, index, argument_type, varargs_type
                    ))
                    .with_location(location).as_boxed());
                }
                ExprTypeCheckResult::Error(error) => return Err(error),
                ExprTypeCheckResult::Equals => {}
            }
        }
    }

    Ok(())
}

/// Check that all selected fields types are defined correctly
/// Return a Diagnostic Error if anything is wrong
pub fn type_check_selected_fields(
    env: &mut Environment,
    table_name: &str,
    fields_names: &Vec<String>,
    location: Location,
) -> Result<(), Box<Diagnostic>> {
    for field_name in fields_names {
        if let Some(data_type) = env.resolve_type(field_name) {
            if data_type.is_undefined() {
                return Err(
                    Diagnostic::error(&format!("No field with name `{}`", field_name))
                        .with_location(location)
                        .as_boxed(),
                );
            }
            continue;
        }

        return Err(Diagnostic::error(&format!(
            "Table `{}` has no field with name `{}`",
            table_name, field_name
        ))
        .add_help("Check the documentations to see available fields for each tables")
        .with_location(location)
        .as_boxed());
    }
    Ok(())
}

/// Check that all projection columns are valid for this table name
/// Return a Diagnostic Error if anything is wrong
pub fn type_check_projection_symbols(
    env: &mut Environment,
    table_name: &str,
    projection_names: &[String],
    projection_locations: &[Location],
) -> Result<(), Box<Diagnostic>> {
    if table_name.is_empty() && !projection_names.is_empty() {
        return Err(Diagnostic::error(&format!(
            "Unresolved field with name `{}`",
            projection_names[0]
        ))
        .with_location(projection_locations[0])
        .as_boxed());
    }

    if table_name.is_empty() {
        return Ok(());
    }

    let count = projection_names.len();
    let table_fields = &env.schema.tables_fields_names[table_name];
    for i in 0..count {
        if !table_fields.contains(&projection_names[i].as_str()) {
            return Err(Diagnostic::error(&format!(
                "Table {} has no field with name `{}`",
                table_name, projection_names[i]
            ))
            .add_help("Check the documentations to see available fields for each tables")
            .with_location(projection_locations[i])
            .as_boxed());
        }
    }

    Ok(())
}

/// Type check the right hand side of prefix unary expression
/// Return Equals, Error, or new expression after implicit casting
#[allow(clippy::borrowed_box)]
pub fn type_check_prefix_unary(
    env: &Environment,
    right: &Box<dyn Expression>,
    op: &PrefixUnaryOperator,
    location: Location,
) -> ExprTypeCheckResult {
    let right_type = right.expr_type(env);
    let expected_type = prefix_unary_expected_type(op);

    if *op == PrefixUnaryOperator::Bang {
        return is_expression_type_equals(env, right, &expected_type);
    }

    if *op == PrefixUnaryOperator::Minus {
        if !right_type.is_number() {
            return ExprTypeCheckResult::Error(type_mismatch_error(
                location,
                expected_type,
                right_type,
            ));
        }
        return ExprTypeCheckResult::Equals;
    }

    if *op == PrefixUnaryOperator::Not {
        if !right_type.is_int() {
            return ExprTypeCheckResult::Error(type_mismatch_error(
                location,
                expected_type,
                right_type,
            ));
        }
        return ExprTypeCheckResult::Equals;
    }

    ExprTypeCheckResult::Equals
}

/// Return the expected [DataType] depending on the prefix unary operator
#[inline(always)]
pub fn prefix_unary_expected_type(op: &PrefixUnaryOperator) -> DataType {
    match op {
        PrefixUnaryOperator::Minus => DataType::Variant(vec![DataType::Integer, DataType::Float]),
        PrefixUnaryOperator::Bang => DataType::Boolean,
        PrefixUnaryOperator::Not => DataType::Integer,
    }
}

/// Resolve the return type of Std or Aggregation function and re resolve it if it variant or dynamic
pub fn resolve_call_expression_return_type(
    env: &Environment,
    signature: &Signature,
    arguments: &Vec<Box<dyn Expression>>,
) -> DataType {
    let mut return_type = signature.return_type.clone();

    if let DataType::Dynamic(calculate_type) = return_type {
        return_type = calculate_type(&signature.parameters);

        // In Case that return type is variant for example [Int | Float] need to resolve it from arguments types
        // To be able to use it with other expressions
        if !arguments.is_empty() && return_type.is_variant() {
            let mut arguments_types = Vec::with_capacity(arguments.len());
            for argument in arguments {
                arguments_types.push(argument.expr_type(env));
            }
            return_type = calculate_type(&arguments_types);
        }
    }

    return_type
}

/// Return a [Diagnostic] with common type mismatch error message
#[inline(always)]
pub fn type_mismatch_error(
    location: Location,
    expected: DataType,
    actual: DataType,
) -> Box<Diagnostic> {
    Diagnostic::error(&format!(
        "Type mismatch expected `{}`, got `{}`",
        expected, actual
    ))
    .with_location(location)
    .as_boxed()
}
