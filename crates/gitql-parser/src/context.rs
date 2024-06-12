use std::collections::HashMap;

use gitql_ast::statement::AggregateValue;

use crate::tokenizer::Location;

#[derive(Default)]
pub struct ParserContext {
    pub aggregations: HashMap<String, AggregateValue>,

    pub selected_fields: Vec<String>,
    pub hidden_selections: Vec<String>,

    pub table_name: String,
    pub projection_names: Vec<String>,
    pub projection_locations: Vec<Location>,

    pub name_alias_table: HashMap<String, String>,

    pub has_select_statement: bool,
    pub generated_field_count: i32,
    pub is_single_value_query: bool,
    pub has_group_by_statement: bool,
}

impl ParserContext {
    pub fn generate_column_name(&mut self) -> String {
        self.generated_field_count += 1;
        format!("column_{}", self.generated_field_count)
    }
}
