use std::any::Any;
use std::cmp::Ordering;

use dyn_clone::DynClone;
use gitql_ast::types::base::DataType;

dyn_clone::clone_trait_object!(Value);

pub trait Value: DynClone {
    fn literal(&self) -> String;
    
    fn equals(&self, other: &Box<dyn Value>) -> bool;

    fn compare(&self, other: &Box<dyn Value>) -> Option<Ordering>;

    fn data_type(&self) -> Box<dyn DataType>;

    fn as_any(&self) -> &dyn Any;

    fn perform_add_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_sub_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_mul_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_div_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_rem_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_caret_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_or_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_and_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_xor_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_logical_or_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_logical_and_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_logical_xor_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_shl_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_shr_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_index_op(&self, _index: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_slice_op(
        &self,
        _start: &Option<Box<dyn Value>>,
        _end: &Option<Box<dyn Value>>,
    ) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_eq_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_bang_eq_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_null_safe_eq_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_gt_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_gte_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_lt_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_lte_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_not_op(&self) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_neg_op(&self) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_bang_op(&self) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_contains_op(&self, _other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }

    fn perform_cast_op(&self, _target_type: &Box<dyn DataType>) -> Result<Box<dyn Value>, String> {
        Err("Unsupported operator for this type".to_string())
    }
}

impl PartialEq for Box<dyn Value> {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

impl PartialOrd for Box<dyn Value> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.compare(other)
    }
}
