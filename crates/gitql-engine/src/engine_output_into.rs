use std::fs::File;
use std::io::Write;

use gitql_ast::statement::IntoStatement;
use gitql_core::object::GitQLObject;
use gitql_core::values::Value;

pub(crate) fn execute_into_statement(
    statement: &IntoStatement,
    gitql_object: &mut GitQLObject,
) -> Result<(), String> {
    let mut buffer = String::new();

    let line_terminated_by = &statement.lines_terminated;
    let field_terminated_by = &statement.fields_terminated;
    let enclosing = &statement.enclosed;

    // Headers
    let header = gitql_object.titles.join(field_terminated_by);
    buffer.push_str(&header);
    buffer.push_str(line_terminated_by);

    // Rows of the main group
    if let Some(main_group) = gitql_object.groups.first() {
        for row in &main_group.rows {
            let row_values: Vec<String> = row
                .values
                .iter()
                .map(|r| value_to_string_with_optional_enclosing(r, enclosing))
                .collect();
            buffer.push_str(&row_values.join(field_terminated_by));
            buffer.push_str(line_terminated_by);
        }
    }

    let file_result = File::create(statement.file_path.clone());
    if let Err(error) = file_result {
        return Err(error.to_string());
    }

    let mut file = file_result.ok().unwrap();
    let write_result = file.write_all(buffer.as_bytes());
    if let Err(error) = write_result {
        return Err(error.to_string());
    }

    Ok(())
}

#[inline(always)]
#[allow(clippy::borrowed_box)]
fn value_to_string_with_optional_enclosing(value: &Box<dyn Value>, enclosed: &String) -> String {
    if enclosed.is_empty() {
        return value.literal();
    }
    format!("{}{}{}", enclosed, value.literal(), enclosed)
}
