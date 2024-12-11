use std::collections::HashMap;

use gitql_ast::expression::CastExpr;
use gitql_ast::expression::Expr;
use gitql_ast::statement::TableSelection;
use gitql_ast::types::any::AnyType;
use gitql_ast::types::base::DataType;
use gitql_ast::types::dynamic::DynamicType;
use gitql_ast::types::varargs::VarargsType;
use gitql_core::environment::Environment;

use crate::diagnostic::Diagnostic;
use crate::token::SourceLocation;

/// Checks if all values has the same type
/// If they have the same type, return it or return None
pub fn check_all_values_are_same_type(arguments: &[Box<dyn Expr>]) -> Option<Box<dyn DataType>> {
    let arguments_count = arguments.len();
    if arguments_count == 0 {
        return Some(Box::new(AnyType));
    }

    let data_type = arguments[0].expr_type();
    for argument in arguments.iter().take(arguments_count).skip(1) {
        let expr_type = argument.expr_type();
        if !data_type.equals(&expr_type) {
            return None;
        }
    }

    Some(data_type)
}

/// Check That function call arguments types are matches the parameter types
/// Return a Diagnostic Error if anything is wrong
pub fn check_function_call_arguments(
    arguments: &mut [Box<dyn Expr>],
    parameters: &[Box<dyn DataType>],
    function_name: String,
    location: SourceLocation,
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
        let parameter_type =
            resolve_dynamic_data_type(parameters, arguments, parameters.get(index).unwrap());
        let argument = arguments.get(index).unwrap();
        let argument_type = argument.expr_type();

        // Catch undefined arguments
        if argument_type.is_undefined() {
            return Err(Diagnostic::error(&format!(
                "Function `{}` argument number {} has Undefined type",
                function_name, index,
            ))
            .add_help("Make sure you used a correct field name")
            .add_help("Check column names for each table from docs website")
            .with_location(location)
            .as_boxed());
        }

        // Both types are equals
        if parameter_type.equals(&argument_type) {
            continue;
        }

        // Argument exp can be implicit casted to Parameter type
        if parameter_type.has_implicit_cast_from(argument) {
            arguments[index] = Box::new(CastExpr {
                value: argument.clone(),
                result_type: parameter_type.clone(),
            });
            continue;
        }

        // Argument type is not equal and can't be casted to parameter type
        return Err(Diagnostic::error(&format!(
            "Function `{}` argument number {} with type `{}` don't match expected type `{}`",
            function_name,
            index,
            argument_type.literal(),
            parameter_type.literal()
        ))
        .with_location(location)
        .as_boxed());
    }

    // Type check the optional parameters
    let last_optional_param_index = min_arguments_count + optional_parameters_count;
    for index in min_arguments_count..last_optional_param_index {
        if index >= arguments_count {
            return Ok(());
        }

        let parameter_type =
            resolve_dynamic_data_type(parameters, arguments, parameters.get(index).unwrap());
        let argument = arguments.get(index).unwrap();
        let argument_type = argument.expr_type();

        // Catch undefined arguments
        if argument_type.is_undefined() {
            return Err(Diagnostic::error(&format!(
                "Function `{}` argument number {} has Undefined type",
                function_name, index,
            ))
            .add_help("Make sure you used a correct field name")
            .add_help("Check column names for each table from docs website")
            .with_location(location)
            .as_boxed());
        }

        // Both types are equals
        if parameter_type.equals(&argument_type) {
            continue;
        }

        // Argument exp can be implicit casted to Parameter type
        if parameter_type.has_implicit_cast_from(argument) {
            arguments[index] = Box::new(CastExpr {
                value: argument.clone(),
                result_type: parameter_type.clone(),
            });
            continue;
        }

        // Argument type is not equal and can't be casted to parameter type
        return Err(Diagnostic::error(&format!(
            "Function `{}` argument number {} with type `{}` don't match expected type `{}`",
            function_name,
            index,
            argument_type.literal(),
            parameter_type.literal()
        ))
        .with_location(location)
        .as_boxed());
    }

    // Type check the variable parameters if exists
    if has_varargs_parameter {
        let varargs_type =
            resolve_dynamic_data_type(parameters, arguments, parameters.last().unwrap());
        for index in last_optional_param_index..arguments_count {
            let argument = arguments.get(index).unwrap();
            let argument_type = argument.expr_type();

            // Catch undefined arguments
            if argument_type.is_undefined() {
                return Err(Diagnostic::error(&format!(
                    "Function `{}` argument number {} has Undefined type",
                    function_name, index,
                ))
                .add_help("Make sure you used a correct field name")
                .add_help("Check column names for each table from docs website")
                .with_location(location)
                .as_boxed());
            }

            // Both types are equals
            if varargs_type.equals(&argument_type) {
                continue;
            }

            // Argument exp can be implicit casted to Parameter type
            if varargs_type.has_implicit_cast_from(argument) {
                arguments[index] = Box::new(CastExpr {
                    value: argument.clone(),
                    result_type: varargs_type.clone(),
                });
                continue;
            }

            return Err(Diagnostic::error(&format!(
                "Function `{}` argument number {} with type `{}` don't match expected type `{}`",
                function_name,
                index,
                &argument_type.literal(),
                &varargs_type.literal()
            ))
            .with_location(location)
            .as_boxed());
        }
    }

    Ok(())
}

