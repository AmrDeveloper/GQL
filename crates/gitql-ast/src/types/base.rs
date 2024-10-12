use std::any::Any;

use dyn_clone::DynClone;

use crate::expression::Expression;

use super::any::AnyType;
use super::array::ArrayType;
use super::boolean::BoolType;
use super::date::DateType;
use super::datetime::DateTimeType;
use super::float::FloatType;
use super::integer::IntType;
use super::null::NullType;
use super::optional::OptionType;
use super::range::RangeType;
use super::text::TextType;
use super::time::TimeType;
use super::undefined::UndefType;
use super::variant::VariantType;

dyn_clone::clone_trait_object!(DataType);

pub trait DataType: DynClone {
    fn literal(&self) -> String;

    fn equals(&self, _other: &Box<dyn DataType>) -> bool {
        false
    }

    fn as_any(&self) -> &dyn Any;

    /*
       fn has_add_op_with(&self, _other: &Box<dyn DataType>) -> bool {
           false
       }
    */

    fn can_perform_add_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    fn add_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    /*
    fn has_sub_op_with(&self, _other: &Box<dyn DataType>) -> bool {
        false
    }
    */

    fn can_perform_sub_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    fn sub_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    /*
       fn has_mul_op_with(&self, _other: &Box<dyn DataType>) -> bool {
           false
       }
    */

    fn can_perform_mul_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    fn mul_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    /*
       fn has_div_op_with(&self, _other: &Box<dyn DataType>) -> bool {
           false
       }
    */

    fn can_perform_div_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    fn div_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    /*
        fn has_rem_op_with(&self, _other: &Box<dyn DataType>) -> bool {
            false
        }
    */

    fn can_perform_rem_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    fn rem_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    fn can_perform_caret_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /*
    fn has_caret_op_with(&self, _other: &Box<dyn DataType>) -> bool {
            false
        }
    */

    fn caret_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    fn can_perform_or_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /*
    fn has_or_op_with(&self, _other: &Box<dyn DataType>) -> bool {
            false
        }
    */

    fn or_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    fn can_perform_and_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /*
    fn has_and_op_with(&self, _other: &Box<dyn DataType>) -> bool {
            false
        }
    */

    fn and_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    fn can_perform_xor_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /*
       fn has_xor_op_with(&self, _other: &Box<dyn DataType>) -> bool {
           false
       }
    */

    fn xor_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    fn can_perform_shl_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /*
    fn has_shl_op_with(&self, _other: &Box<dyn DataType>) -> bool {
           false
       }
    */

    fn shl_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    fn can_perform_shr_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /*
    fn has_shr_op_with(&self, _other: &Box<dyn DataType>) -> bool {
           false
       }
    */

    fn shr_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    fn can_perform_logical_or_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /*
    fn has_logical_or_op_with(&self, _other: &Box<dyn DataType>) -> bool {
            false
        }
    */

    fn logical_or_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    fn can_perform_logical_and_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /*
       fn has_logical_and_op_with(&self, _other: &Box<dyn DataType>) -> bool {
           false
       }
    */

    fn logical_and_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    fn can_perform_logical_xor_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /*
       fn has_logical_xor_op_with(&self, _other: &Box<dyn DataType>) -> bool {
           false
       }
    */

    fn logical_xor_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    fn can_perform_index_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /*
       fn has_index_op_with(&self, _other: &Box<dyn DataType>) -> bool {
           false
       }
    */

    fn index_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    fn can_perform_slice_op(&self) -> bool {
        false
    }

    fn can_perform_slice_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /*
    fn has_slice_op_with(
        &self,
        _start: &Option<Box<dyn DataType>>,
        _end: &Option<Box<dyn DataType>>,
    ) -> bool {
        false
    }
     */

    fn slice_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    /*
        fn has_eq_op_with(&self, _other: &Box<dyn DataType>) -> bool {
            false
        }
    */

