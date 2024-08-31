use crate::types::DataType;

/// Return a clone of the first type from the list
pub fn type_of_first_element(elements: &[DataType]) -> DataType {
    elements[0].clone()
}

/// Return a clone of the first array element type of first type
pub fn array_element_type_of_first_element(elements: &[DataType]) -> DataType {
    let first_element_type = &elements[0];
    match first_element_type {
        DataType::Array(element_type) => *element_type.clone(),
        _ => panic!("First element type must be an Array"),
    }
}
