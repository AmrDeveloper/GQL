use std::any::Any;

use dyn_clone::DynClone;

use super::types::array::ArrayType;
use super::types::boolean::BoolType;
use super::types::integer::IntType;
use super::types::null::NullType;
use super::types::text::TextType;
use super::types::DataType;

use crate::interval::Interval;
use crate::operator::ArithmeticOperator;
use crate::operator::BinaryBitwiseOperator;
use crate::operator::BinaryLogicalOperator;
use crate::operator::ComparisonOperator;
use crate::operator::GroupComparisonOperator;
use crate::operator::PrefixUnaryOperator;
use crate::types::float::FloatType;
use crate::types::interval::IntervalType;
use crate::types::row::RowType;

#[derive(PartialEq)]
pub enum ExprKind {
    Assignment,
    String,
    Symbol,
    Array,
    GlobalVariable,
    Number,
    Boolean,
    Interval,
    PrefixUnary,
    Index,
    Slice,
    Arithmetic,
    Comparison,
    GroupComparison,
    Contains,
    ContainedBy,
    Like,
    Regex,
    Glob,
    Logical,
    Bitwise,
    Call,
    BenchmarkCall,
    Between,
    Case,
    In,
    IsNull,
    Null,
    Cast,
    Column,
    Row,
    MemberAccess,
}

dyn_clone::clone_trait_object!(Expr);

pub trait Expr: DynClone {
    fn kind(&self) -> ExprKind;
    fn expr_type(&self) -> Box<dyn DataType>;
    fn as_any(&self) -> &dyn Any;
}

impl dyn Expr {
    pub fn is_const(&self) -> bool {
        matches!(
            self.kind(),
            ExprKind::Number | ExprKind::Boolean | ExprKind::String | ExprKind::Null
        )
    }
}

#[derive(Clone)]
pub struct AssignmentExpr {
    pub symbol: String,
    pub value: Box<dyn Expr>,
}

impl Expr for AssignmentExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::Assignment
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        self.value.expr_type()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct StringExpr {
    pub value: String,
}

impl Expr for StringExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::String
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        Box::new(TextType)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(PartialEq, Clone)]
pub enum SymbolFlag {
    AggregationReference,
    WindowReference,
    None,
}

#[derive(Clone)]
pub struct SymbolExpr {
    pub value: String,
    pub expr_type: Box<dyn DataType>,
    pub flag: SymbolFlag,
}

impl Expr for SymbolExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::Symbol
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        self.expr_type.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct ArrayExpr {
    pub values: Vec<Box<dyn Expr>>,
    pub element_type: Box<dyn DataType>,
}

impl Expr for ArrayExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::Array
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        Box::new(ArrayType::new(self.element_type.clone()))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct GlobalVariableExpr {
    pub name: String,
    pub result_type: Box<dyn DataType>,
}

impl Expr for GlobalVariableExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::GlobalVariable
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        self.result_type.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, PartialEq)]
pub enum Number {
    Int(i64),
    Float(f64),
}

#[derive(Clone)]
pub struct NumberExpr {
    pub value: Number,
}

