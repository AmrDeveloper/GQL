use std::collections::HashMap;
use std::collections::HashSet;

use gitql_ast::types::boolean::BoolType;
use gitql_ast::types::integer::IntType;
use gitql_ast::types::text::TextType;
use gitql_core::signature::Signature;
use gitql_core::signature::StandardFunction;
use gitql_core::values::base::Value;
use gitql_core::values::boolean::BoolValue;
use gitql_core::values::integer::IntValue;
use gitql_core::values::text::TextValue;

use crate::gitql::types::diff_changes::DiffChangesType;
use crate::gitql::values::diff_changes::DiffChangeKind;
use crate::gitql::values::diff_changes::DiffChangesValue;

#[inline(always)]
pub(crate) fn register_diffs_functions(map: &mut HashMap<&'static str, StandardFunction>) {
    map.insert("diff_content", diff_changes_full_content);
    map.insert("diff_added_content", diff_changes_added_content);
    map.insert("diff_deleted_content", diff_changes_deleted_content);
    map.insert("diff_modified_content", diff_changes_modified_content);

    map.insert("diff_content_contains", diff_changes_full_content_contains);
    map.insert(
        "diff_added_content_contains",
        diff_changes_added_content_contains,
    );
    map.insert(
        "diff_deleted_content_contains",
        diff_changes_deleted_content_contains,
    );
    map.insert(
        "diff_modified_content_contains",
        diff_changes_modified_content_contains,
    );

    map.insert("diff_files_count", diff_changes_files_count);

    map.insert("is_diff_has_file", diff_changes_contains_file);
}

#[inline(always)]
pub(crate) fn register_diffs_function_signatures(map: &mut HashMap<&'static str, Signature>) {
    map.insert(
        "diff_content",
        Signature {
            parameters: vec![Box::new(DiffChangesType)],
            return_type: Box::new(TextType),
        },
    );

    map.insert(
        "diff_added_content",
        Signature {
            parameters: vec![Box::new(DiffChangesType)],
            return_type: Box::new(TextType),
        },
    );

    map.insert(
        "diff_deleted_content",
        Signature {
            parameters: vec![Box::new(DiffChangesType)],
            return_type: Box::new(TextType),
        },
    );

    map.insert(
        "diff_modified_content",
        Signature {
            parameters: vec![Box::new(DiffChangesType)],
            return_type: Box::new(TextType),
        },
    );

    map.insert(
        "diff_content_contains",
        Signature {
            parameters: vec![Box::new(DiffChangesType), Box::new(TextType)],
            return_type: Box::new(TextType),
        },
    );

    map.insert(
        "diff_added_content_contains",
        Signature {
            parameters: vec![Box::new(DiffChangesType), Box::new(TextType)],
            return_type: Box::new(BoolType),
        },
    );

    map.insert(
        "diff_deleted_content_contains",
        Signature {
            parameters: vec![Box::new(DiffChangesType), Box::new(TextType)],
            return_type: Box::new(BoolType),
        },
    );

    map.insert(
        "diff_modified_content_contains",
        Signature {
            parameters: vec![Box::new(DiffChangesType), Box::new(TextType)],
            return_type: Box::new(BoolType),
        },
    );

    map.insert(
        "diff_files_count",
        Signature {
            parameters: vec![Box::new(DiffChangesType)],
            return_type: Box::new(IntType),
        },
    );

    map.insert(
        "is_diff_has_file",
        Signature {
            parameters: vec![Box::new(DiffChangesType)],
            return_type: Box::new(BoolType),
        },
    );
}

fn diff_changes_full_content(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    if let Some(changes) = values[0].as_any().downcast_ref::<DiffChangesValue>() {
        let mut content = String::new();
        for change in changes.changes.iter() {
            content += &String::from_utf8_lossy(&change.content);
        }

        return Box::new(TextValue { value: content });
    }
    Box::new(TextValue {
        value: String::default(),
    })
}

