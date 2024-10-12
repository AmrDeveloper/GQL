use gitql_ast::expression::Expression;
use gitql_core::environment::Environment;
use gitql_core::object::Row;
use gitql_core::values::boolean::BoolValue;

use crate::engine_evaluator::evaluate_expression;

#[inline(always)]
#[allow(clippy::borrowed_box)]
pub(crate) fn apply_filter_operation(
    env: &mut Environment,
    condition: &Box<dyn Expression>,
    titles: &[String],
    objects: &Vec<Row>,
) -> Result<Vec<Row>, String> {
    let mut rows: Vec<Row> = vec![];

    for object in objects {
        let expression = evaluate_expression(env, condition, titles, &object.values)?;
        if let Some(bool_value) = expression.as_any().downcast_ref::<BoolValue>() {
            if bool_value.value {
                rows.push(Row {
                    values: object.values.clone(),
                });
            }
        }
    }

    Ok(rows)
}
