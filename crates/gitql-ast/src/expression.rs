use std::any::Any;

use crate::enviroment::Enviroment;
use crate::function::PROTOTYPES;
use crate::types::{DataType, TABLES_FIELDS_TYPES};
use crate::value::Value;

#[derive(PartialEq)]
pub enum ExpressionKind {
    Assignment,
    String,
    Symbol,
    GlobalVariable,
    Number,
    Boolean,
    PrefixUnary,
    Arithmetic,
    Comparison,
    Like,
    Glob,
    Logical,
    Bitwise,
    Call,
    Between,
    Case,
    In,
    IsNull,
    Null,
}

pub trait Expression {
    fn expression_kind(&self) -> ExpressionKind;
    fn expr_type(&self, scope: &Enviroment) -> DataType;
    fn as_any(&self) -> &dyn Any;
}

impl dyn Expression {
    pub fn is_const(&self) -> bool {
        matches!(
            self.expression_kind(),
            ExpressionKind::Number | ExpressionKind::Boolean | ExpressionKind::String
        )
    }
}

pub struct AssignmentExpression {
    pub symbol: String,
    pub value: Box<dyn Expression>,
}

impl Expression for AssignmentExpression {
    fn expression_kind(&self) -> ExpressionKind {
        ExpressionKind::Assignment
    }

    fn expr_type(&self, scope: &Enviroment) -> DataType {
        self.value.expr_type(scope)
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
}

pub struct StringExpression {
    pub value: String,
    pub value_type: StringValueType,
}

impl Expression for StringExpression {
    fn expression_kind(&self) -> ExpressionKind {
        ExpressionKind::String
    }