impl Expr for NumberExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::Number
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        match self.value {
            Number::Int(_) => Box::new(IntType),
            Number::Float(_) => Box::new(FloatType),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct IntervalExpr {
    pub interval: Interval,
}

impl IntervalExpr {
    pub fn new(interval: Interval) -> Self {
        IntervalExpr { interval }
    }
}

impl Expr for IntervalExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::Interval
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        Box::new(IntervalType)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct BooleanExpr {
    pub is_true: bool,
}

impl Expr for BooleanExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::Boolean
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        Box::new(BoolType)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct UnaryExpr {
    pub right: Box<dyn Expr>,
    pub operator: PrefixUnaryOperator,
    pub result_type: Box<dyn DataType>,
}

impl Expr for UnaryExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::PrefixUnary
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        self.result_type.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct IndexExpr {
    pub collection: Box<dyn Expr>,
    pub element_type: Box<dyn DataType>,
    pub index: Box<dyn Expr>,
    pub result_type: Box<dyn DataType>,
}

impl Expr for IndexExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::Index
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        self.result_type.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct SliceExpr {
    pub collection: Box<dyn Expr>,
    pub start: Option<Box<dyn Expr>>,
    pub end: Option<Box<dyn Expr>>,
    pub result_type: Box<dyn DataType>,
}

impl Expr for SliceExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::Slice
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        self.result_type.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct ArithmeticExpr {
    pub left: Box<dyn Expr>,
    pub operator: ArithmeticOperator,
    pub right: Box<dyn Expr>,
    pub result_type: Box<dyn DataType>,
}

impl Expr for ArithmeticExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::Arithmetic
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        self.result_type.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct ComparisonExpr {
    pub left: Box<dyn Expr>,
    pub operator: ComparisonOperator,
    pub right: Box<dyn Expr>,
}

impl Expr for ComparisonExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::Comparison
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        if self.operator == ComparisonOperator::NullSafeEqual {
            Box::new(IntType)
        } else {
            Box::new(BoolType)
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct GroupComparisonExpr {
    pub left: Box<dyn Expr>,
    pub comparison_operator: ComparisonOperator,
    pub group_operator: GroupComparisonOperator,
    pub right: Box<dyn Expr>,
}

impl Expr for GroupComparisonExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::GroupComparison
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        if self.comparison_operator == ComparisonOperator::NullSafeEqual {
            Box::new(IntType)
        } else {
            Box::new(BoolType)
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct ContainsExpr {
    pub left: Box<dyn Expr>,
    pub right: Box<dyn Expr>,
}

impl Expr for ContainsExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::Contains
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        Box::new(BoolType)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct ContainedByExpr {
    pub left: Box<dyn Expr>,
    pub right: Box<dyn Expr>,
}

impl Expr for ContainedByExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::ContainedBy
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        Box::new(BoolType)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct LikeExpr {
    pub input: Box<dyn Expr>,
    pub pattern: Box<dyn Expr>,
}

impl Expr for LikeExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::Like
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        Box::new(BoolType)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct RegexExpr {
    pub input: Box<dyn Expr>,
    pub pattern: Box<dyn Expr>,
}

impl Expr for RegexExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::Regex
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        Box::new(BoolType)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct GlobExpr {
    pub input: Box<dyn Expr>,
    pub pattern: Box<dyn Expr>,
}

impl Expr for GlobExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::Glob
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        Box::new(BoolType)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct LogicalExpr {
    pub left: Box<dyn Expr>,
    pub operator: BinaryLogicalOperator,
    pub right: Box<dyn Expr>,
}

impl Expr for LogicalExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::Logical
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        let lhs_type = self.left.expr_type();
        let rhs_type = &self.right.expr_type();
        match self.operator {
            BinaryLogicalOperator::Or => lhs_type.logical_or_op_result_type(rhs_type),
            BinaryLogicalOperator::And => lhs_type.logical_and_op_result_type(rhs_type),
            BinaryLogicalOperator::Xor => lhs_type.logical_xor_op_result_type(rhs_type),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct BitwiseExpr {
    pub left: Box<dyn Expr>,
    pub operator: BinaryBitwiseOperator,
    pub right: Box<dyn Expr>,
    pub result_type: Box<dyn DataType>,
}

impl Expr for BitwiseExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::Bitwise
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        self.result_type.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct CallExpr {
    pub function_name: String,
    pub arguments: Vec<Box<dyn Expr>>,
    pub return_type: Box<dyn DataType>,
}

impl Expr for CallExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::Call
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        self.return_type.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct BenchmarkCallExpr {
    pub expression: Box<dyn Expr>,
    pub count: Box<dyn Expr>,
}

impl Expr for BenchmarkCallExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::BenchmarkCall
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        Box::new(IntType)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// ASYMMETRIC is default. ASYMMETRIC is noise, it's current behavior.
/// If b <= c, then SYMMETRIC and ASYMMETRIC work the same way.
/// If b > c, then SYMMETRIC in effect reverses the operands.
#[derive(PartialEq, Clone)]
pub enum BetweenKind {
    Symmetric,
    Asymmetric,
}

#[derive(Clone)]
pub struct BetweenExpr {
    pub value: Box<dyn Expr>,
    pub range_start: Box<dyn Expr>,
    pub range_end: Box<dyn Expr>,
    pub kind: BetweenKind,
}

impl Expr for BetweenExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::Between
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        Box::new(BoolType)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct CaseExpr {
    pub conditions: Vec<Box<dyn Expr>>,
    pub values: Vec<Box<dyn Expr>>,
    pub default_value: Option<Box<dyn Expr>>,
    pub values_type: Box<dyn DataType>,
}

impl Expr for CaseExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::Case
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        self.values_type.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct InExpr {
    pub argument: Box<dyn Expr>,
    pub values: Vec<Box<dyn Expr>>,
    pub values_type: Box<dyn DataType>,
    pub has_not_keyword: bool,
}

impl Expr for InExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::In
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        self.values_type.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct IsNullExpr {
    pub argument: Box<dyn Expr>,
    pub has_not: bool,
}

impl Expr for IsNullExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::IsNull
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        Box::new(BoolType)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct NullExpr;

impl Expr for NullExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::Null
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct CastExpr {
    pub value: Box<dyn Expr>,
    pub result_type: Box<dyn DataType>,
}

impl Expr for CastExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::Cast
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        self.result_type.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct ColumnExpr {
    pub expr: Box<dyn Expr>,
}

impl Expr for ColumnExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::Column
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        self.expr.expr_type().clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct RowExpr {
    pub exprs: Vec<Box<dyn Expr>>,
    pub row_type: RowType,
}

impl Expr for RowExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::Row
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        Box::new(self.row_type.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct MemberAccessExpr {
    pub composite: Box<dyn Expr>,
    pub member_name: String,
    pub member_type: Box<dyn DataType>,
}

impl Expr for MemberAccessExpr {
    fn kind(&self) -> ExprKind {
        ExprKind::MemberAccess
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        self.member_type.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
