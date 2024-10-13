use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::OnceLock;

use gitql_ast::types::any::AnyType;
use gitql_ast::types::boolean::BoolType;
use gitql_ast::types::date::DateType;
use gitql_ast::types::datetime::DateTimeType;
use gitql_ast::types::dynamic::DynamicType;
use gitql_ast::types::float::FloatType;
use gitql_ast::types::integer::IntType;
use gitql_ast::types::null::NullType;
use gitql_ast::types::optional::OptionType;
use gitql_ast::types::text::TextType;
use gitql_ast::types::time::TimeType;
use gitql_ast::types::varargs::VarargsType;
use gitql_ast::types::variant::VariantType;
use gitql_core::dynamic_types::array_of_type;
use gitql_core::dynamic_types::first_element_type;
use gitql_core::signature::Aggregation;
use gitql_core::signature::Signature;
use gitql_core::values::array::ArrayValue;
use gitql_core::values::base::Value;
use gitql_core::values::boolean::BoolValue;
use gitql_core::values::integer::IntValue;
use gitql_core::values::null::NullValue;
use gitql_core::values::text::TextValue;

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
        map.insert("bit_xor", aggregation_bit_xor);
        map.insert("array_agg", aggregation_array_agg);
        map
    })
}

pub fn aggregation_function_signatures() -> HashMap<&'static str, Signature> {
    let mut map: HashMap<&'static str, Signature> = HashMap::new();
    map.insert(
        "max",
        Signature {
            parameters: vec![Box::new(VariantType {
                variants: vec![
                    Box::new(IntType),
                    Box::new(FloatType),
                    Box::new(TextType),
                    Box::new(DateType),
                    Box::new(TimeType),
                    Box::new(DateTimeType),
                ],
            })],
            return_type: Box::new(DynamicType {
                function: first_element_type,
            }),
        },
    );
    map.insert(
        "min",
        Signature {
            parameters: vec![Box::new(VariantType {
                variants: vec![
                    Box::new(IntType),
                    Box::new(FloatType),
                    Box::new(TextType),
                    Box::new(DateType),
                    Box::new(TimeType),
                    Box::new(DateTimeType),
                ],
            })],
            return_type: Box::new(DynamicType {
                function: first_element_type,
            }),
        },
    );
    map.insert(
        "sum",
        Signature {
            parameters: vec![Box::new(IntType)],
            return_type: Box::new(IntType),
        },
    );
    map.insert(
        "avg",
        Signature {
            parameters: vec![Box::new(IntType)],
            return_type: Box::new(IntType),
        },
    );
    map.insert(
        "count",
        Signature {
            parameters: vec![Box::new(OptionType {
                base: Some(Box::new(AnyType)),
            })],
            return_type: Box::new(IntType),
        },
    );
    map.insert(
        "group_concat",
        Signature {
            parameters: vec![Box::new(VarargsType {
                base: Box::new(AnyType),
            })],
            return_type: Box::new(TextType),
        },
    );
    map.insert(
        "bool_and",
        Signature {
            parameters: vec![Box::new(BoolType)],
            return_type: Box::new(BoolType),
        },
    );
    map.insert(
        "bool_or",
        Signature {
            parameters: vec![Box::new(BoolType)],
            return_type: Box::new(BoolType),
        },
    );
    map.insert(
        "bit_and",
        Signature {
            parameters: vec![Box::new(IntType)],
            return_type: Box::new(IntType),
        },
    );
    map.insert(
        "bit_or",
        Signature {
            parameters: vec![Box::new(IntType)],
            return_type: Box::new(IntType),
        },
    );
    map.insert(
        "bit_xor",
        Signature {
            parameters: vec![Box::new(IntType)],
            return_type: Box::new(IntType),
        },
    );
    map.insert(
        "array_agg",
        Signature {
            parameters: vec![Box::new(AnyType)],
            return_type: Box::new(DynamicType {
                function: |elements| array_of_type(first_element_type(elements)),
            }),
        },
    );
    map
}

pub fn aggregation_max(group_values: Vec<Vec<Box<dyn Value>>>) -> Box<dyn Value> {
    let mut max_value = &group_values[0][0];
    for row_values in &group_values {
        let single_value = &row_values[0];
        if max_value.compare(single_value) == Some(Ordering::Less) {
            max_value = single_value;
        }
    }
    max_value.clone()
}

