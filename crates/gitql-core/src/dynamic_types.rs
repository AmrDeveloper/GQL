use crate::types::DataType;

/// Return a clone of the first type from the list
pub fn type_of_first_element(elements: &[DataType]) -> DataType {
    elements[0].clone()
}

/// Return a clone of the second type from the list
pub fn type_of_second_element(elements: &[DataType]) -> DataType {
    elements[1].clone()
}

/// Return Array type of element type equal of first element type
pub fn array_type_of_first_element_type(elements: &[DataType]) -> DataType {
    let first_element_type = &elements[0];
    DataType::Array(first_element_type)
}

/// Return a clone of the first array element type of first type
pub fn array_element_type_of_first_element(elements: &[DataType]) -> DataType {
    let first_element_type = &elements[0];
    match first_element_type {
        DataType::Array(element_type) => *element_type.clone(),
        _ => panic!("First element type must be an Array"),
    }
}
