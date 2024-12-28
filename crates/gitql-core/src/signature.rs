use super::values::base::Value;

use gitql_ast::types::base::DataType;

/// Standard function accept array of values and return single [`Value`]
pub type StandardFunction = fn(&[Box<dyn Value>]) -> Box<dyn Value>;

/// Aggregation function accept a selected row values for each row in group and return single [`Value`]
///
/// [`Vec<Vec<Value>>`] represent the selected values from each row in group
///
/// For Example if we have three rows in group and select name and email from each one
///
/// [[name, email], [name, email], [name, email]]
///
/// This implementation allow aggregation function to accept more than one parameter,
/// and also accept any Expression not only field name
///
pub type AggregationFunction = fn(&[Vec<Box<dyn Value>>]) -> Box<dyn Value>;

/// Window function  a selected row values for each row in a specific frame and return single [`Value`]
///
/// [`Vec<Vec<Value>>`] represent the selected values from each row in frame of rows
///
/// For Example if we have three rows in frame of row and select name and email from each one
///
/// [[name, email], [name, email], [name, email]]
///
/// This implementation allow Window` function to accept more than one parameter,
/// and also accept any Expression not only field name
///
pub type WindowFunction = fn(&[Vec<Box<dyn Value>>]) -> Vec<Box<dyn Value>>;

/// Signature struct is a representation of function type
///
/// Function type in GitQL used to track parameters and return type for now
/// but later can track parameter names to allow pass parameter by name and improve error messages
///
/// GitQL Function Signature has some rules to follow
///
/// Rules:
/// - Parameters must contains 0 or 1 [`VarargsType`] parameter type only.
/// - [`VarargsType`] must be the last parameter.
/// - You can add 0 or more [`DataType::Optional`] parameters.
/// - [`OptionalType`] parameters must be at the end but also before [`VarargsType`] if exists.
///
/// The return type can be a static [`DataType`] such as Int, Float or Dynamic
/// so you can return a dynamic type depending on parameters.
#[derive(Clone)]
pub struct Signature {
    pub parameters: Vec<Box<dyn DataType>>,
    pub return_type: Box<dyn DataType>,
}

impl Signature {
    /// Create Instance of [`Signature`] with parameters and return type
    pub fn new(parameters: Vec<Box<dyn DataType>>, return_type: Box<dyn DataType>) -> Self {
        Signature {
            parameters,
            return_type,
        }
    }

    /// Create Instance of [`Signature`] with return type and zero parameters
    pub fn with_return(return_type: Box<dyn DataType>) -> Self {
        Signature {
            parameters: Vec::default(),
            return_type,
        }
    }

    /// Add list of parameters to the [`Signature`]
    pub fn add_parameters(mut self, mut parameters: Vec<Box<dyn DataType>>) -> Self {
        self.parameters.append(&mut parameters);
        self
    }

    /// Add parameter to the [`Signature`]
    pub fn add_parameter(mut self, parameter: Box<dyn DataType>) -> Self {
        self.parameters.push(parameter);
        self
    }
}
