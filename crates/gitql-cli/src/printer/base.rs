use gitql_core::object::GitQLObject;

pub trait OutputPrinter {
    fn print(&self, object: &mut GitQLObject);
}
