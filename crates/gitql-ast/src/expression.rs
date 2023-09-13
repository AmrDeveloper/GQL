use std::any::Any;

use crate::function::PROTOTYPES;
use crate::scope::Scope;
use crate::types::{DataType, TABLES_FIELDS_TYPES};
use crate::value::Value;

pub enum ExpressionKind {
    String,
    Symbol,
    Number,
    Boolean,
    PrefixUnary,
    Arithmetic,
    Comparison,
    Check,
    Logical,
    Bitwise,
    Call,
    Between,
    Case,
    In,
}

pub trait Expression {
    fn get_expression_kind(&self) -> ExpressionKind;
    fn expr_type(&self, scope: &Scope) -> DataType;
    fn as_any(&self) -> &dyn Any;
}

pub struct StringExpression {
    pub value: String,
}

impl Expression for StringExpression {
    fn get_expression_kind(&self) -> ExpressionKind {
        ExpressionKind::String
    }

    fn expr_type(&self, _scope: &Scope) -> DataType {
        return DataType::Text;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct SymbolExpression {
    pub value: String,
}

impl Expression for SymbolExpression {
    fn get_expression_kind(&self) -> ExpressionKind {
        ExpressionKind::Symbol
    }

    fn expr_type(&self, scope: &Scope) -> DataType {
        // Search in symbol table
        if scope.contains(&self.value) {
            return scope.env[self.value.as_str()].clone();
        }

        // Search in static table fields types
        if TABLES_FIELDS_TYPES.contains_key(&self.value.as_str()) {
            return TABLES_FIELDS_TYPES[&self.value.as_str()].clone();
        }

        return DataType::Undefined;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct NumberExpression {
    pub value: Value,
}

impl Expression for NumberExpression {
    fn get_expression_kind(&self) -> ExpressionKind {
        ExpressionKind::Number
    }

    fn expr_type(&self, _scope: &Scope) -> DataType {
        return self.value.data_type();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct BooleanExpression {
    pub is_true: bool,
}

impl Expression for BooleanExpression {
    fn get_expression_kind(&self) -> ExpressionKind {
        ExpressionKind::Boolean
    }

    fn expr_type(&self, _scope: &Scope) -> DataType {
        return DataType::Boolean;
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
    fn get_expression_kind(&self) -> ExpressionKind {
        ExpressionKind::PrefixUnary
    }

    fn expr_type(&self, _scope: &Scope) -> DataType {
        return if self.op == PrefixUnaryOperator::Bang {
            DataType::Boolean
        } else {
            DataType::Integer
        };
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
    fn get_expression_kind(&self) -> ExpressionKind {
        ExpressionKind::Arithmetic
    }

    fn expr_type(&self, _scope: &Scope) -> DataType {
        return DataType::Integer;
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
}

pub struct ComparisonExpression {
    pub left: Box<dyn Expression>,
    pub operator: ComparisonOperator,
    pub right: Box<dyn Expression>,
}

impl Expression for ComparisonExpression {
    fn get_expression_kind(&self) -> ExpressionKind {
        ExpressionKind::Comparison
    }

    fn expr_type(&self, _scope: &Scope) -> DataType {
        return DataType::Boolean;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(PartialEq)]
pub enum CheckOperator {
    Contains,
    StartsWith,
    EndsWith,
    Matches,
}

pub struct CheckExpression {
    pub left: Box<dyn Expression>,
    pub operator: CheckOperator,
    pub right: Box<dyn Expression>,
}

impl Expression for CheckExpression {
    fn get_expression_kind(&self) -> ExpressionKind {
        ExpressionKind::Check
    }

    fn expr_type(&self, _scope: &Scope) -> DataType {
        return DataType::Boolean;
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
    fn get_expression_kind(&self) -> ExpressionKind {
        ExpressionKind::Logical
    }

    fn expr_type(&self, _scope: &Scope) -> DataType {
        return DataType::Boolean;
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
    fn get_expression_kind(&self) -> ExpressionKind {
        ExpressionKind::Bitwise
    }

    fn expr_type(&self, _scope: &Scope) -> DataType {
        return DataType::Integer;
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
    fn get_expression_kind(&self) -> ExpressionKind {
        ExpressionKind::Call
    }

    fn expr_type(&self, _scope: &Scope) -> DataType {
        let prototype = PROTOTYPES.get(&self.function_name.as_str()).unwrap();
        return prototype.result.clone();
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
    fn get_expression_kind(&self) -> ExpressionKind {
        ExpressionKind::Between
    }

    fn expr_type(&self, _scope: &Scope) -> DataType {
        return DataType::Boolean;
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
    fn get_expression_kind(&self) -> ExpressionKind {
        ExpressionKind::Case
    }

    fn expr_type(&self, _scope: &Scope) -> DataType {
        return self.values_type.clone();
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
    fn get_expression_kind(&self) -> ExpressionKind {
        ExpressionKind::In
    }

    fn expr_type(&self, _scope: &Scope) -> DataType {
        return self.values_type.clone();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
