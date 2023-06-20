use crate::{
    object::{self, render_objects},
    statement::Statement,
};

pub fn evaluate(repo: &git2::Repository, statements: Vec<Box<dyn Statement>>) {
    let mut objects: Vec<object::GQLObject> = Vec::new();
    for statement in statements {
        statement.execute(repo, &mut objects);
    }

    render_objects(&objects);
}
