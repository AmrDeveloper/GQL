use crate::{object, statement::Statement};

pub fn evaluate(repo: &git2::Repository, statements: Vec<Box<dyn Statement>>) {
    let mut objects: Vec<object::GQLObject> = Vec::new();
    for statement in statements {
        statement.execute(repo, &mut objects);
    }

    for object in objects {
        object.print();
    }
}
