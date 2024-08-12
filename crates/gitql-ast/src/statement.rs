use std::any::Any;
use std::collections::HashMap;

use crate::expression::Expression;

pub enum StatementKind {
    Do,
    Select,
    Where,
    Having,
    Limit,
    Offset,
    OrderBy,
    GroupBy,
    AggregateFunction,
    GlobalVariable,
    Into,
}

pub trait Statement {
    fn kind(&self) -> StatementKind;
    fn as_any(&self) -> &dyn Any;
}

pub enum Query {
    Do(DoStatement),
    Select(GQLQuery),
    GlobalVariableDeclaration(GlobalVariableStatement),
    Describe(DescribeStatement),
    ShowTables,
}

pub struct GQLQuery {
    pub statements: HashMap<&'static str, Box<dyn Statement>>,
    pub alias_table: HashMap<String, String>,
    pub has_aggregation_function: bool,
    pub has_group_by_statement: bool,
    pub hidden_selections: HashMap<String, Vec<String>>,
}

pub struct DoStatement {
    pub expression: Box<dyn Expression>,
}

impl Statement for DoStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn kind(&self) -> StatementKind {
        StatementKind::Do
    }
}

pub enum Distinct {
    None,
    DistinctAll,
    DistinctOn(Vec<String>),
}

pub struct TableSelection {
    pub table_name: String,
    pub columns_names: Vec<String>,
}

#[derive(PartialEq)]
pub enum JoinKind {
    Cross,
    Inner,
    Left,
    Right,
    Default,
}

pub enum JoinOperand {
    /// Used when JOIN is used first time on query, X JOIN Y,
    OuterAndInner(String, String),
    /// Used for JOIN that used afrer first time, JOIN Z
    Inner(String),
}

pub struct Join {
    pub operand: JoinOperand,
    pub kind: JoinKind,
    pub predicate: Option<Box<dyn Expression>>,
}

pub struct SelectStatement {
    pub table_selections: Vec<TableSelection>,
    pub joins: Vec<Join>,
    pub selected_expr_titles: Vec<String>,
    pub selected_expr: Vec<Box<dyn Expression>>,
    pub distinct: Distinct,
}

impl Statement for SelectStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn kind(&self) -> StatementKind {
        StatementKind::Select
    }
}

pub struct WhereStatement {
    pub condition: Box<dyn Expression>,
}

impl Statement for WhereStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn kind(&self) -> StatementKind {
        StatementKind::Where
    }
}

pub struct HavingStatement {
    pub condition: Box<dyn Expression>,
}

impl Statement for HavingStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn kind(&self) -> StatementKind {
        StatementKind::Having
    }
}

pub struct LimitStatement {
    pub count: usize,
}

impl Statement for LimitStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn kind(&self) -> StatementKind {
        StatementKind::Limit
    }
}

pub struct OffsetStatement {
    pub count: usize,
}

impl Statement for OffsetStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn kind(&self) -> StatementKind {
        StatementKind::Offset
    }
}

#[derive(PartialEq)]
pub enum SortingOrder {
    Ascending,
    Descending,
}

pub struct OrderByStatement {
    pub arguments: Vec<Box<dyn Expression>>,
    pub sorting_orders: Vec<SortingOrder>,
}

impl Statement for OrderByStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn kind(&self) -> StatementKind {
        StatementKind::OrderBy
    }
}

pub struct GroupByStatement {
    pub values: Vec<Box<dyn Expression>>,
}

impl Statement for GroupByStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn kind(&self) -> StatementKind {
        StatementKind::GroupBy
    }
}

pub enum AggregateValue {
    Expression(Box<dyn Expression>),
    Function(String, Vec<Box<dyn Expression>>),
}

pub struct AggregationsStatement {
    pub aggregations: HashMap<String, AggregateValue>,
}

impl Statement for AggregationsStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn kind(&self) -> StatementKind {
        StatementKind::AggregateFunction
    }
}

pub struct GlobalVariableStatement {
    pub name: String,
    pub value: Box<dyn Expression>,
}

impl Statement for GlobalVariableStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn kind(&self) -> StatementKind {
        StatementKind::GlobalVariable
    }
}

pub struct IntoStatement {
    pub file_path: String,
    pub lines_terminated: String,
    pub fields_terminated: String,
    pub enclosed: String,
}

impl Statement for IntoStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn kind(&self) -> StatementKind {
        StatementKind::Into
    }
}

#[derive(Debug)]
pub struct DescribeStatement {
    pub table_name: String,
}
