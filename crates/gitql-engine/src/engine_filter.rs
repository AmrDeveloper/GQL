use gitql_ast::expression::Expression;
use gitql_core::environment::Environment;
use gitql_core::object::Row;

use crate::engine_evaluator::evaluate_expression;

#[inline(always)]
#[allow(clippy::borrowed_box)]
pub fn filter_rows_by_condition(
    env: &mut Environment,
    condition: &Box<dyn Expression>,
    titles: &[String],
    objects: &Vec<Row>,
) -> Result<Vec<Row>, String> {
    let mut rows: Vec<Row> = vec![];

    for object in objects {
        if evaluate_expression(env, condition, titles, &object.values)?.as_bool() {
            rows.push(Row {
                values: object.values.clone(),
            });
        }
    }

    Ok(rows)
}