pub fn aggregation_min(group_values: Vec<Vec<Box<dyn Value>>>) -> Box<dyn Value> {
    let mut min_value = &group_values[0][0];
    for row_values in &group_values {
        let single_value = &row_values[0];
        if min_value.compare(single_value) == Some(Ordering::Greater) {
            min_value = single_value;
        }
    }
    min_value.clone()
}

pub fn aggregation_sum(group_values: Vec<Vec<Box<dyn Value>>>) -> Box<dyn Value> {
    let mut sum: i64 = 0;
    for row_values in group_values {
        if let Some(int_value) = row_values[0].as_any().downcast_ref::<IntValue>() {
            sum += int_value.value;
        }
    }
    Box::new(IntValue { value: sum })
}

pub fn aggregation_average(group_values: Vec<Vec<Box<dyn Value>>>) -> Box<dyn Value> {
    let mut sum: i64 = 0;
    for row_values in &group_values {
        if let Some(int_value) = row_values[0].as_any().downcast_ref::<IntValue>() {
            sum += int_value.value;
        }
    }
    let count: i64 = group_values[0].len().try_into().unwrap();
    Box::new(IntValue { value: sum / count })
}

pub fn aggregation_count(group_values: Vec<Vec<Box<dyn Value>>>) -> Box<dyn Value> {
    Box::new(IntValue {
        value: group_values.len() as i64,
    })
}

pub fn aggregation_group_concat(group_values: Vec<Vec<Box<dyn Value>>>) -> Box<dyn Value> {
    let mut string_values: Vec<String> = vec![];
    for row_values in group_values {
        for value in row_values {
            string_values.push(value.literal());
        }
    }
    Box::new(TextValue {
        value: string_values.concat(),
    })
}

pub fn aggregation_bool_and(group_values: Vec<Vec<Box<dyn Value>>>) -> Box<dyn Value> {
    for row_values in group_values {
        if let Some(bool_value) = row_values[0].as_any().downcast_ref::<BoolValue>() {
            if !bool_value.value {
                return Box::new(BoolValue { value: false });
            }
        }
    }
    Box::new(BoolValue { value: true })
}

pub fn aggregation_bool_or(group_values: Vec<Vec<Box<dyn Value>>>) -> Box<dyn Value> {
    for row_values in group_values {
        if let Some(bool_value) = row_values[0].as_any().downcast_ref::<BoolValue>() {
            if bool_value.value {
                return Box::new(BoolValue { value: true });
            }
        }
    }
    Box::new(BoolValue { value: false })
}

pub fn aggregation_bit_and(group_values: Vec<Vec<Box<dyn Value>>>) -> Box<dyn Value> {
    let mut value: i64 = 1;
    let mut has_non_null = false;
    for row_values in group_values {
        if row_values[0].data_type().is_null() {
            continue;
        }

        if let Some(int_value) = row_values[0].as_any().downcast_ref::<IntValue>() {
            value &= int_value.value;
            has_non_null = true;
        }
    }

    if has_non_null {
        Box::new(IntValue { value })
    } else {
        Box::new(NullValue)
    }
}

pub fn aggregation_bit_or(group_values: Vec<Vec<Box<dyn Value>>>) -> Box<dyn Value> {
    let mut value: i64 = 0;
    let mut has_non_null = false;
    for row_values in group_values {
        if row_values[0].data_type().is_null() {
            continue;
        }

        if let Some(int_value) = row_values[0].as_any().downcast_ref::<IntValue>() {
            value |= int_value.value;
            has_non_null = true;
        }
    }

    if has_non_null {
        Box::new(IntValue { value })
    } else {
        Box::new(NullValue)
    }
}

pub fn aggregation_bit_xor(group_values: Vec<Vec<Box<dyn Value>>>) -> Box<dyn Value> {
    let mut value: i64 = 0;
    let mut has_non_null = false;
    for row_values in group_values {
        if row_values[0].data_type().is_null() {
            continue;
        }

        if let Some(int_value) = row_values[0].as_any().downcast_ref::<IntValue>() {
            value ^= int_value.value;
            has_non_null = true;
        }
    }

    if has_non_null {
        Box::new(IntValue { value })
    } else {
        Box::new(NullValue)
    }
}

pub fn aggregation_array_agg(group_values: Vec<Vec<Box<dyn Value>>>) -> Box<dyn Value> {
    let mut array: Vec<Box<dyn Value>> = vec![];
    for row_values in group_values {
        array.push(row_values[0].clone());
    }

    let element_type = if array.is_empty() {
        Box::new(NullType)
    } else {
        array[0].data_type()
    };

    Box::new(ArrayValue {
        values: array,
        base_type: element_type,
    })
}
