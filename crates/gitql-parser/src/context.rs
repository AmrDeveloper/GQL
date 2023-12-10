use std::collections::HashMap;

use gitql_ast::statement::AggregateValue;

#[derive(Default)]
pub struct ParserContext {
    pub aggregations: HashMap<String, AggregateValue>,

    pub selected_fields: Vec<String>,
    pub hidden_selections: Vec<String>,

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
