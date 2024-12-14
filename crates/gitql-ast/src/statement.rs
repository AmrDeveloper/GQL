use std::any::Any;
use std::collections::HashMap;

use dyn_clone::DynClone;

use crate::expression::Expr;

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
    WindowFunction,
    GlobalVariable,
    Into,
}

dyn_clone::clone_trait_object!(Statement);

pub trait Statement: DynClone {
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

#[derive(Clone)]
pub struct DoStatement {
    pub expression: Box<dyn Expr>,
}

impl Statement for DoStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn kind(&self) -> StatementKind {
        StatementKind::Do
    }
}

#[derive(Clone)]
pub enum Distinct {
    None,
    DistinctAll,
    DistinctOn(Vec<String>),
}

#[derive(Clone)]
pub struct TableSelection {
    pub table_name: String,
    pub columns_names: Vec<String>,
}

#[derive(Clone, PartialEq)]
pub enum JoinKind {
    Cross,
    Inner,
    Left,
    Right,
    Default,
}

#[derive(Clone)]
pub enum JoinOperand {
    /// Used when JOIN is used first time on query, X JOIN Y,
    OuterAndInner(String, String),
    /// Used for JOIN that used after first time, JOIN Z
    Inner(String),
}

#[derive(Clone)]
pub struct Join {
    pub operand: JoinOperand,
    pub kind: JoinKind,
    pub predicate: Option<Box<dyn Expr>>,
}

#[derive(Clone)]
pub struct SelectStatement {
    pub table_selections: Vec<TableSelection>,
    pub joins: Vec<Join>,
    pub selected_expr_titles: Vec<String>,
    pub selected_expr: Vec<Box<dyn Expr>>,
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

#[derive(Clone)]
pub struct WhereStatement {
    pub condition: Box<dyn Expr>,
}

impl Statement for WhereStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn kind(&self) -> StatementKind {
        StatementKind::Where
    }
}

#[derive(Clone)]
pub struct HavingStatement {
    pub condition: Box<dyn Expr>,
}

impl Statement for HavingStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn kind(&self) -> StatementKind {
        StatementKind::Having
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone, PartialEq)]
pub enum SortingOrder {
    Ascending,
    Descending,
}

#[derive(Clone, PartialEq)]
pub enum NullsOrderPolicy {
    NullsFirst,
    NullsLast,
}

#[derive(Clone)]
pub struct OrderByStatement {
    pub arguments: Vec<Box<dyn Expr>>,
    pub sorting_orders: Vec<SortingOrder>,
    pub nulls_order_policies: Vec<NullsOrderPolicy>,
}

impl Statement for OrderByStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn kind(&self) -> StatementKind {
        StatementKind::OrderBy
    }
}

#[derive(Clone)]
pub struct GroupByStatement {
    pub values: Vec<Box<dyn Expr>>,
    pub has_with_roll_up: bool,
}

impl Statement for GroupByStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn kind(&self) -> StatementKind {
        StatementKind::GroupBy
    }
}

#[derive(Clone)]
pub struct WindowPartitioningClause {
    pub expr: Box<dyn Expr>,
}

#[derive(Clone)]
pub struct WindowOrderingClause {
    pub order_by: OrderByStatement,
}

#[derive(Clone)]
pub struct WindowDefinition {
    pub name: Option<String>,
    pub partitioning_clause: Option<WindowPartitioningClause>,
    pub ordering_clause: Option<WindowOrderingClause>,
}

#[derive(Clone)]
pub enum WindowFunctionKind {
    AggregatedWindowFunction,
    PureWindowFunction,
}

#[derive(Clone)]
pub struct WindowFunction {
    pub function_name: String,
    pub arguments: Vec<Box<dyn Expr>>,
    pub window_definition: WindowDefinition,
    pub kind: WindowFunctionKind,
}

#[derive(Clone)]
pub struct WindowFunctionsStatement {
    pub functions: HashMap<String, WindowFunction>,
}

impl Statement for WindowFunctionsStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn kind(&self) -> StatementKind {
        StatementKind::WindowFunction
    }
}

#[derive(Clone)]
pub enum AggregateValue {
    Expression(Box<dyn Expr>),
    Function(String, Vec<Box<dyn Expr>>),
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct GlobalVariableStatement {
    pub name: String,
    pub value: Box<dyn Expr>,
}

impl Statement for GlobalVariableStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn kind(&self) -> StatementKind {
        StatementKind::GlobalVariable
    }
}

#[derive(Clone)]
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
