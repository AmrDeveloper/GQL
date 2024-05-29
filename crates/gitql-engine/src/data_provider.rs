use gitql_ast::expression::Expression;
use gitql_core::environment::Environment;
use gitql_core::object::GitQLObject;
use gitql_core::object::Group;
use gitql_core::object::Row;

use crate::engine_evaluator::evaluate_expression;

/// DataProvider is a component that used to provide and map the data to the GitQL Engine
///
/// User should implement [`DataProvider`] trait for each data format for example files, logs, api
pub trait DataProvider {
    fn provide(
        &self,
        env: &mut Environment,
        table: &str,
        fields_names: &[String],
        titles: &[String],
        fields_values: &[Box<dyn Expression>],
    ) -> GitQLObject;
}

pub fn select_values(
    env: &mut Environment,
    titles: &[String],
    fields_values: &[Box<dyn Expression>],
) -> Result<Group, String> {
    let mut group = Group { rows: vec![] };
    let mut values = Vec::with_capacity(fields_values.len());

    for value in fields_values.iter() {
        let evaluated = evaluate_expression(env, value, titles, &values)?;
        values.push(evaluated);
    }

    group.rows.push(Row { values });
    Ok(group)
}
