use std::collections::HashMap;

use crate::value::Value;

#[derive(Clone)]
pub struct GQLObject {
    pub attributes: HashMap<String, Value>,
}

pub fn flat_gql_groups(groups: &mut Vec<Vec<GQLObject>>) {
    let mut main_group: Vec<GQLObject> = Vec::new();
    for group in groups.into_iter() {
        main_group.append(group);
    }

    groups.clear();
    groups.push(main_group);
}
