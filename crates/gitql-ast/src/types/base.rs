use std::any::Any;
use std::fmt;

use dyn_clone::DynClone;

use crate::expression::Expr;

use super::any::AnyType;
use super::array::ArrayType;
use super::boolean::BoolType;
use super::composite::CompositeType;
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

/// The in memory representation of the Data type in the GitQL query engine
pub trait DataType: DynClone {
    /// Return the literal representation for this [`DataType`]
    fn literal(&self) -> String;

    /// Return if other [`DataType`] is equal or not to current Type
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn equals(&self, other: &Box<dyn DataType>) -> bool {
        false
    }

    /// Return the current value as dynamic [`Any`]
    fn as_any(&self) -> &dyn Any;

    /// Return a list of types that it's possible to perform `+` operator with
    /// between current DataType and any one of them
    fn can_perform_add_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /// Return the expected type after perform `=` operator between current type and argument type
    ///
    /// Note that you don't need to check again that the argument type is possible to perform operator with
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn add_op_result_type(&self, other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    /// Return a list of types that it's possible to perform `-` operator with
    /// between current DataType and any one of them
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn can_perform_sub_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /// Return the expected type after perform `-` operator between current type and argument type
    ///
    /// Note that you don't need to check again that the argument type is possible to perform operator with
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn sub_op_result_type(&self, other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    /// Return a list of types that it's possible to perform `*` operator with
    /// between current DataType and any one of them
    fn can_perform_mul_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /// Return the expected type after perform `*` operator between current type and argument type
    ///
    /// Note that you don't need to check again that the argument type is possible to perform operator with
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn mul_op_result_type(&self, other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    /// Return a list of types that it's possible to perform `/` operator with
    /// between current DataType and any one of them
    fn can_perform_div_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /// Return the expected type after perform `/` operator between current type and argument type
    ///
    /// Note that you don't need to check again that the argument type is possible to perform operator with
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn div_op_result_type(&self, other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    /// Return a list of types that it's possible to perform `%` operator with
    /// between current DataType and any one of them
    fn can_perform_rem_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /// Return the expected type after perform `&` operator between current type and argument type
    ///
    /// Note that you don't need to check again that the argument type is possible to perform operator with
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn rem_op_result_type(&self, other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    /// Return a list of types that it's possible to perform `^` operator with
    /// between current DataType and any one of them
    fn can_perform_caret_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /// Return the expected type after perform `^` operator between current type and argument type
    ///
    /// Note that you don't need to check again that the argument type is possible to perform operator with
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn caret_op_result_type(&self, other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    /// Return a list of types that it's possible to perform `|` operator with
    /// between current DataType and any one of them
    fn can_perform_or_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /// Return the expected type after perform `|` operator between current type and argument type
    ///
    /// Note that you don't need to check again that the argument type is possible to perform operator with
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn or_op_result_type(&self, other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    /// Return a list of types that it's possible to perform `&` operator with
    /// between current DataType and any one of them
    fn can_perform_and_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /// Return the expected type after perform `&` operator between current type and argument type
    ///
    /// Note that you don't need to check again that the argument type is possible to perform operator with
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn and_op_result_type(&self, other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    /// Return a list of types that it's possible to perform `#` operator with
    /// between current DataType and any one of them
    fn can_perform_xor_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /// Return the expected type after perform `#` operator between current type and argument type
    ///
    /// Note that you don't need to check again that the argument type is possible to perform operator with
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn xor_op_result_type(&self, other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    /// Return a list of types that it's possible to perform `<<` operator with
    /// between current DataType and any one of them
    fn can_perform_shl_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /// Return the expected type after perform `<<` operator between current type and argument type
    ///
    /// Note that you don't need to check again that the argument type is possible to perform operator with
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn shl_op_result_type(&self, other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    /// Return a list of types that it's possible to perform `>>` operator with
    /// between current DataType and any one of them
    fn can_perform_shr_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /// Return the expected type after perform `>>` operator between current type and argument type
    ///
    /// Note that you don't need to check again that the argument type is possible to perform operator with
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn shr_op_result_type(&self, other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    /// Return a list of types that it's possible to perform `||` or  `OR` operator with
    /// between current DataType and any one of them
    fn can_perform_logical_or_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /// Return the expected type after perform `||` or `OR` operator between current type and argument type
    ///
    /// Note that you don't need to check again that the argument type is possible to perform operator with
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn logical_or_op_result_type(&self, other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    /// Return a list of types that it's possible to perform `&&` or `AND` operator with
    /// between current DataType and any one of them
    fn can_perform_logical_and_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /// Return the expected type after perform `&&` or `AND` operator between current type and argument type
    ///
    /// Note that you don't need to check again that the argument type is possible to perform operator with
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn logical_and_op_result_type(&self, other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    /// Return a list of types that it's possible to perform `XOR' operator with
    /// between current DataType and any one of them
    fn can_perform_logical_xor_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /// Return the expected type after perform 'XOR' operator between current type and argument type
    ///
    /// Note that you don't need to check again that the argument type is possible to perform operator with
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn logical_xor_op_result_type(&self, other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    /// Return a list of types that it's possible to perform `[I]' operator with
    /// between current DataType and any one of them
    fn can_perform_index_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /// Return the expected type after perform '[]' operator on current type
    ///
    /// * `other` - The index type
    ///
    /// Note that you don't need to check again that the argument type is possible to perform operator with
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn index_op_result_type(&self, other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    /// Return true if this type support Slice operator with
    fn can_perform_slice_op(&self) -> bool {
        false
    }

    /// Return a list of types that it's possible to perform `[S : E]' operator with
    /// between current DataType and any one of them
    fn can_perform_slice_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /// Return a list of types that it's possible to perform `=' operator with
    /// between current DataType and any one of them
    ///
    /// No need to define the result type, it always BoolType
    fn can_perform_eq_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /// Return a list of types that it's possible to perform `!=' or `<>` operator with
    /// between current DataType and any one of them
    ///
    /// No need to define the result type, it always BoolType
    fn can_perform_bang_eq_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /// Return a list of types that it's possible to perform `<=>' operator with
    /// between current DataType and any one of them
    ///
    /// No need to define the result type, it always BoolType
    fn can_perform_null_safe_eq_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /// Return a list of types that it's possible to perform `>' operator with
    /// between current DataType and any one of them
    ///
    /// No need to define the result type, it always BoolType
    fn can_perform_gt_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /// Return a list of types that it's possible to perform `>=' operator with
    /// between current DataType and any one of them
    ///
    /// No need to define the result type, it always BoolType
    fn can_perform_gte_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /// Return a list of types that it's possible to perform `<' operator with
    /// between current DataType and any one of them
    ///
    /// No need to define the result type, it always BoolType
    fn can_perform_lt_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /// Return a list of types that it's possible to perform `=<' operator with
    /// between current DataType and any one of them
    ///
    /// No need to define the result type, it always BoolType
    fn can_perform_lte_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /// Return a list of types that it's possible to perform unary `NOT' operator with
    /// between current DataType and any one of them
    fn can_perform_not_op(&self) -> bool {
        false
    }

    /// Return the expected type after perform unary `NOT' operator on current type
    ///
    /// Note that you don't need to check again that the argument type is possible to perform operator with
    fn not_op_result_type(&self) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    /// Return a list of types that it's possible to perform unary `-' operator with
    /// between current DataType and any one of them
    fn can_perform_neg_op(&self) -> bool {
        false
    }

    /// Return the expected type after perform unary `-' operator on current type
    ///
    /// Note that you don't need to check again that the argument type is possible to perform operator with
    fn neg_op_result_type(&self) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    /// Return a list of types that it's possible to perform unary `!' operator with
    /// between current DataType and any one of them
    fn can_perform_bang_op(&self) -> bool {
        false
    }

    /// Return the expected type after perform unary `!' operator on current type
    ///
    /// Note that you don't need to check again that the argument type is possible to perform operator with
    fn bang_op_result_type(&self) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    /// Return a list of types that it's possible to perform unary `@>' operator with
    /// between current DataType and any one of them
    ///
    /// No need to define the result type, it always BoolType
    fn can_perform_contains_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }

    /// Return true if this Expression can be casted to the current type without any evaluation
    ///
    /// For example casting Text with specific format to Date or Time
    ///
    /// The result type is equal to the current type
    #[allow(unused_variables)]
    #[allow(clippy::borrowed_box)]
    fn has_implicit_cast_from(&self, expr: &Box<dyn Expr>) -> bool {
        false
    }

    /// Return a list of types that it's possible to perform `Cast' operator to
    ///
    /// For example casting column with float value to integer or verse versa
    ///
    /// The result type is equal to the current type
    fn can_perform_explicit_cast_op_to(&self) -> Vec<Box<dyn DataType>> {
        vec![]
    }
}

impl dyn DataType {
    /// Return true if this type is [`AnyType`]
    pub fn is_any(&self) -> bool {
        self.as_any().downcast_ref::<AnyType>().is_some()
    }

    /// Return true if this type is [`TextType`]
    pub fn is_text(&self) -> bool {
        self.as_any().downcast_ref::<TextType>().is_some()
    }

    /// Return true if this type is [`IntType`]
    pub fn is_int(&self) -> bool {
        self.as_any().downcast_ref::<IntType>().is_some()
    }

    /// Return true if this type is [`FloatType`]
    pub fn is_float(&self) -> bool {
        self.as_any().downcast_ref::<FloatType>().is_some()
    }

    /// Return true if this type is [`IntType`] or [`FloatType`]
    pub fn is_number(&self) -> bool {
        self.is_int() || self.is_float()
    }

    /// Return true if this type is [`BoolType`]
    pub fn is_bool(&self) -> bool {
        self.as_any().downcast_ref::<BoolType>().is_some()
    }

    /// Return true if this type is [`DateType`]
    pub fn is_date(&self) -> bool {
        self.as_any().downcast_ref::<DateType>().is_some()
    }

    /// Return true if this type is [`TimeType`]
    pub fn is_time(&self) -> bool {
        self.as_any().downcast_ref::<TimeType>().is_some()
    }

    /// Return true if this type is [`DateTimeType`]
    pub fn is_date_time(&self) -> bool {
        self.as_any().downcast_ref::<DateTimeType>().is_some()
    }

    /// Return true if this type is [`ArrayType`]
    pub fn is_array(&self) -> bool {
        self.as_any().downcast_ref::<ArrayType>().is_some()
    }

    /// Return true if this type is [`RangeType`]
    pub fn is_range(&self) -> bool {
        self.as_any().downcast_ref::<RangeType>().is_some()
    }

    /// Return true if this type is [`VariantType`]
    pub fn is_variant(&self) -> bool {
        self.as_any().downcast_ref::<VariantType>().is_some()
    }

    /// Return true if this type is [`VariantType`]
    /// and applying the matcher function is return true on one of the variants
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

    /// Return true if this type is [`VariantType`] and contain specific type as one of it variants
    #[allow(clippy::borrowed_box)]
    pub fn is_variant_contains(&self, other: &Box<dyn DataType>) -> bool {
        if let Some(variant_type) = self.as_any().downcast_ref::<VariantType>() {
            return variant_type.variants.contains(other);
        }
        false
    }

    /// Return true if this type is [`OptionType`]
    pub fn is_optional(&self) -> bool {
        self.as_any().downcast_ref::<OptionType>().is_some()
    }

    /// Return true if this type is [`VariantType`]
    pub fn is_varargs(&self) -> bool {
        self.as_any().downcast_ref::<VariantType>().is_some()
    }

    /// Return true if this type is [`CompositeType`]
    pub fn is_composite(&self) -> bool {
        self.as_any().downcast_ref::<CompositeType>().is_some()
    }

    /// Return true if this type is [`UndefType`]
    pub fn is_undefined(&self) -> bool {
        self.as_any().downcast_ref::<UndefType>().is_some()
    }

    /// Return true if this type is [`NullType`]
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
