use std::collections::HashMap;

use gitql_ast::scope::Scope;
use gitql_ast::statement::AggregateFunction;

pub struct ParserContext {
    pub symbol_table: Scope,
    pub aggregations: HashMap<String, AggregateFunction>,

    pub selected_fields: Vec<String>,
    pub hidden_selections: Vec<String>,

    pub generated_field_count: i32,
    pub is_single_value_query: bool,
    pub has_group_by_statement: bool,
}

impl Default for ParserContext {
    fn default() -> Self {
        ParserContext {
            symbol_table: Scope::new(),
            aggregations: HashMap::new(),
            selected_fields: Vec::new(),
            hidden_selections: Vec::new(),
            generated_field_count: 0,
            is_single_value_query: false,
            has_group_by_statement: false,
        }
    }
}

impl ParserContext {
    pub fn generate_column_name(&mut self) -> String {
        self.generated_field_count += 1;
        format!("column_{}", self.generated_field_count)
    }
}
