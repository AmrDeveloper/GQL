use crate::types::DataType;

/// Signature struct is a representation of function type
///
/// Function type in GitQL used to track parameters and return type for now
/// but later can track parameter names to allow pass parameter by name and improve error messages
///
/// GitQL Function Signature has some rules to follow
///
/// Rules:
/// - Parameters must contains 0 or 1 [`DataType::Varargs`] parameter type only.
/// - [`DataType::Varargs`] must be the last parameter.
/// - You can add 0 or more [`DataType::Optional`] parameters.
/// - [`DataType::Optional`] parameters must be at the end but also before [`DataType::Varargs`] if exists.
///
/// The return type can be a static [`DataType`] such as Int, Flow or Dynamic
/// so you can return a dynamic type depending on parameters.
pub struct Signature {
    pub parameters: Vec<DataType>,
    pub return_type: DataType,
}
