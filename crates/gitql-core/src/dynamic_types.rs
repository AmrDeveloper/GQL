use crate::types::DataType;

/// Returns the type of of first element
/// (T1, T2, ...) -> T1
#[inline(always)]
pub fn first_element_type(elements: &[DataType]) -> DataType {
    elements[0].clone()
}

/// Returns the type of second element
/// (T1, T2, ...) -> T2
#[inline(always)]
pub fn second_element_type(elements: &[DataType]) -> DataType {
    elements[0].clone()
}

/// Returns Array type of the passed element type
/// T -> Array<T>
#[inline(always)]
pub fn array_of_type(element_type: DataType) -> DataType {
    DataType::Array(Box::new(element_type))
}

/// Returns element type of passed Array type
/// Array<T> -> T
#[inline(always)]
pub fn array_element_type(array: DataType) -> DataType {
    match array {
        DataType::Array(element_type) => *element_type.clone(),
        _ => panic!("Expect Array type"),
    }
}
