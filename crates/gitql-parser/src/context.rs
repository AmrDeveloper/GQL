use std::collections::HashMap;

use gitql_ast::statement::AggregateValue;

use crate::tokenizer::Location;

#[derive(Default)]
pub struct ParserContext {
    pub aggregations: HashMap<String, AggregateValue>,

    pub selected_fields: Vec<String>,
    pub hidden_selections: Vec<String>,

    pub selected_tables: Vec<String>,
    pub projection_names: Vec<String>,
    pub projection_locations: Vec<Location>,

    pub name_alias_table: HashMap<String, String>,

    pub has_select_statement: bool,
    pub is_single_value_query: bool,
    pub has_group_by_statement: bool,
}
