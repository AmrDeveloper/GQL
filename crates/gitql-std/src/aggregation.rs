use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::OnceLock;

use gitql_core::dynamic_types::type_of_first_element;
use gitql_core::signature::Aggregation;
use gitql_core::signature::Signature;
use gitql_core::types::DataType;
use gitql_core::value::Value;

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
        map.insert("bool_and", aggregation_bool_and);
        map.insert("bool_or", aggregation_bool_or);
        map.insert("bit_and", aggregation_bit_and);
        map.insert("bit_or", aggregation_bit_or);
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
                return_type: DataType::Dynamic(type_of_first_element),
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
                return_type: DataType::Dynamic(type_of_first_element),
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
        map.insert(
            "bool_and",
            Signature {
                parameters: vec![DataType::Boolean],
                return_type: DataType::Boolean,
            },
        );
        map.insert(
            "bool_or",
            Signature {
                parameters: vec![DataType::Boolean],
                return_type: DataType::Boolean,
            },
        );
        map.insert(
            "bit_and",
            Signature {
                parameters: vec![DataType::Integer],
                return_type: DataType::Integer,
            },
        );
        map.insert(
            "bit_or",
            Signature {
                parameters: vec![DataType::Integer],
                return_type: DataType::Integer,
            },
        );
        map
    })
}

pub fn aggregation_max(group_values: Vec<Vec<Value>>) -> Value {
    let mut max_value = &group_values[0][0];
    for row_values in &group_values {
        let single_value = &row_values[0];
        if max_value.compare(single_value) == Ordering::Greater {
            max_value = single_value;
        }
    }
    max_value.clone()
}

pub fn aggregation_min(group_values: Vec<Vec<Value>>) -> Value {
    let mut min_value = &group_values[0][0];
    for row_values in &group_values {
        let single_value = &row_values[0];
        if min_value.compare(single_value) == Ordering::Less {
            min_value = single_value;
        }
    }
    min_value.clone()
}

pub fn aggregation_sum(group_values: Vec<Vec<Value>>) -> Value {
    let mut sum: i64 = 0;
    for row_values in group_values {
        sum += &row_values[0].as_int();
    }
    Value::Integer(sum)
}

pub fn aggregation_average(group_values: Vec<Vec<Value>>) -> Value {
    let mut sum: i64 = 0;
    for row_values in &group_values {
        sum += &row_values[0].as_int();
    }
    let count: i64 = group_values[0].len().try_into().unwrap();
    Value::Integer(sum / count)
}

pub fn aggregation_count(group_values: Vec<Vec<Value>>) -> Value {
    Value::Integer(group_values.len() as i64)
}

pub fn aggregation_group_concat(group_values: Vec<Vec<Value>>) -> Value {
    let mut string_values: Vec<String> = vec![];
    for row_values in group_values {
        for value in row_values {
            string_values.push(value.to_string());
        }
    }
    Value::Text(string_values.concat())
}

pub fn aggregation_bool_and(group_values: Vec<Vec<Value>>) -> Value {
    for row_values in group_values {
        if !row_values[0].as_bool() {
            return Value::Boolean(false);
        }
    }
    Value::Boolean(true)
}

pub fn aggregation_bool_or(group_values: Vec<Vec<Value>>) -> Value {
    for row_values in group_values {
        if row_values[0].as_bool() {
            return Value::Boolean(true);
        }
    }
    Value::Boolean(false)
}

pub fn aggregation_bit_and(group_values: Vec<Vec<Value>>) -> Value {
    let mut value: i64 = 1;
    let mut has_non_null = false;
    for row_values in group_values {
        if row_values[0].data_type().is_null() {
            continue;
        }
        value &= row_values[0].as_int();
        has_non_null = true;
    }

    if has_non_null {
        Value::Integer(value)
    } else {
        Value::Null
    }
}

pub fn aggregation_bit_or(group_values: Vec<Vec<Value>>) -> Value {
    let mut value: i64 = 0;
    let mut has_non_null = false;
    for row_values in group_values {
        if row_values[0].data_type().is_null() {
            continue;
        }
        value |= row_values[0].as_int();
        has_non_null = true;
    }

    if has_non_null {
        Value::Integer(value)
    } else {
        Value::Null
    }
}
