use std::collections::HashMap;

#[derive(Clone)]
pub struct GQLObject {
    pub attributes: HashMap<String, String>,
}

impl GQLObject {
    pub fn print(self) {
        let mut keys: Vec<&str> = self.attributes.keys().map(|k| k.as_ref()).collect();
        keys.sort();

        for key in keys {
            println!("[{}] = {}", key, self.attributes[key]);
        }

        println!("-------------------------------------------");
    }
}