    fn expr_type(&self, _scope: &Enviroment) -> DataType {
        match self.value_type {
            StringValueType::Text => DataType::Text,
            StringValueType::Time => DataType::Time,
            StringValueType::Date => DataType::Date,
            StringValueType::DateTime => DataType::DateTime,
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct SymbolExpression {
    pub value: String,
}

impl Expression for SymbolExpression {
    fn expression_kind(&self) -> ExpressionKind {
        ExpressionKind::Symbol
    }

    fn expr_type(&self, scope: &Enviroment) -> DataType {
        // Search in symbol table
        if scope.contains(&self.value) {
            return scope.scopes[self.value.as_str()].clone();
        }

        // Search in static table fields types
        if TABLES_FIELDS_TYPES.contains_key(&self.value.as_str()) {
            return TABLES_FIELDS_TYPES[&self.value.as_str()].clone();
        }

        DataType::Undefined
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct GlobalVariableExpression {
    pub name: String,
}

impl Expression for GlobalVariableExpression {
    fn expression_kind(&self) -> ExpressionKind {
        ExpressionKind::GlobalVariable
    }

    fn expr_type(&self, scope: &Enviroment) -> DataType {
        if scope.globals_types.contains_key(&self.name) {
            return scope.globals_types[self.name.as_str()].clone();
        }
        DataType::Undefined
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct NumberExpression {
    pub value: Value,
}

impl Expression for NumberExpression {
    fn expression_kind(&self) -> ExpressionKind {
        ExpressionKind::Number
    }

    fn expr_type(&self, _scope: &Enviroment) -> DataType {
        self.value.data_type()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct BooleanExpression {
    pub is_true: bool,
}

impl Expression for BooleanExpression {
    fn expression_kind(&self) -> ExpressionKind {
        ExpressionKind::Boolean
    }

    fn expr_type(&self, _scope: &Enviroment) -> DataType {
        DataType::Boolean
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(PartialEq)]
pub enum PrefixUnaryOperator {
    Minus,
    Bang,
}

pub struct PrefixUnary {
    pub right: Box<dyn Expression>,
    pub op: PrefixUnaryOperator,
}

impl Expression for PrefixUnary {
    fn expression_kind(&self) -> ExpressionKind {
        ExpressionKind::PrefixUnary
    }

    fn expr_type(&self, _scope: &Enviroment) -> DataType {
        if self.op == PrefixUnaryOperator::Bang {
            DataType::Boolean
        } else {
            DataType::Integer
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(PartialEq)]
pub enum ArithmeticOperator {
    Plus,
    Minus,
    Star,
    Slash,
    Modulus,
}

pub struct ArithmeticExpression {
    pub left: Box<dyn Expression>,
    pub operator: ArithmeticOperator,
    pub right: Box<dyn Expression>,
}

impl Expression for ArithmeticExpression {
    fn expression_kind(&self) -> ExpressionKind {
        ExpressionKind::Arithmetic
    }

    fn expr_type(&self, scope: &Enviroment) -> DataType {
        let lhs = self.left.expr_type(scope);
        let rhs = self.left.expr_type(scope);

        if lhs.is_int() && rhs.is_int() {
            return DataType::Integer;
        }

        DataType::Float
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(PartialEq)]
pub enum ComparisonOperator {
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Equal,
    NotEqual,
    NullSafeEqual,
}

pub struct ComparisonExpression {
    pub left: Box<dyn Expression>,
    pub operator: ComparisonOperator,
    pub right: Box<dyn Expression>,
}

impl Expression for ComparisonExpression {
    fn expression_kind(&self) -> ExpressionKind {
        ExpressionKind::Comparison
    }

    fn expr_type(&self, _scope: &Enviroment) -> DataType {
        if self.operator == ComparisonOperator::NullSafeEqual {
            DataType::Integer
        } else {
            DataType::Boolean
        }
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
    fn expression_kind(&self) -> ExpressionKind {
        ExpressionKind::Like
    }

    fn expr_type(&self, _scope: &Enviroment) -> DataType {
        DataType::Boolean
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
    fn expression_kind(&self) -> ExpressionKind {
        ExpressionKind::Glob
    }

    fn expr_type(&self, _scope: &Enviroment) -> DataType {
        DataType::Boolean
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(PartialEq)]
pub enum LogicalOperator {
    Or,
    And,
    Xor,
}

pub struct LogicalExpression {
    pub left: Box<dyn Expression>,
    pub operator: LogicalOperator,
    pub right: Box<dyn Expression>,
}

impl Expression for LogicalExpression {
    fn expression_kind(&self) -> ExpressionKind {
        ExpressionKind::Logical
    }

    fn expr_type(&self, _scope: &Enviroment) -> DataType {
        DataType::Boolean
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(PartialEq)]
pub enum BitwiseOperator {
    Or,
    And,
    RightShift,
    LeftShift,
}

pub struct BitwiseExpression {
    pub left: Box<dyn Expression>,
    pub operator: BitwiseOperator,
    pub right: Box<dyn Expression>,
}

impl Expression for BitwiseExpression {
    fn expression_kind(&self) -> ExpressionKind {
        ExpressionKind::Bitwise
    }

    fn expr_type(&self, _scope: &Enviroment) -> DataType {
        DataType::Integer
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct CallExpression {
    pub function_name: String,
    pub arguments: Vec<Box<dyn Expression>>,
    pub is_aggregation: bool,
}

impl Expression for CallExpression {
    fn expression_kind(&self) -> ExpressionKind {
        ExpressionKind::Call
    }

    fn expr_type(&self, _scope: &Enviroment) -> DataType {
        let prototype = PROTOTYPES.get(&self.function_name.as_str()).unwrap();
        prototype.result.clone()
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
    fn expression_kind(&self) -> ExpressionKind {
        ExpressionKind::Between
    }

    fn expr_type(&self, _scope: &Enviroment) -> DataType {
        DataType::Boolean
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct CaseExpression {
    pub conditions: Vec<Box<dyn Expression>>,
    pub values: Vec<Box<dyn Expression>>,
    pub default_value: Option<Box<dyn Expression>>,
    pub values_type: DataType,
}

impl Expression for CaseExpression {
    fn expression_kind(&self) -> ExpressionKind {
        ExpressionKind::Case
    }

    fn expr_type(&self, _scope: &Enviroment) -> DataType {
        self.values_type.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct InExpression {
    pub argument: Box<dyn Expression>,
    pub values: Vec<Box<dyn Expression>>,
    pub values_type: DataType,
}

impl Expression for InExpression {
    fn expression_kind(&self) -> ExpressionKind {
        ExpressionKind::In
    }

    fn expr_type(&self, _scope: &Enviroment) -> DataType {
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
    fn expression_kind(&self) -> ExpressionKind {
        ExpressionKind::IsNull
    }

    fn expr_type(&self, _scope: &Enviroment) -> DataType {
        DataType::Boolean
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct NullExpression {}

impl Expression for NullExpression {
    fn expression_kind(&self) -> ExpressionKind {
        ExpressionKind::Null
    }

    fn expr_type(&self, _scope: &Enviroment) -> DataType {
        DataType::Null
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
