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
            println!("=> Line {}, Column {},", location.0, location.1);
        }

        if !query.is_empty() {
            println!("  |");
            println!("1 | {}", query);
            if let Some(location) = diagnostic.location() {
                print!("  | ");
                print!("{}", &"-".repeat(location.0));
                self.stdout.set_color(Some(Color::Yellow));
                println!("{}", &"^".repeat(usize::max(1, location.1 - location.0)));
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
