use prettytable::{Cell, Row, Table};
use std::collections::HashMap;

#[derive(Clone)]
pub struct GQLObject {
    pub attributes: HashMap<String, String>,
}

pub fn render_objects(groups: &Vec<Vec<GQLObject>>) {
    if groups.is_empty() || groups[0].is_empty() {
        return;
    }

    let mut titles: Vec<&str> = groups[0][0].attributes.keys().map(|k| k.as_ref()).collect();
    titles.sort();

    let mut table = Table::new();
    let mut table_titles: Vec<Cell> = Vec::new();
    for key in titles {
        table_titles.push(Cell::new(key));
    }

    table.add_row(Row::new(table_titles));

    let table_field_max_len = 40;
    for group in groups {
        for object in group {
            let mut keys: Vec<&str> = object.attributes.keys().map(|k| k.as_ref()).collect();
            keys.sort();

            let mut table_row = Row::new(Vec::new());
            for key in keys {
                let value = &object.attributes[key];
                if value.len() > table_field_max_len {
                    let wrapped = textwrap::wrap(value, table_field_max_len);
                    let formatted = wrapped.join("\n");
                    table_row.add_cell(Cell::new(&formatted));
                } else {
                    table_row.add_cell(Cell::new(value));
                }
            }
            table.add_row(table_row);
        }
    }

    table.printstd();
}
