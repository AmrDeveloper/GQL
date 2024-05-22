use crate::object::Group;
use crate::signature::Signature;
use crate::types::same_type_as_first_parameter;
use crate::types::DataType;
use crate::value::Value;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::OnceLock;

type Aggregation = fn(&str, &[String], &Group) -> Value;

pub fn aggregation_functions() -> &'static HashMap<&'static str, Aggregation> {
    static HASHMAP: OnceLock<HashMap<&'static str, Aggregation>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut map: HashMap<&'static str, Aggregation> = HashMap::new();
        map.insert("max", aggregation_max);
        map.insert("min", aggregation_min);
        map.insert("sum", aggregation_sum);
        map.insert("avg", aggregation_average);
        map.insert("count", aggregation_count);
        map
    })
}

pub fn aggregation_function_signatures() -> &'static HashMap<&'static str, Signature> {
    static HASHMAP: OnceLock<HashMap<&'static str, Signature>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut map: HashMap<&'static str, Signature> = HashMap::new();
        map.insert(
            "max",
            Signature {
                parameters: vec![DataType::Variant(vec![
                    DataType::Integer,
                    DataType::Float,
                    DataType::Text,
                    DataType::Date,
                    DataType::Time,
                    DataType::DateTime,
                ])],
                return_type: DataType::Dynamic(same_type_as_first_parameter),
            },
        );
        map.insert(
            "min",
            Signature {
                parameters: vec![DataType::Variant(vec![
                    DataType::Integer,
                    DataType::Float,
                    DataType::Text,
                    DataType::Date,
                    DataType::Time,
                    DataType::DateTime,
                ])],
                return_type: DataType::Dynamic(same_type_as_first_parameter),
            },
        );
        map.insert(
            "sum",
            Signature {
                parameters: vec![DataType::Integer],
                return_type: DataType::Integer,
            },
        );
        map.insert(
            "avg",
            Signature {
                parameters: vec![DataType::Integer],
                return_type: DataType::Integer,
            },
        );
        map.insert(
            "count",
            Signature {
                parameters: vec![DataType::Optional(Box::new(DataType::Any))],
                return_type: DataType::Integer,
            },
        );
        map
    })
}

fn aggregation_max(field_name: &str, titles: &[String], objects: &Group) -> Value {
    let column_index = titles.iter().position(|r| r.eq(&field_name)).unwrap();
    let mut max_value = objects.rows[0].values.get(column_index).unwrap();
    for row in &objects.rows {
        let field_value = &row.values.get(column_index).unwrap();
        if max_value.compare(field_value) == Ordering::Greater {
            max_value = field_value;
        }
    }
    max_value.clone()
}

fn aggregation_min(field_name: &str, titles: &[String], objects: &Group) -> Value {
    let column_index = titles.iter().position(|r| r.eq(&field_name)).unwrap();
    let mut min_value = objects.rows[0].values.get(column_index).unwrap();
    for row in &objects.rows {
        let field_value = &row.values.get(column_index).unwrap();
        if min_value.compare(field_value) == Ordering::Less {
            min_value = field_value;
        }
    }
    min_value.clone()
}

fn aggregation_sum(field_name: &str, titles: &[String], objects: &Group) -> Value {
    let mut sum: i64 = 0;
    let column_index = titles.iter().position(|r| r.eq(&field_name)).unwrap();
    for row in &objects.rows {
        let field_value = &row.values.get(column_index).unwrap();
        sum += field_value.as_int();
    }
    Value::Integer(sum)
}

fn aggregation_average(field_name: &str, titles: &[String], objects: &Group) -> Value {
    let mut sum: i64 = 0;
    let count: i64 = objects.len().try_into().unwrap();
    let column_index = titles.iter().position(|r| r.eq(&field_name)).unwrap();
    for row in &objects.rows {
        let field_value = &row.values.get(column_index).unwrap();
        sum += field_value.as_int();
    }
    let avg = sum / count;
    Value::Integer(avg)
}

fn aggregation_count(_field_name: &str, _titles: &[String], objects: &Group) -> Value {
    Value::Integer(objects.len() as i64)
}
