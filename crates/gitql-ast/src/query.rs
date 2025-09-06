use std::collections::HashMap;

use crate::expression::Expr;
use crate::statement::Statement;

pub enum Query {
    Select(SelectQuery),
    GlobalVariableDecl(GlobalVariableDeclQuery),
    Do(DoQuery),
    DescribeTable(DescribeQuery),
    ShowTables,
}

pub struct SelectQuery {
    pub statements: HashMap<&'static str, Statement>,
    pub alias_table: HashMap<String, String>,
    pub has_aggregation_function: bool,
    pub has_group_by_statement: bool,
    pub hidden_selections: HashMap<String, Vec<String>>,
}

pub struct DoQuery {
    pub exprs: Vec<Box<dyn Expr>>,
}

pub struct DescribeQuery {
    pub table_name: String,
}

pub struct GlobalVariableDeclQuery {
    pub name: String,
    pub value: Box<dyn Expr>,
}
