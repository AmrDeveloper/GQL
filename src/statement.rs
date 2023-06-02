use std::cmp;

use self::expression::Expression;

#[path = "expression.rs"]
mod expression;

use crate::object;

use crate::engine_function::select_gql_objects;

pub trait Statement {
    fn execute(&self, repo: &git2::Repository, objects: &mut Vec<object::GQLObject>);
}

pub struct SelectStatement {
    pub table_name: String,
    pub fields: Vec<String>,
}

impl Statement for SelectStatement {
    fn execute(&self, repo: &git2::Repository, objects: &mut Vec<object::GQLObject>) {
        let elements =
            select_gql_objects(repo, self.table_name.to_string(), self.fields.to_owned());
        for element in elements {
            objects.push(element);
        }
    }
}

pub struct WhereStatement {
    pub fields: dyn Expression,
}

impl Statement for WhereStatement {
    fn execute(&self, repo: &git2::Repository, objects: &mut Vec<object::GQLObject>) {}
}

pub struct LimitStatement {
    pub count: usize,
}

impl Statement for LimitStatement {
    fn execute(&self, repo: &git2::Repository, objects: &mut Vec<object::GQLObject>) {
        if self.count <= objects.len() {
            objects.drain(self.count..objects.len());
        }
    }
}

pub struct OffsetStatement {
    pub count: usize,
}

impl Statement for OffsetStatement {
    fn execute(&self, repo: &git2::Repository, objects: &mut Vec<object::GQLObject>) {
        objects.drain(0..cmp::min(self.count, objects.len()));
    }
}
