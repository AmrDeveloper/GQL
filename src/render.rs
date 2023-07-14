use prettytable::{Cell, Row, Table};

use crate::object::GQLObject;

pub fn render_objects(groups: &Vec<Vec<GQLObject>>, hidden_selections: &Vec<String>) {
    if groups.is_empty() || groups[0].is_empty() {
        return;
    }

    let titles: Vec<&str> = groups[0][0]
        .attributes
        .keys()
        .filter(|s| !hidden_selections.contains(s))
        .map(|k| k.as_ref())
        .collect();

    let mut table = Table::new();
    let mut table_titles: Vec<Cell> = Vec::new();

    for key in &titles {
        table_titles.push(Cell::new(key));
    }

    table.add_row(Row::new(table_titles));

    let table_field_max_len = 40;
    for group in groups {
        for object in group {
            let mut table_row = Row::new(Vec::new());
            for key in &titles {
                let value = &object.attributes.get(&key as &str).unwrap();
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
