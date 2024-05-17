use crate::types::DataType;

pub struct Signature {
    pub parameters: Vec<DataType>,
    pub return_type: DataType,
}