fn diff_changes_added_content(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    if let Some(changes) = values[0].as_any().downcast_ref::<DiffChangesValue>() {
        let mut content = String::new();
        for change in changes.changes.iter() {
            if change.kind == DiffChangeKind::Addition {
                content += &String::from_utf8_lossy(&change.content);
            }
        }

        return Box::new(TextValue { value: content });
    }

    Box::new(TextValue {
        value: String::default(),
    })
}

fn diff_changes_deleted_content(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    if let Some(changes) = values[0].as_any().downcast_ref::<DiffChangesValue>() {
        let mut content = String::new();
        for change in changes.changes.iter() {
            if change.kind == DiffChangeKind::Deletion {
                content += &String::from_utf8_lossy(&change.content);
            }
        }

        return Box::new(TextValue { value: content });
    }

    Box::new(TextValue {
        value: String::default(),
    })
}

fn diff_changes_modified_content(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    if let Some(changes) = values[0].as_any().downcast_ref::<DiffChangesValue>() {
        let mut content = String::new();
        for change in changes.changes.iter() {
            if change.kind == DiffChangeKind::Modification {
                content += &String::from_utf8_lossy(&change.content);
            }
        }

        return Box::new(TextValue { value: content });
    }

    Box::new(TextValue {
        value: String::default(),
    })
}

fn diff_changes_full_content_contains(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    if let Some(changes) = values[0].as_any().downcast_ref::<DiffChangesValue>() {
        let str = values[1].as_text().unwrap();
        let mut content = String::new();
        for change in changes.changes.iter() {
            content += &String::from_utf8_lossy(&change.content);
        }

        let is_contains = content.contains(&str);
        return Box::new(BoolValue { value: is_contains });
    }
    Box::new(BoolValue { value: false })
}

fn diff_changes_added_content_contains(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    if let Some(changes) = values[0].as_any().downcast_ref::<DiffChangesValue>() {
        let str = values[1].as_text().unwrap();
        let mut content = String::new();
        for change in changes.changes.iter() {
            if change.kind == DiffChangeKind::Addition {
                content += &String::from_utf8_lossy(&change.content);
            }
        }

        let is_contains = content.contains(&str);
        return Box::new(BoolValue { value: is_contains });
    }
    Box::new(BoolValue { value: false })
}

fn diff_changes_deleted_content_contains(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    if let Some(changes) = values[0].as_any().downcast_ref::<DiffChangesValue>() {
        let str = values[1].as_text().unwrap();
        let mut content = String::new();
        for change in changes.changes.iter() {
            if change.kind == DiffChangeKind::Deletion {
                content += &String::from_utf8_lossy(&change.content);
            }
        }

        let is_contains = content.contains(&str);
        return Box::new(BoolValue { value: is_contains });
    }
    Box::new(BoolValue { value: false })
}

fn diff_changes_modified_content_contains(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    if let Some(changes) = values[0].as_any().downcast_ref::<DiffChangesValue>() {
        let str = values[1].as_text().unwrap();
        let mut content = String::new();
        for change in changes.changes.iter() {
            if change.kind == DiffChangeKind::Modification {
                content += &String::from_utf8_lossy(&change.content);
            }
        }

        let is_contains = content.contains(&str);
        return Box::new(BoolValue { value: is_contains });
    }
    Box::new(BoolValue { value: false })
}

fn diff_changes_files_count(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    if let Some(changes) = values[0].as_any().downcast_ref::<DiffChangesValue>() {
        let mut unique_files: HashSet<&String> = HashSet::new();
        for change in changes.changes.iter() {
            unique_files.insert(&change.location);
        }
        let value = unique_files.len() as i64;
        return Box::new(IntValue { value });
    }
    Box::new(IntValue { value: 0 })
}

fn diff_changes_contains_file(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    if let Some(changes) = values[0].as_any().downcast_ref::<DiffChangesValue>() {
        let file = values[1].as_text().unwrap();
        for change in changes.changes.iter() {
            if change.location.eq(&file) {
                return Box::new(BoolValue { value: true });
            }
        }
    }
    Box::new(BoolValue { value: false })
}
