use crate::object::GQLObject;
use crate::types::DataType;
use crate::value::Value;

use lazy_static::lazy_static;
use std::collections::HashMap;

type Aggregation = fn(&String, &Vec<GQLObject>) -> Value;

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
                parameter: DataType::Integer,
                result: DataType::Integer,
            },
        );
        map.insert(
            "min",
            AggregationPrototype {
                parameter: DataType::Integer,
                result: DataType::Integer,
            },
        );
        map.insert(
            "sum",
            AggregationPrototype {
                parameter: DataType::Any,
                result: DataType::Integer,
            },
        );
        map.insert(
            "avg",
            AggregationPrototype {
                parameter: DataType::Any,
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

fn aggregation_max(field_name: &String, objects: &Vec<GQLObject>) -> Value {
    let mut max_length: i64 = 0;
    for object in objects {
        let field_value = &object.attributes.get(field_name).unwrap();
        let int_value = field_value.as_int();
        if int_value > max_length {
            max_length = int_value;
        }
    }
    return Value::Integer(max_length);
}

fn aggregation_min(field_name: &String, objects: &Vec<GQLObject>) -> Value {
    let mut min_length: i64 = 0;
    for object in objects {
        let field_value = &object.attributes.get(field_name).unwrap();
        let int_value = field_value.as_int();
        if int_value < min_length {
            min_length = int_value;
        }
    }
    return Value::Integer(min_length);
}

fn aggregation_sum(field_name: &String, objects: &Vec<GQLObject>) -> Value {
    let mut sum: i64 = 0;
    for object in objects {
        let field_value = &object.attributes.get(field_name).unwrap();
        sum += field_value.as_int();
    }
    return Value::Integer(sum);
}

fn aggregation_average(field_name: &String, objects: &Vec<GQLObject>) -> Value {
    let mut sum: i64 = 0;
    let count: i64 = objects.len().try_into().unwrap();
    for object in objects {
        let field_value = &object.attributes.get(field_name).unwrap();
        sum += field_value.as_int();
    }
    let avg = sum / count;
    return Value::Integer(avg);
}

fn aggregation_count(_field_name: &String, objects: &Vec<GQLObject>) -> Value {
    return Value::Integer(objects.len() as i64);
}
