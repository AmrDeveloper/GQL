use gitql_ast::types::array::ArrayType;
use gitql_ast::types::base::DataType;

/// Returns the type of of first element
/// (T1, T2, ...) -> T1
#[inline(always)]
pub fn first_element_type(elements: &[Box<dyn DataType>]) -> Box<dyn DataType> {
    elements[0].clone()
}

/// Returns the type of second element
/// (T1, T2, ...) -> T2
#[inline(always)]
pub fn second_element_type(elements: &[Box<dyn DataType>]) -> Box<dyn DataType> {
    elements[0].clone()
}

/// Returns Array type of the passed element type
/// T -> Array<T>
#[inline(always)]
pub fn array_of_type(element_type: Box<dyn DataType>) -> Box<dyn DataType> {
    Box::new(ArrayType {
        base: element_type.clone(),
    })
}

/// Returns element type of passed Array type
/// Array<T> -> T
#[inline(always)]
pub fn array_element_type(array: Box<dyn DataType>) -> Box<dyn DataType> {
    if let Some(other_array) = array.as_any().downcast_ref::<ArrayType>() {
        return other_array.base.clone();
    }
    panic!("Expect Array type")
}
