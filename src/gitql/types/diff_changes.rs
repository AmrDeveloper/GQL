use std::any::Any;

use gitql_ast::types::DataType;

#[derive(Clone)]
pub struct DiffChangesType;

impl DataType for DiffChangesType {
    fn literal(&self) -> String {
        "DiffChangesType".to_owned()
    }

    #[allow(clippy::borrowed_box)]
    fn equals(&self, other: &Box<dyn DataType>) -> bool {
        let self_type: Box<dyn DataType> = Box::new(DiffChangesType);
        other.is_any()
            || other.is_variant_contains(&self_type)
            || other.as_any().downcast_ref::<DiffChangesType>().is_some()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
