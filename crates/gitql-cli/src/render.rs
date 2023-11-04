use gitql_ast::object::flat_gql_groups;
use gitql_ast::object::GQLObject;

enum PaginationInput {
    NextPage,
    PreviousPage,
    Quit,
}

pub fn render_objects(
    groups: &mut Vec<Vec<GQLObject>>,
    hidden_selections: &[String],
    pagination: bool,
    page_size: usize,
) {
    if groups.len() > 1 {
        flat_gql_groups(groups);
    }

    if groups.is_empty() || groups[0].is_empty() {
        return;
    }

    let gql_group = groups.first().unwrap();
    let gql_group_len = gql_group.len();

    let titles: Vec<&str> = groups[0][0]
        .attributes
        .keys()
        .filter(|s| !hidden_selections.contains(s))
        .map(|k| k.as_ref())
        .collect();

    // Setup table headers
    let header_color = comfy_table::Color::Green;
    let mut table_headers = vec![];
    for key in &titles {
        table_headers.push(comfy_table::Cell::new(key).fg(header_color));
    }

    // Print all data without pagination
    if !pagination || page_size >= gql_group_len {
        print_group_as_table(&titles, table_headers, gql_group);
        return;
    }

    // Setup the pagination mode
    let number_of_pages = (gql_group_len as f64 / page_size as f64).ceil() as usize;
    let mut current_page = 1;

    loop {
        let start_index = (current_page - 1) * page_size;
        let end_index = (start_index + page_size).min(gql_group_len);

        let current_page_groups = &gql_group[start_index..end_index].to_vec();

        println!("Page {}/{}", current_page, number_of_pages);
        print_group_as_table(&titles, table_headers.clone(), current_page_groups);

        let pagination_input = handle_pagination_input(current_page, number_of_pages);
        match pagination_input {
            PaginationInput::NextPage => current_page += 1,
            PaginationInput::PreviousPage => current_page -= 1,
            PaginationInput::Quit => break,
        }
    }
}

fn print_group_as_table(
    titles: &Vec<&str>,
    table_headers: Vec<comfy_table::Cell>,
    group: &Vec<GQLObject>,
) {
    let mut table = comfy_table::Table::new();

    // Setup table style
    table.load_preset(comfy_table::presets::UTF8_FULL);
    table.apply_modifier(comfy_table::modifiers::UTF8_ROUND_CORNERS);
    table.set_content_arrangement(comfy_table::ContentArrangement::Dynamic);

    table.set_header(table_headers);

    // Add rows to the table
    for object in group {
        let mut table_row: Vec<comfy_table::Cell> = vec![];
        for key in titles {
            let value = object.attributes.get(key as &str).unwrap();
            let value_literal = value.literal();
            table_row.push(comfy_table::Cell::new(value_literal.as_str()));
        }
        table.add_row(table_row);
    }

    // Print table
    println!("{table}");
}

fn handle_pagination_input(current_page: usize, number_of_pages: usize) -> PaginationInput {
    loop {
        if current_page < 2 {
            println!("Enter 'n' for next page, or 'q' to quit:");
        } else if current_page == number_of_pages {
            println!("'p' for previous page, or 'q' to quit:");
        } else {
            println!("Enter 'n' for next page, 'p' for previous page, or 'q' to quit:");
        }

        std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!");

        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read input");

        let input = line.trim();
        if input == "q" || input == "n" || input == "p" {
            match input {
                "n" => {
                    if current_page < number_of_pages {
                        return PaginationInput::NextPage;
                    } else {
                        println!("Already on the last page");
                        continue;
                    }
                }
                "p" => {
                    if current_page > 1 {
                        return PaginationInput::PreviousPage;
                    } else {
                        println!("Already on the first page");
                        continue;
                    }
                }
                "q" => return PaginationInput::Quit,
                _ => unreachable!(),
            }
        }

        println!("Invalid input");
    }
}
