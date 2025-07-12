use std::io::stdout;
use std::io::Write;

use csv::Writer;
use gitql_core::object::GitQLObject;

use super::BaseOutputPrinter;

pub struct CSVPrinter;

impl BaseOutputPrinter for CSVPrinter {
    fn print(&self, object: &mut GitQLObject) {
        let mut writer = Writer::from_writer(vec![]);
        let _ = writer.write_record(object.titles.clone());
        let row_len = object.titles.len();
        if let Some(group) = object.groups.first() {
            for row in &group.rows {
                let mut values_row: Vec<String> = Vec::with_capacity(row_len);
                for value in &row.values {
                    values_row.push(value.literal());
                }
                let _ = writer.write_record(values_row);
            }
        }

        if let Ok(writer_content) = writer.into_inner() {
            if let Ok(content) = String::from_utf8(writer_content) {
                if let Err(error) = writeln!(stdout(), "{content}") {
                    eprintln!("{error}");
                    std::process::exit(1);
                }
            }
        }
    }
}
