use std::any::Any;

use crate::environment::Environment;
use crate::function::PROTOTYPES;
use crate::types::DataType;
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
    fn kind(&self) -> ExpressionKind;
    fn expr_type(&self, scope: &Environment) -> DataType;
    fn as_any(&self) -> &dyn Any;
}

impl dyn Expression {
    pub fn is_const(&self) -> bool {
        matches!(
            self.kind(),
            ExpressionKind::Number | ExpressionKind::Boolean | ExpressionKind::String
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

    fn expr_type(&self, scope: &Environment) -> DataType {
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
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::String
    }

    fn expr_type(&self, _scope: &Environment) -> DataType {
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
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Symbol
    }

    fn expr_type(&self, scope: &Environment) -> DataType {
        // Search in symbol table
        if scope.contains(&self.value) {
            return scope.scopes[self.value.as_str()].clone();
        }

        // Search in static table fields types
        if scope
            .schema
            .tables_fields_types
            .contains_key(&self.value.as_str())
        {
            return scope.schema.tables_fields_types[&self.value.as_str()].clone();
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
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::GlobalVariable
    }

    fn expr_type(&self, scope: &Environment) -> DataType {
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
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Number
    }

    fn expr_type(&self, _scope: &Environment) -> DataType {
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
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Boolean
    }

    fn expr_type(&self, _scope: &Environment) -> DataType {
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
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::PrefixUnary
    }

    fn expr_type(&self, _scope: &Environment) -> DataType {
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
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Arithmetic
    }

    fn expr_type(&self, scope: &Environment) -> DataType {
        if self.left.expr_type(scope).is_int() && self.right.expr_type(scope).is_int() {
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
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Comparison
    }

    fn expr_type(&self, _scope: &Environment) -> DataType {
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
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Like
    }

    fn expr_type(&self, _scope: &Environment) -> DataType {
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
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Glob
    }

    fn expr_type(&self, _scope: &Environment) -> DataType {
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
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Logical
    }

    fn expr_type(&self, _scope: &Environment) -> DataType {
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
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Bitwise
    }

    fn expr_type(&self, _scope: &Environment) -> DataType {
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
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Call
    }

    fn expr_type(&self, _scope: &Environment) -> DataType {
        let prototype = PROTOTYPES.get(&self.function_name.as_str()).unwrap();
        prototype.return_type.clone()
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

    fn expr_type(&self, _scope: &Environment) -> DataType {
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
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Case
    }

    fn expr_type(&self, _scope: &Environment) -> DataType {
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
    pub has_not_keyword: bool,
}

impl Expression for InExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::In
    }

    fn expr_type(&self, _scope: &Environment) -> DataType {
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

    fn expr_type(&self, _scope: &Environment) -> DataType {
        DataType::Boolean
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

    fn expr_type(&self, _scope: &Environment) -> DataType {
        DataType::Null
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
