use gitql_core::object::GitQLObject;

/// Represent the different type of available formats
#[derive(Debug, PartialEq)]
pub enum OutputFormatKind {
    /// Render the output as table
    Table,
    /// Print the output in JSON format
    JSON,
    /// Print the output in CSV format
    CSV,
    /// Print the output in YAML format
    YAML,
}

pub trait BaseOutputPrinter {
    fn print(&self, object: &mut GitQLObject);
}

mod csv_printer;
pub use csv_printer::CSVPrinter;

mod json_printer;
pub use json_printer::JSONPrinter;

mod table_printer;
pub use table_printer::TablePrinter;

mod yaml_printer;
pub use yaml_printer::YAMLPrinter;
