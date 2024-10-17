use std::any::Any;
use std::fmt;

use dyn_clone::DynClone;

use crate::expression::Expr;

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

    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn equals(&self, other: &Box<dyn DataType>) -> bool {
        false
    }

    fn as_any(&self) -> &dyn Any;

    fn can_perform_add_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn add_op_result_type(&self, other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn can_perform_sub_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn sub_op_result_type(&self, other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    fn can_perform_mul_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn mul_op_result_type(&self, other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn can_perform_div_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn div_op_result_type(&self, other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    fn can_perform_rem_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn rem_op_result_type(&self, other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    fn can_perform_caret_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn caret_op_result_type(&self, other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    fn can_perform_or_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn or_op_result_type(&self, other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    fn can_perform_and_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn and_op_result_type(&self, other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    fn can_perform_xor_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn xor_op_result_type(&self, other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    fn can_perform_shl_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn shl_op_result_type(&self, other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    fn can_perform_shr_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn shr_op_result_type(&self, other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    fn can_perform_logical_or_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn logical_or_op_result_type(&self, other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    fn can_perform_logical_and_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn logical_and_op_result_type(&self, other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    fn can_perform_logical_xor_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn logical_xor_op_result_type(&self, other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    fn can_perform_index_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn index_op_result_type(&self, other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    fn can_perform_slice_op(&self) -> bool {
        false
    }

    fn can_perform_slice_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    fn can_perform_eq_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    fn can_perform_bang_eq_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    fn can_perform_null_safe_eq_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

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

    fn can_perform_not_op(&self) -> bool {
        false
    }

    fn not_op_result_type(&self) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    fn can_perform_neg_op(&self) -> bool {
        false
    }

    fn neg_op_result_type(&self) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    fn can_perform_bang_op(&self) -> bool {
        false
    }

    fn bang_op_result_type(&self) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    fn can_perform_contains_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    fn can_perform_contained_by_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn has_implicit_cast_from(&self, expr: &Box<dyn Expr>) -> bool {
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

    #[allow(clippy::borrowed_box)]
    pub fn is_variant_contains(&self, other: &Box<dyn DataType>) -> bool {
        if let Some(variant_type) = self.as_any().downcast_ref::<VariantType>() {
            return variant_type.variants.contains(other);
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

impl fmt::Display for Box<dyn DataType> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.literal())
    }
}

impl PartialEq for Box<dyn DataType> {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}
