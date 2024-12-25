use std::any::Any;
use std::cmp::Ordering;

use gitql_ast::types::base::DataType;
use gitql_core::values::base::Value;

use crate::gitql::types::diff_changes::DiffChangesType;

#[derive(PartialEq, Clone)]
pub enum DiffChangeKind {
    Addition,
    Deletion,
    Modification,
    Rewrite,
}

#[derive(Clone)]
pub struct DiffChange {
    pub location: String,
    pub content: Vec<u8>,
    pub insertions: u32,
    pub removals: u32,

    #[allow(dead_code)]
    pub kind: DiffChangeKind,
}

impl DiffChange {
    pub fn new(kind: DiffChangeKind) -> Self {
        DiffChange {
            location: String::default(),
            content: vec![],
            insertions: 0,
            removals: 0,
            kind,
        }
    }
}

#[derive(Clone)]
pub struct DiffChangesValue {
    pub changes: Vec<DiffChange>,
}

impl Value for DiffChangesValue {
    fn literal(&self) -> String {
        format!("{} Changes", self.changes.len())
    }

    fn equals(&self, _other: &Box<dyn Value>) -> bool {
        false
    }

    fn compare(&self, _other: &Box<dyn Value>) -> Option<Ordering> {
        None
    }

    fn data_type(&self) -> Box<dyn DataType> {
        Box::new(DiffChangesType)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
