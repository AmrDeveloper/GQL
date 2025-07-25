use comfy_table::Color;
use comfy_table::ContentArrangement;
use gitql_core::object::GitQLObject;
use gitql_core::object::Row;

use super::BaseOutputPrinter;

enum PaginationInput {
    NextPage,
    PreviousPage,
    Quit,
}

pub struct TablePrinter {
    pub pagination: bool,
    pub page_size: usize,
    pub theme_config: TableThemeConfig,
}

pub struct TableThemeConfig {
    pub header_forground_color: Option<Color>,
    pub header_background_color: Option<Color>,
    pub arrangement: Option<ContentArrangement>,
}

impl Default for TableThemeConfig {
    fn default() -> TableThemeConfig {
        TableThemeConfig {
            header_forground_color: Some(Color::Green),
            header_background_color: None,
            arrangement: Some(comfy_table::ContentArrangement::Dynamic),
        }
    }
}

impl TablePrinter {
    pub fn new(pagination: bool, page_size: usize) -> Self {
        TablePrinter {
            pagination,
            page_size,
            theme_config: TableThemeConfig::default(),
        }
    }

    pub fn set_theme(&mut self, theme: TableThemeConfig) {
        self.theme_config = theme;
    }
}

impl BaseOutputPrinter for TablePrinter {
    fn print(&self, object: &mut GitQLObject) {
        if object.is_empty() || object.groups[0].is_empty() {
            return;
        }

        let titles = &object.titles;
        let group = object.groups.first().unwrap();
        let group_len = group.len();

        // Setup table headers
        let mut table_headers = vec![];
        for key in titles {
            let mut table_cell = comfy_table::Cell::new(key);
            if let Some(forground_color) = self.theme_config.header_forground_color {
                table_cell = table_cell.fg(forground_color);
            }

            if let Some(background_color) = self.theme_config.header_background_color {
                table_cell = table_cell.bg(background_color);
            }

            table_headers.push(table_cell);
        }

        // Print all data without pagination
        if !self.pagination || self.page_size >= group_len {
            self.print_group_as_table(titles, table_headers, &group.rows);
            return;
        }

        // Setup the pagination mode
        let number_of_pages = (group_len as f64 / self.page_size as f64).ceil() as usize;
        let mut current_page = 1;

        loop {
            let start_index = (current_page - 1) * self.page_size;
            let end_index = (start_index + self.page_size).min(group_len);

            let current_page_groups = &group.rows[start_index..end_index];
            println!("Page {current_page}/{number_of_pages}");
            self.print_group_as_table(titles, table_headers.clone(), current_page_groups);

            let pagination_input = self.handle_pagination_input(current_page, number_of_pages);
            match pagination_input {
                PaginationInput::NextPage => current_page += 1,
                PaginationInput::PreviousPage => current_page -= 1,
                PaginationInput::Quit => break,
            }
        }
    }
}

impl TablePrinter {
    fn print_group_as_table(
        &self,
        titles: &[String],
        table_headers: Vec<comfy_table::Cell>,
        rows: &[Row],
    ) {
        let mut table = comfy_table::Table::new();

        // Setup table style
        table.load_preset(comfy_table::presets::UTF8_FULL);
        table.apply_modifier(comfy_table::modifiers::UTF8_ROUND_CORNERS);

        if let Some(arrangement) = &self.theme_config.arrangement {
            table.set_content_arrangement(arrangement.clone());
        }

        table.set_header(table_headers);

        let titles_len = titles.len();

        // Add rows to the table
        for row in rows {
            let mut table_row: Vec<comfy_table::Cell> = vec![];
            for index in 0..titles_len {
                if let Some(value) = row.values.get(index) {
                    table_row.push(comfy_table::Cell::new(value.literal()));
                }
            }
            table.add_row(table_row);
        }

        // Print table
        println!("{table}");
    }

    fn handle_pagination_input(
        &self,
        current_page: usize,
        number_of_pages: usize,
    ) -> PaginationInput {
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
}
