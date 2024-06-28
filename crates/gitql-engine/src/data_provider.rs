use gitql_core::object::Row;

/// DataProvider is a component that used to provide and map the data to the GitQL Engine
///
/// User should implement [`DataProvider`] trait for each data format for example files, logs, api
pub trait DataProvider {
    fn provide(&self, table: &str, selected_columns: &[String]) -> Result<Vec<Row>, String>;
}
