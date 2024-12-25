use std::any::Any;

use gitql_ast::types::base::DataType;

#[derive(Clone)]
pub struct DiffChangesType;

impl DataType for DiffChangesType {
    fn literal(&self) -> String {
        "DiffChangesType".to_owned()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
