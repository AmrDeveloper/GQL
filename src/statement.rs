use std::cmp;
use std::collections::HashMap;
use std::vec;

use crate::aggregation::AGGREGATIONS;
use crate::engine_function::select_gql_objects;
use crate::expression::Expression;
use crate::object::GQLObject;
use crate::types::DataType;

pub trait Statement {
    fn execute(&self, repo: &git2::Repository, groups: &mut Vec<Vec<GQLObject>>);
}

pub struct GQLQuery {
    pub statements: HashMap<String, Box<dyn Statement>>,
    pub select_aggregations_only: bool,
}

pub struct SelectStatement {
    pub table_name: String,
    pub fields: Vec<String>,
    pub alias_table: HashMap<String, String>,
}

impl Statement for SelectStatement {
    fn execute(&self, repo: &git2::Repository, groups: &mut Vec<Vec<GQLObject>>) {
        // Select obects from the target table
        let objects = select_gql_objects(
            repo,
            self.table_name.to_string(),
            self.fields.to_owned(),
            self.alias_table.to_owned(),
        );

        // Push the selected elements as a first group
        groups.push(objects);
    }
}

pub struct WhereStatement {
    pub condition: Box<dyn Expression>,
}

impl Statement for WhereStatement {
    fn execute(&self, _repo: &git2::Repository, groups: &mut Vec<Vec<GQLObject>>) {
        // Perform where command only on the first group
        // because group by command not executed yet
        let filtered_group: Vec<GQLObject> = groups
            .first()
            .unwrap()
            .iter()
            .filter(|&object| self.condition.evaluate(object).eq("true"))
            .cloned()
            .collect();

        // Update the main group with the filtered data
        groups.remove(0);
        groups.push(filtered_group);
    }
}

pub struct HavingStatement {
    pub condition: Box<dyn Expression>,
}

impl Statement for HavingStatement {
    fn execute(&self, _repo: &git2::Repository, groups: &mut Vec<Vec<GQLObject>>) {
        if groups.len() > 1 {
            flat_groups(groups);
        }

        let main_group: &mut Vec<GQLObject> = groups[0].as_mut();

        let result: Vec<GQLObject> = main_group
            .iter()
            .filter(|&object| self.condition.evaluate(object).eq("true"))
            .cloned()
            .collect();

        main_group.clear();

        for object in result {
            main_group.push(object);
        }
    }
}

pub struct LimitStatement {
    pub count: usize,
}

impl Statement for LimitStatement {
    fn execute(&self, _repo: &git2::Repository, groups: &mut Vec<Vec<GQLObject>>) {
        if groups.len() > 1 {
            flat_groups(groups);
        }

        let main_group: &mut Vec<GQLObject> = groups[0].as_mut();
        if self.count <= main_group.len() {
            main_group.drain(self.count..main_group.len());
        }
    }
}

pub struct OffsetStatement {
    pub count: usize,
}

impl Statement for OffsetStatement {
    fn execute(&self, _repo: &git2::Repository, groups: &mut Vec<Vec<GQLObject>>) {
        if groups.len() > 1 {
            flat_groups(groups);
        }

        let main_group: &mut Vec<GQLObject> = groups[0].as_mut();
        main_group.drain(0..cmp::min(self.count, main_group.len()));
    }
}

pub struct OrderByStatement {
    pub field_name: String,
    pub is_ascending: bool,
    pub field_type: DataType,
}

impl Statement for OrderByStatement {
    fn execute(&self, _repo: &git2::Repository, groups: &mut Vec<Vec<GQLObject>>) {
        if groups.len() > 1 {
            flat_groups(groups);
        }

        let main_group: &mut Vec<GQLObject> = groups[0].as_mut();
        if main_group.is_empty() {
            return;
        }

        if main_group[0].attributes.contains_key(&self.field_name) {
            if self.field_type == DataType::Number {
                main_group.sort_by(|a, b| {
                    let first_value = a
                        .attributes
                        .get(&self.field_name.to_string())
                        .unwrap()
                        .to_string()
                        .parse::<i64>()
                        .unwrap();

                    let other = b
                        .attributes
                        .get(&self.field_name.to_string())
                        .unwrap()
                        .to_string()
                        .parse::<i64>()
                        .unwrap();
                    first_value.partial_cmp(&other).unwrap()
                });
            } else {
                main_group.sort_by_cached_key(|object| {
                    object
                        .attributes
                        .get(&self.field_name.to_string())
                        .unwrap()
                        .to_string()
                });
            }

            if !self.is_ascending {
                main_group.reverse();
            }
        }
    }
}

pub struct GroupByStatement {
    pub field_name: String,
}

impl Statement for GroupByStatement {
    fn execute(&self, _repo: &git2::Repository, groups: &mut Vec<Vec<GQLObject>>) {
        let main_group: Vec<GQLObject> = groups.remove(0);
        if main_group.is_empty() {
            return;
        }

        // Mapping each unique value to it group index
        let mut groups_map: HashMap<String, usize> = HashMap::new();

        // Track current group index
        let mut next_group_index = 0;

        for object in main_group.into_iter() {
            let field_value = object.attributes.get(&self.field_name).unwrap();

            // If there is an existing group for this value, append current object to it
            if groups_map.contains_key(field_value) {
                let index = *groups_map.get(field_value).unwrap();
                let target_group = &mut groups[index];
                target_group.push(object.to_owned());
            }
            // Push a new group for this unique value and update the next index
            else {
                groups_map.insert(field_value.to_string(), next_group_index);
                next_group_index += 1;
                groups.push(vec![object.to_owned()]);
            }
        }
    }
}

pub struct AggregateFunction {
    pub function_name: String,
    pub argument: String,
}

pub struct AggregationFunctionsStatement {
    pub aggregations: HashMap<String, AggregateFunction>,
}

impl Statement for AggregationFunctionsStatement {
    fn execute(&self, _repo: &git2::Repository, groups: &mut Vec<Vec<GQLObject>>) {
        // Make sure you have at least one aggregation function to calculate
        let aggregations_map = &self.aggregations;
        if aggregations_map.is_empty() {
            return;
        }

        // Used to determind if group by statement is executed before or not
        let groups_count = groups.len();

        // We should run aggregation function for each group
        for group in groups {
            for aggregation in aggregations_map {
                let function = aggregation.1;

                // Get the target aggregation function
                let aggregation_function =
                    AGGREGATIONS.get(function.function_name.as_str()).unwrap();

                // Execute aggregation function once for group
                let result_column_name = aggregation.0;
                let result = &aggregation_function(&function.argument, &group);

                // Insert the calculated value in the group objects
                for object in group.into_iter() {
                    object
                        .attributes
                        .insert(result_column_name.to_string(), result.to_string());
                }
            }

            // In case of group by statement is exectued
            // Remove all elements expect the first one
            if groups_count > 1 {
                group.drain(1..);
            }
        }
    }
}

fn flat_groups(groups: &mut Vec<Vec<GQLObject>>) {
    let mut main_group: Vec<GQLObject> = Vec::new();
    for group in groups.into_iter() {
        main_group.append(group);
    }

    groups.clear();
    groups.push(main_group);
}
