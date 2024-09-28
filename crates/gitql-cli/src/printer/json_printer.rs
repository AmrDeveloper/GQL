use gitql_core::object::GitQLObject;

use super::base::OutputPrinter;

pub struct JSONPrinter;

impl OutputPrinter for JSONPrinter {
    fn print(&self, object: &mut GitQLObject) {
        let mut elements: Vec<serde_json::Value> = vec![];

        if let Some(group) = object.groups.first() {
            let titles = &object.titles;
            for row in &group.rows {
                let mut object = serde_json::Map::new();
                for (i, value) in row.values.iter().enumerate() {
                    object.insert(
                        titles[i].to_string(),
                        serde_json::Value::String(value.to_string()),
                    );
                }
                elements.push(serde_json::Value::Object(object));
            }
        }

        if let Ok(json_str) = serde_json::to_string(&serde_json::Value::Array(elements)) {
            println!("{}", json_str);
        }
    }
}