    fn can_perform_eq_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    fn can_perform_bang_eq_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /*
    fn has_bang_eq_op_with(&self, _other: &Box<dyn DataType>) -> bool {
        false
    } */

    fn can_perform_null_safe_eq_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /*
       fn has_null_safe_eq_op_with(&self, _other: &Box<dyn DataType>) -> bool {
           false
       }
    */

    fn can_perform_gt_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    fn can_perform_gte_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    fn can_perform_lt_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    fn can_perform_lte_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /*
       fn has_gt_op_with(&self, _other: &Box<dyn DataType>) -> bool {
           false
       }

       fn has_gte_op_with(&self, _other: &Box<dyn DataType>) -> bool {
           false
       }

       fn has_lt_op_with(&self, _other: &Box<dyn DataType>) -> bool {
           false
       }

       fn has_lte_op_with(&self, _other: &Box<dyn DataType>) -> bool {
           false
       }
    */

    fn can_perform_not_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /*
       fn has_not_op(&self) -> bool {
           false
       }
    */

    fn not_op_result_type(&self) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    fn can_perform_neg_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /*
       fn has_neg_op(&self) -> bool {
           false
       }
    */

    fn neg_op_result_type(&self) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    fn can_perform_bang_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /*
       fn has_bang_op(&self) -> bool {
           false
       }
    */

    fn bang_op_result_type(&self) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    fn can_perform_contains_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /*
       fn has_contains_op_with(&self, _other: &Box<dyn DataType>) -> bool {
           false
       }
    */

    fn contains_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    // Performed with constants values
    fn has_implicit_cast_from(&self, _expr: &Box<dyn Expression>) -> bool {
        false
    }

    fn can_perform_explicit_cast_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }
}

impl dyn DataType {
    pub fn is_any(&self) -> bool {
        self.as_any().downcast_ref::<AnyType>().is_some()
    }

    pub fn is_text(&self) -> bool {
        self.as_any().downcast_ref::<TextType>().is_some()
    }

    pub fn is_int(&self) -> bool {
        self.as_any().downcast_ref::<IntType>().is_some()
    }

    pub fn is_float(&self) -> bool {
        self.as_any().downcast_ref::<FloatType>().is_some()
    }

    pub fn is_number(&self) -> bool {
        self.is_int() || self.is_float()
    }

    pub fn is_bool(&self) -> bool {
        self.as_any().downcast_ref::<BoolType>().is_some()
    }

    pub fn is_date(&self) -> bool {
        self.as_any().downcast_ref::<DateType>().is_some()
    }

    pub fn is_time(&self) -> bool {
        self.as_any().downcast_ref::<TimeType>().is_some()
    }

    pub fn is_datetime(&self) -> bool {
        self.as_any().downcast_ref::<DateTimeType>().is_some()
    }

    pub fn is_array(&self) -> bool {
        self.as_any().downcast_ref::<ArrayType>().is_some()
    }

    pub fn is_range(&self) -> bool {
        self.as_any().downcast_ref::<RangeType>().is_some()
    }

    pub fn is_variant(&self) -> bool {
        self.as_any().downcast_ref::<VariantType>().is_some()
    }

    pub fn is_variant_with(&self, matcher: fn(&Box<dyn DataType>) -> bool) -> bool {
        if let Some(variant_type) = self.as_any().downcast_ref::<VariantType>() {
            for variant in variant_type.variants.iter() {
                if matcher(variant) {
                    return true;
                }
            }
        }
        false
    }

    pub fn is_optional(&self) -> bool {
        self.as_any().downcast_ref::<OptionType>().is_some()
    }

    pub fn is_varargs(&self) -> bool {
        self.as_any().downcast_ref::<VariantType>().is_some()
    }

    pub fn is_undefined(&self) -> bool {
        self.as_any().downcast_ref::<UndefType>().is_some()
    }

    pub fn is_null(&self) -> bool {
        self.as_any().downcast_ref::<NullType>().is_some()
    }
}

impl PartialEq for Box<dyn DataType> {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}
