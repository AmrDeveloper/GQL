use std::any::Any;
use std::cmp::Ordering;

use gitql_ast::types::DataType;
use gitql_core::values::Value;
use gix::Repository;
use gix::diff::blob::Platform;
use gix::object::tree::diff::Change;

use crate::gitql::types::diff_changes::DiffChangesType;

#[derive(PartialEq, Clone)]
pub enum DiffChangeKind {
    Addition,
    Deletion,
    Modification,
    Rewrite,
    Copy,
}

impl DiffChangeKind {
    pub fn from(change: &Change) -> Self {
        match change {
            Change::Addition { .. } => DiffChangeKind::Addition,
            Change::Deletion { .. } => DiffChangeKind::Deletion,
            Change::Modification { .. } => DiffChangeKind::Modification,
            Change::Rewrite {
                source_location: _,
                source_relation: _,
                source_entry_mode: _,
                source_id: _,
                diff: _,
                entry_mode: _,
                location: _,
                id: _,
                relation: _,
                copy,
            } => {
                if *copy {
                    DiffChangeKind::Rewrite
                } else {
                    DiffChangeKind::Copy
                }
            }
        }
    }

    pub fn mode(&self) -> char {
        match self {
            DiffChangeKind::Addition => 'A',
            DiffChangeKind::Deletion => 'D',
            DiffChangeKind::Modification => 'M',
            DiffChangeKind::Rewrite => 'R',
            DiffChangeKind::Copy => 'C',
        }
    }
}

#[derive(Clone)]
pub struct DiffChange {
    pub location: String,
    pub content: Vec<u8>,
    pub insertions: u32,
    pub removals: u32,
    pub kind: DiffChangeKind,
}

impl DiffChange {
    pub fn new_without_content(change: &Change, diff_cache: &mut Platform) -> Self {
        let location = change.location().to_string();
        let kind = DiffChangeKind::from(change);

        let (mut insertions, mut removals) = (0, 0);
        match change {
            Change::Rewrite {
                source_location: _,
                source_relation: _,
                source_entry_mode: _,
                source_id: _,
                diff,
                entry_mode: _,
                location: _,
                id: _,
                relation: _,
                copy: _,
            } => {
                if let Some(diff_line_stats) = diff {
                    insertions = diff_line_stats.insertions;
                    removals = diff_line_stats.removals;
                }
            }
            _ => {
                if let Ok(mut platform) = change.diff(diff_cache)
                    && let Ok(Some(counts)) = platform.line_counts()
                {
                    insertions = counts.insertions;
                    removals = counts.removals;
                };
            }
        }

        DiffChange {
            location,
            content: vec![],
            insertions,
            removals,
            kind,
        }
    }

    pub fn new_with_content(change: &Change, diff_cache: &mut Platform, repo: &Repository) -> Self {
        let mut diff_change = DiffChange::new_without_content(change, diff_cache);
        if let Ok(object) = repo.find_object(change.id())
            && let Ok(blob) = object.try_into_blob()
        {
            diff_change.content = blob.data.clone()
        }
        diff_change
    }
}

#[derive(Clone)]
pub struct DiffChangesValue {
    pub changes: Vec<DiffChange>,
}

impl DiffChangesValue {
    pub fn new(changes: Vec<DiffChange>) -> Self {
        DiffChangesValue { changes }
    }
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
