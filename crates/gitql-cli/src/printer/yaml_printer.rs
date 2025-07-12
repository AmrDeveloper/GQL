use std::io::stdout;
use std::io::Write;

use gitql_core::object::GitQLObject;
use linked_hash_map::LinkedHashMap;
use yaml_rust::Yaml;
use yaml_rust::YamlEmitter;

use super::BaseOutputPrinter;

pub struct YAMLPrinter;

impl BaseOutputPrinter for YAMLPrinter {
    fn print(&self, object: &mut GitQLObject) {
        let mut out_str = String::new();
        let mut emitter = YamlEmitter::new(&mut out_str);

        let columns_titles = &object.titles;

        let main_group = object.groups.first().unwrap();
        let mut rows_rows: Vec<Yaml> = Vec::with_capacity(main_group.rows.len());

        for row in main_group.rows.clone() {
            let mut vec: LinkedHashMap<Yaml, Yaml> = LinkedHashMap::new();
            for (column_index, column_value) in row.values.iter().enumerate() {
                vec.insert(
                    Yaml::String(columns_titles[column_index].to_string()),
                    Yaml::String(column_value.to_string()),
                );
            }
            let row_yaml = Yaml::Hash(vec);
            rows_rows.push(row_yaml);
        }

        if let Err(error) = emitter.dump(&Yaml::Array(rows_rows)) {
            eprintln!("{error}");
            std::process::exit(1);
        }

        if let Err(error) = writeln!(stdout(), "{out_str}") {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}
