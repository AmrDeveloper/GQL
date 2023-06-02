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
    fn execute(&self, repo: &git2::Repository, list: &mut Vec<object::GQLObject>) {
        let objects = select_gql_objects(repo, self.table_name.to_string(), self.fields.to_owned());
        for object in objects {
            list.push(object);
        }
    }
}

pub struct WhereStatement {
    pub fields: dyn Expression,
}

impl Statement for WhereStatement {
    fn execute(&self, repo: &git2::Repository, list: &mut Vec<object::GQLObject>) {}
}
