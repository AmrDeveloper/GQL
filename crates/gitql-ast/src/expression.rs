use std::any::Any;

use super::types::array::ArrayType;
use super::types::base::DataType;
use super::types::boolean::BoolType;
use super::types::date::DateType;
use super::types::integer::IntType;
use super::types::null::NullType;
use super::types::text::TextType;
use super::types::time::TimeType;

use crate::operator::ArithmeticOperator;
use crate::operator::BinaryBitwiseOperator;
use crate::operator::BinaryLogicalOperator;
use crate::operator::ComparisonOperator;
use crate::operator::ContainsOperator;
use crate::operator::PrefixUnaryOperator;
use crate::types::float::FloatType;

#[derive(PartialEq)]
pub enum ExpressionKind {
    Assignment,
    String,
    Symbol,
    Array,
    GlobalVariable,
    Number,
    Boolean,
    PrefixUnary,
    Index,
    Slice,
    Arithmetic,
    Comparison,
    Contains,
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
}

pub trait Expression {
    fn kind(&self) -> ExpressionKind;
    fn expr_type(&self) -> Box<dyn DataType>;
    fn as_any(&self) -> &dyn Any;
}

impl dyn Expression {
    pub fn is_const(&self) -> bool {
        matches!(
            self.kind(),
            ExpressionKind::Number
                | ExpressionKind::Boolean
                | ExpressionKind::String
                | ExpressionKind::Null
        )
    }
}

pub struct AssignmentExpression {
    pub symbol: String,
    pub value: Box<dyn Expression>,
}

impl Expression for AssignmentExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Assignment
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        self.value.expr_type()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub enum StringValueType {
    Text,
    Time,
    Date,
    DateTime,
    Boolean,
}

pub struct StringExpression {
    pub value: String,
    pub value_type: StringValueType,
}

impl Expression for StringExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::String
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        match self.value_type {
            StringValueType::Text => Box::new(TextType),
            StringValueType::Time => Box::new(TimeType),
            StringValueType::Date => Box::new(DateType),
            StringValueType::DateTime => Box::new(TimeType),
            StringValueType::Boolean => Box::new(BoolType),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct SymbolExpression {
    pub value: String,
    pub result_type: Box<dyn DataType>,
}

impl Expression for SymbolExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Symbol
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        self.result_type.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ArrayExpression {
    pub values: Vec<Box<dyn Expression>>,
    pub element_type: Box<dyn DataType>,
}

impl Expression for ArrayExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Array
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        Box::new(ArrayType {
            base: self.element_type.clone(),
        })
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct GlobalVariableExpression {
    pub name: String,
    pub result_type: Box<dyn DataType>,
}

impl Expression for GlobalVariableExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::GlobalVariable
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        self.result_type.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub enum Number {
    Int(i64),
    Float(f64),
}

pub struct NumberExpression {
    pub value: Number,
}

impl Expression for NumberExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Number
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

pub struct BooleanExpression {
    pub is_true: bool,
}

impl Expression for BooleanExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Boolean
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        Box::new(BoolType)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct UnaryExpression {
    pub right: Box<dyn Expression>,
    pub operator: PrefixUnaryOperator,
    pub result_type: Box<dyn DataType>,
}

impl Expression for UnaryExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::PrefixUnary
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        self.result_type.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct IndexExpression {
    pub collection: Box<dyn Expression>,
    pub element_type: Box<dyn DataType>,
    pub index: Box<dyn Expression>,
    pub result_type: Box<dyn DataType>,
}

impl Expression for IndexExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Index
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        self.result_type.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct SliceExpression {
    pub collection: Box<dyn Expression>,
    pub start: Option<Box<dyn Expression>>,
    pub end: Option<Box<dyn Expression>>,
    pub result_type: Box<dyn DataType>,
}

impl Expression for SliceExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Slice
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        self.result_type.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ArithmeticExpression {
    pub left: Box<dyn Expression>,
    pub operator: ArithmeticOperator,
    pub right: Box<dyn Expression>,
    pub result_type: Box<dyn DataType>,
}

impl Expression for ArithmeticExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Arithmetic
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        self.result_type.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ComparisonExpression {
    pub left: Box<dyn Expression>,
    pub operator: ComparisonOperator,
    pub right: Box<dyn Expression>,
}

impl Expression for ComparisonExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Comparison
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

pub struct ContainsExpression {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
    pub operator: ContainsOperator,
}

impl Expression for ContainsExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Contains
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        Box::new(BoolType)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct LikeExpression {
    pub input: Box<dyn Expression>,
    pub pattern: Box<dyn Expression>,
}

impl Expression for LikeExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Like
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        Box::new(BoolType)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct RegexExpression {
    pub input: Box<dyn Expression>,
    pub pattern: Box<dyn Expression>,
}

impl Expression for RegexExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Regex
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        Box::new(BoolType)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct GlobExpression {
    pub input: Box<dyn Expression>,
    pub pattern: Box<dyn Expression>,
}

impl Expression for GlobExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Glob
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        Box::new(BoolType)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct LogicalExpression {
    pub left: Box<dyn Expression>,
    pub operator: BinaryLogicalOperator,
    pub right: Box<dyn Expression>,
}

impl Expression for LogicalExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Logical
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        Box::new(BoolType)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct BitwiseExpression {
    pub left: Box<dyn Expression>,
    pub operator: BinaryBitwiseOperator,
    pub right: Box<dyn Expression>,
    pub result_type: Box<dyn DataType>,
}

impl Expression for BitwiseExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Bitwise
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        self.result_type.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct CallExpression {
    pub function_name: String,
    pub arguments: Vec<Box<dyn Expression>>,
    pub return_type: Box<dyn DataType>,
}

impl Expression for CallExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Call
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        self.return_type.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct BenchmarkExpression {
    pub expression: Box<dyn Expression>,
    pub count: Box<dyn Expression>,
}

impl Expression for BenchmarkExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::BenchmarkCall
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        Box::new(IntType)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct BetweenExpression {
    pub value: Box<dyn Expression>,
    pub range_start: Box<dyn Expression>,
    pub range_end: Box<dyn Expression>,
}

impl Expression for BetweenExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Between
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        Box::new(BoolType)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct CaseExpression {
    pub conditions: Vec<Box<dyn Expression>>,
    pub values: Vec<Box<dyn Expression>>,
    pub default_value: Option<Box<dyn Expression>>,
    pub values_type: Box<dyn DataType>,
}

impl Expression for CaseExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Case
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        self.values_type.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct InExpression {
    pub argument: Box<dyn Expression>,
    pub values: Vec<Box<dyn Expression>>,
    pub values_type: Box<dyn DataType>,
    pub has_not_keyword: bool,
}

impl Expression for InExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::In
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        self.values_type.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct IsNullExpression {
    pub argument: Box<dyn Expression>,
    pub has_not: bool,
}

impl Expression for IsNullExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::IsNull
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        Box::new(BoolType)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct NullExpression {}

impl Expression for NullExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Null
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        Box::new(NullType)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct CastExpression {
    pub value: Box<dyn Expression>,
    pub result_type: Box<dyn DataType>,
}

impl Expression for CastExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Cast
    }

    fn expr_type(&self) -> Box<dyn DataType> {
        self.result_type.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