/// Check that all selected fields types are defined correctly in selected tables
/// Return the columns classified for each table
/// Return a Diagnostic Error if anything is wrong
pub fn type_check_and_classify_selected_fields(
    env: &mut Environment,
    selected_tables: &Vec<String>,
    selected_columns: &Vec<String>,
    location: SourceLocation,
) -> Result<Vec<TableSelection>, Box<Diagnostic>> {
    let mut table_selections: Vec<TableSelection> = vec![];
    let mut table_index: HashMap<String, usize> = HashMap::new();
    for (index, table) in selected_tables.iter().enumerate() {
        table_selections.push(TableSelection {
            table_name: table.to_string(),
            columns_names: vec![],
        });
        table_index.insert(table.to_string(), index);
    }

    for selected_column in selected_columns {
        let mut is_column_resolved = false;
        for table in selected_tables {
            let table_columns = env.schema.tables_fields_names.get(table.as_str()).unwrap();

            // Check if this column name exists in current table
            if table_columns.contains(&selected_column.as_str()) {
                is_column_resolved = true;
                let table_selection_index = *table_index.get(table).unwrap();
                let selection = &mut table_selections[table_selection_index];
                selection.columns_names.push(selected_column.to_string());
                continue;
            }
        }

        if !is_column_resolved {
            // This case for aggregated values or functions
            if let Some(data_type) = env.resolve_type(selected_column) {
                if !data_type.is_undefined() {
                    if table_selections.is_empty() {
                        table_selections.push(TableSelection {
                            table_name: selected_tables
                                .first()
                                .unwrap_or(&"".to_string())
                                .to_string(),
                            columns_names: vec![selected_column.to_string()],
                        });
                    } else {
                        table_selections[0]
                            .columns_names
                            .push(selected_column.to_string());
                    }
                    continue;
                }
            }

            return Err(Diagnostic::error(&format!(
                "Column `{}` not exists in any of the selected tables",
                selected_column
            ))
            .add_help("Check the documentations to see available fields for each tables")
            .with_location(location)
            .as_boxed());
        }
    }

    Ok(table_selections)
}

/// Check that all projection columns are valid for this table name
/// Return a Diagnostic Error if anything is wrong
pub fn type_check_projection_symbols(
    env: &mut Environment,
    selected_tables: &[String],
    projection_names: &[String],
    projection_locations: &[SourceLocation],
) -> Result<(), Box<Diagnostic>> {
    for (index, selected_column) in projection_names.iter().enumerate() {
        let mut is_column_resolved = false;
        for table in selected_tables {
            let table_columns = env.schema.tables_fields_names.get(table.as_str()).unwrap();
            if table_columns.contains(&selected_column.as_str()) {
                is_column_resolved = true;
                break;
            }
        }

        if !is_column_resolved {
            return Err(Diagnostic::error(&format!(
                "Column `{}` not exists in any of the selected tables",
                selected_column
            ))
            .add_help("Check the documentations to see available fields for each tables")
            .with_location(projection_locations[index])
            .as_boxed());
        }
    }

    Ok(())
}

/// Resolve dynamic data type depending on the parameters and arguments types to actual DataType
#[allow(clippy::borrowed_box)]
pub fn resolve_dynamic_data_type(
    parameters: &[Box<dyn DataType>],
    arguments: &[Box<dyn Expr>],
    data_type: &Box<dyn DataType>,
) -> Box<dyn DataType> {
    // Resolve Dynamic type
    if let Some(dynamic_type) = data_type.as_any().downcast_ref::<DynamicType>() {
        let mut resolved_data_type = (dynamic_type.function)(parameters);

        // In Case that data type is Any or Variant [Type1 | Type2...] need to resolve it from arguments types
        // To be able to use it with other expressions
        if !arguments.is_empty() && (resolved_data_type.is_variant() || resolved_data_type.is_any())
        {
            let mut arguments_types: Vec<Box<dyn DataType>> = Vec::with_capacity(arguments.len());
            for argument in arguments {
                arguments_types.push(argument.expr_type());
            }
            resolved_data_type = (dynamic_type.function)(&arguments_types);
        }

        return resolved_data_type;
    }

    // Resolve ...Dynamic to ...<TYPE> recursively
    if let Some(varargs) = data_type.as_any().downcast_ref::<VarargsType>() {
        if varargs
            .base
            .as_any()
            .downcast_ref::<DynamicType>()
            .is_some()
        {
            let base = resolve_dynamic_data_type(parameters, arguments, &varargs.base);
            return Box::new(VarargsType { base });
        }
    }

    data_type.clone()
}
