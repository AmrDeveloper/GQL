use std::collections::HashMap;

use gitql_ast::statement::{AggregateValue, WindowDefinition, WindowFunction};

use crate::name_generator::NameGenerator;
use crate::token::SourceLocation;

#[derive(Default)]
pub struct ParserContext {
    pub aggregations: HashMap<String, AggregateValue>,

    pub window_functions: HashMap<String, WindowFunction>,
    pub named_window_clauses: HashMap<String, WindowDefinition>,

    pub selected_fields: Vec<String>,
    pub hidden_selections: Vec<String>,

    pub selected_tables: Vec<String>,
    pub projection_names: Vec<String>,
    pub projection_locations: Vec<SourceLocation>,

    pub name_alias_table: HashMap<String, String>,
    pub name_generator: NameGenerator,

    pub has_select_statement: bool,
    
    pub is_single_value_query: bool,
    pub has_group_by_statement: bool,

    pub inside_selections: bool,
    pub inside_order_by: bool,
    pub inside_over_clauses: bool,

}
