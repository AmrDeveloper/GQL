use crate::object::GQLObject;
use crate::types::DataType;
use crate::value::Value;

use lazy_static::lazy_static;
use std::cmp::Ordering;
use std::collections::HashMap;

type Aggregation = fn(&str, &[GQLObject]) -> Value;

pub struct AggregationPrototype {
    pub parameter: DataType,
    pub result: DataType,
}

lazy_static! {
    pub static ref AGGREGATIONS: HashMap<&'static str, Aggregation> = {
        let mut map: HashMap<&'static str, Aggregation> = HashMap::new();
        map.insert("max", aggregation_max);
        map.insert("min", aggregation_min);
        map.insert("sum", aggregation_sum);
        map.insert("avg", aggregation_average);
        map.insert("count", aggregation_count);
        map
    };
}

lazy_static! {
    pub static ref AGGREGATIONS_PROTOS: HashMap<&'static str, AggregationPrototype> = {
        let mut map: HashMap<&'static str, AggregationPrototype> = HashMap::new();
        map.insert(
            "max",
            AggregationPrototype {
                parameter: DataType::Variant(vec![
                    DataType::Integer,
                    DataType::Float,
                    DataType::Text,
                    DataType::Date,
                    DataType::Time,
                    DataType::DateTime,
                ]),
                result: DataType::Integer,
            },
        );
        map.insert(
            "min",
            AggregationPrototype {
                parameter: DataType::Variant(vec![
                    DataType::Integer,
                    DataType::Float,
                    DataType::Text,
                    DataType::Date,
                    DataType::Time,
                    DataType::DateTime,
                ]),
                result: DataType::Integer,
            },
        );
        map.insert(
            "sum",
            AggregationPrototype {
                parameter: DataType::Integer,
                result: DataType::Integer,
            },
        );
        map.insert(
            "avg",
            AggregationPrototype {
                parameter: DataType::Integer,
                result: DataType::Integer,
            },
        );
        map.insert(
            "count",
            AggregationPrototype {
                parameter: DataType::Any,
                result: DataType::Integer,
            },
        );
        map
    };
}

fn aggregation_max(field_name: &str, objects: &[GQLObject]) -> Value {
    let mut max_value = objects[0].attributes.get(field_name).unwrap();
    for object in objects.iter().skip(1) {
        let field_value = object.attributes.get(field_name).unwrap();
        if max_value.compare(field_value) == Ordering::Greater {
            max_value = field_value;
        }
    }
    max_value.clone()
}

fn aggregation_min(field_name: &str, objects: &[GQLObject]) -> Value {
    let mut min_value = objects[0].attributes.get(field_name).unwrap();
    for object in objects.iter().skip(1) {
        let field_value = object.attributes.get(field_name).unwrap();
        if min_value.compare(field_value) == Ordering::Less {
            min_value = field_value;
        }
    }
    min_value.clone()
}

fn aggregation_sum(field_name: &str, objects: &[GQLObject]) -> Value {
    let mut sum: i64 = 0;
    for object in objects {
        let field_value = &object.attributes.get(field_name).unwrap();
        sum += field_value.as_int();
    }
    Value::Integer(sum)
}

fn aggregation_average(field_name: &str, objects: &[GQLObject]) -> Value {
    let mut sum: i64 = 0;
    let count: i64 = objects.len().try_into().unwrap();
    for object in objects {
        let field_value = &object.attributes.get(field_name).unwrap();
        sum += field_value.as_int();
    }
    let avg = sum / count;
    Value::Integer(avg)
}

fn aggregation_count(_field_name: &str, objects: &[GQLObject]) -> Value {
    Value::Integer(objects.len() as i64)
}
