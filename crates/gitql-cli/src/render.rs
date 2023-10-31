use gitql_ast::object::GQLObject;

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

    let mut table = comfy_table::Table::new();

    // Setup table style
    table.load_preset(comfy_table::presets::UTF8_FULL);
    table.apply_modifier(comfy_table::modifiers::UTF8_ROUND_CORNERS);
    table.set_content_arrangement(comfy_table::ContentArrangement::Dynamic);

    // Setup table headers
    let header_color = comfy_table::Color::Green;
    let mut table_headers = vec![];
    for key in &titles {
        table_headers.push(comfy_table::Cell::new(key).fg(header_color));
    }
    table.set_header(table_headers);

    // Push table rows
    for group in groups {
        for object in group {
            let mut table_row: Vec<comfy_table::Cell> = vec![];
            for key in &titles {
                let value = &object.attributes.get(&key as &str).clone().unwrap();
                let value_literal = value.literal();
                table_row.push(comfy_table::Cell::new(value_literal.as_str()));
            }
            table.add_row(table_row);
        }
    }

    // Print table in Stdout
    println!("{table}");
}
