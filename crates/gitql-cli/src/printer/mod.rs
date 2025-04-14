use gitql_core::object::GitQLObject;

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
