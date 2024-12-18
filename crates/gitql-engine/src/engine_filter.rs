use gitql_ast::expression::Expr;
use gitql_core::environment::Environment;
use gitql_core::object::Row;
use gitql_core::values::boolean::BoolValue;

use crate::engine_evaluator::evaluate_expression;

#[inline(always)]
#[allow(clippy::borrowed_box)]
pub(crate) fn apply_filter_operation(
    env: &mut Environment,
    condition: &Box<dyn Expr>,
    titles: &[String],
    rows: &mut Vec<Row>,
) -> Result<(), String> {
    let mut positions_to_delete = vec![];
    for (index, row) in rows.iter().enumerate() {
        let expression = evaluate_expression(env, condition, titles, &row.values)?;
        if let Some(bool_value) = expression.as_any().downcast_ref::<BoolValue>() {
            if !bool_value.value {
                positions_to_delete.push(index);
            }
        }
    }

    for position in positions_to_delete.iter().rev() {
        rows.remove(*position);
    }

    Ok(())
}
