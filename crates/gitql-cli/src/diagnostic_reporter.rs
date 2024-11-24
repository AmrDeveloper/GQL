use gitql_parser::diagnostic::Diagnostic;

use termcolor::Color;

use crate::colored_stream::ColoredStream;

#[derive(Default)]
pub struct DiagnosticReporter {
    stdout: ColoredStream,
}

impl DiagnosticReporter {
    pub fn report_diagnostic(&mut self, query: &str, diagnostic: Diagnostic) {
        self.stdout.set_color(Some(Color::Red));
        println!("[{}]: {}", diagnostic.label(), diagnostic.message());

        if let Some(location) = diagnostic.location() {
            println!(
                "  --> Location {}:{}",
                location.line_start, location.column_start
            );
        }

        if !query.is_empty() {
            println!("  |");
            if let Some(location) = diagnostic.location() {
                let lines: Vec<&str> = query.split('\n').collect();
                let end = u32::min(location.line_end, lines.len() as u32);
                for line_number in location.line_start - 1..end {
                    println!("{} | {}", line_number, lines[line_number as usize]);
                }
                println!("  | ");
                let column_s = location.column_start.saturating_sub(1) as usize;
                print!("{}", &"-".repeat(column_s));

                let diagnostic_length =
                    u32::max(1, location.column_end.saturating_sub(location.column_start)) as usize;

                self.stdout.set_color(Some(Color::Yellow));
                println!("{}", &"^".repeat(diagnostic_length));

                self.stdout.set_color(Some(Color::Red));
            }
            println!("  |");
        }

        self.stdout.set_color(Some(Color::Yellow));
        for note in diagnostic.notes() {
            println!(" = Note: {}", note);
        }

        self.stdout.set_color(Some(Color::Cyan));
        for help in diagnostic.helps() {
            println!(" = Help: {}", help);
        }

        self.stdout.set_color(Some(Color::Blue));
        if let Some(docs) = diagnostic.docs() {
            println!(" = Docs: {}", docs);
        }

        self.stdout.reset();
    }
}
