use crate::signature::Signature;
use crate::types::same_type_as_first_parameter;
use crate::types::DataType;
use crate::value::Value;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::OnceLock;

/// Aggregation function accept a selected row values for each row in group and return single [`Value`]
///
/// [`Vec<Vec<Value>>`] represent the selected values from each row in group
///
/// For Example if we have three rows in group and select name and email from each one
///
/// [[name, email], [name, email], [name, email]]
///
/// This implementation allow aggregation function to accept more than one parameter,
/// and also accept any Expression not only field name
///
type Aggregation = fn(Vec<Vec<Value>>) -> Value;

pub fn aggregation_functions() -> &'static HashMap<&'static str, Aggregation> {
    static HASHMAP: OnceLock<HashMap<&'static str, Aggregation>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut map: HashMap<&'static str, Aggregation> = HashMap::new();
        map.insert("max", aggregation_max);
        map.insert("min", aggregation_min);
        map.insert("sum", aggregation_sum);
        map.insert("avg", aggregation_average);
        map.insert("count", aggregation_count);
        map.insert("group_concat", aggregation_group_concat);
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
        map.insert(
            "group_concat",
            Signature {
                parameters: vec![DataType::Varargs(Box::new(DataType::Any))],
                return_type: DataType::Text,
            },
        );
        map
    })
}

fn aggregation_max(group_values: Vec<Vec<Value>>) -> Value {
    let mut max_value = &group_values[0][0];
    for row_values in &group_values {
        let single_value = &row_values[0];
        if max_value.compare(single_value) == Ordering::Greater {
            max_value = &single_value;
        }
    }
    max_value.clone()
}

fn aggregation_min(group_values: Vec<Vec<Value>>) -> Value {
    let mut min_value = &group_values[0][0];
    for row_values in &group_values {
        let single_value = &row_values[0];
        if min_value.compare(single_value) == Ordering::Less {
            min_value = &single_value;
        }
    }
    min_value.clone()
}

fn aggregation_sum(group_values: Vec<Vec<Value>>) -> Value {
    let mut sum: i64 = 0;
    for row_values in group_values {
        sum += &row_values[0].as_int();
    }
    Value::Integer(sum)
}

fn aggregation_average(group_values: Vec<Vec<Value>>) -> Value {
    let mut sum: i64 = 0;
    for row_values in &group_values {
        sum += &row_values[0].as_int();
    }
    let count: i64 = group_values[0].len().try_into().unwrap();
    Value::Integer(sum / count)
}

fn aggregation_count(group_values: Vec<Vec<Value>>) -> Value {
    Value::Integer(group_values.len() as i64)
}

fn aggregation_group_concat(group_values: Vec<Vec<Value>>) -> Value {
    let mut string_values: Vec<String> = vec![];
    for row_values in group_values {
        for value in row_values {
            string_values.push(value.to_string());
        }
    }
    Value::Text(string_values.concat())
}
