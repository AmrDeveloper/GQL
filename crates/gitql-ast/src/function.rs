use crate::date_utils;
use crate::types::DataType;
use crate::value::Value;

use lazy_static::lazy_static;
use std::collections::HashMap;

type Function = fn(Vec<Value>) -> Value;

pub struct Prototype {
    pub parameters: Vec<DataType>,
    pub result: DataType,
}

lazy_static! {
    pub static ref FUNCTIONS: HashMap<&'static str, Function> = {
        let mut map: HashMap<&'static str, Function> = HashMap::new();
        // String functions
        map.insert("lower", text_lowercase);
        map.insert("upper", text_uppercase);
        map.insert("reverse", text_reverse);
        map.insert("replicate", text_replicate);
        map.insert("space", text_space);
        map.insert("trim", text_trim);
        map.insert("ltrim", text_left_trim);
        map.insert("rtrim", text_right_trim);
        map.insert("len", text_len);
        map.insert("ascii", text_ascii);
        map.insert("left", text_left);
        map.insert("datalength", text_datalength);
        map.insert("char", text_char);
        map.insert("replace", text_replace);
        map.insert("substring", text_substring);
        map.insert("stuff", text_stuff);

        // Date functions
        map.insert("current_date", date_current_date);
        map.insert("current_time", date_current_time);
        map.insert("current_timestamp", date_current_timestamp);
        map
    };
}

lazy_static! {
    pub static ref PROTOTYPES: HashMap<&'static str, Prototype> = {
        let mut map: HashMap<&'static str, Prototype> = HashMap::new();
        // String functions
        map.insert(
            "lower",
            Prototype {
                parameters: vec![DataType::Text],
                result: DataType::Text,
            },
        );
        map.insert(
            "upper",
            Prototype {
                parameters: vec![DataType::Text],
                result: DataType::Text,
            },
        );
        map.insert(
            "reverse",
            Prototype {
                parameters: vec![DataType::Text],
                result: DataType::Text,
            },
        );
        map.insert(
            "replicate",
            Prototype {
                parameters: vec![DataType::Text, DataType::Number],
                result: DataType::Text,
            },
        );
        map.insert(
            "space",
            Prototype {
                parameters: vec![DataType::Number],
                result: DataType::Text,
            },
        );
        map.insert(
            "trim",
            Prototype {
                parameters: vec![DataType::Text],
                result: DataType::Text,
            },
        );
        map.insert(
            "ltrim",
            Prototype {
                parameters: vec![DataType::Text],
                result: DataType::Text,
            },
        );
        map.insert(
            "rtrim",
            Prototype {
                parameters: vec![DataType::Text],
                result: DataType::Text,
            },
        );
        map.insert(
            "len",
            Prototype {
                parameters: vec![DataType::Text],
                result: DataType::Number,
            },
        );
        map.insert(
            "ascii",
            Prototype {
                parameters: vec![DataType::Text],
                result: DataType::Number,
            },
        );
        map.insert(
            "left",
            Prototype {
                parameters: vec![DataType::Text, DataType::Number],
                result: DataType::Text,
            },
        );
        map.insert(
            "datalength",
            Prototype {
                parameters: vec![DataType::Text],
                result: DataType::Number,
            },
        );
        map.insert(
            "char",
            Prototype {
                parameters: vec![DataType::Number],
                result: DataType::Text,
            },
        );
        map.insert(
            "replace",
            Prototype {
                parameters: vec![DataType::Text, DataType::Text, DataType::Text],
                result: DataType::Text
          },
        );
        map.insert(
            "substring",
            Prototype {
                parameters: vec![DataType::Text, DataType::Number, DataType::Number],
                result: DataType::Text,
            },
        );
        map.insert(
            "stuff",
            Prototype {
                parameters: vec![DataType::Text, DataType::Number, DataType::Number, DataType::Text],
                result: DataType::Text,
            },
        );

        // Date functions
        map.insert(
            "current_date",
            Prototype {
                parameters: vec![],
                result: DataType::Date,
            },
        );

        map.insert(
            "current_time",
            Prototype {
                parameters: vec![],
                result: DataType::Time,
            },
        );

        map.insert(
            "current_timestamp",
            Prototype {
                parameters: vec![],
                result: DataType::DateTime,
            },
        );
        map
    };
}

// String functions

fn text_lowercase(inputs: Vec<Value>) -> Value {
    return Value::Text(inputs[0].as_text().to_lowercase());
}

fn text_uppercase(inputs: Vec<Value>) -> Value {
    return Value::Text(inputs[0].as_text().to_uppercase());
}

fn text_reverse(inputs: Vec<Value>) -> Value {
    return Value::Text(inputs[0].as_text().chars().rev().collect::<String>());
}

fn text_replicate(inputs: Vec<Value>) -> Value {
    let str = inputs[0].as_text();
    let count = inputs[1].as_number() as usize;
    return Value::Text(str.repeat(count));
}

fn text_space(inputs: Vec<Value>) -> Value {
    let n = inputs[0].as_number() as usize;
    return Value::Text(" ".repeat(n));
}

fn text_trim(inputs: Vec<Value>) -> Value {
    return Value::Text(inputs[0].as_text().trim().to_string());
}

fn text_left_trim(inputs: Vec<Value>) -> Value {
    return Value::Text(inputs[0].as_text().trim_start().to_string());
}

fn text_right_trim(inputs: Vec<Value>) -> Value {
    return Value::Text(inputs[0].as_text().trim_end().to_string());
}

fn text_len(inputs: Vec<Value>) -> Value {
    return Value::Number(inputs[0].as_text().len() as i64);
}

fn text_ascii(inputs: Vec<Value>) -> Value {
    let text = inputs[0].as_text();
    if text.is_empty() {
        return Value::Number(0);
    }
    return Value::Number(text.chars().nth(0).unwrap() as i64);
}

fn text_left(inputs: Vec<Value>) -> Value {
    let text = inputs[0].as_text();
    if text.is_empty() {
        return Value::Text("".to_string());
    }

    let number_of_chars = inputs[1].as_number();
    if number_of_chars > text.len() as i64 {
        return Value::Text(text);
    }
    let substring = text
        .chars()
        .take(number_of_chars as usize)
        .collect::<String>();

    return Value::Text(substring);
}

fn text_datalength(inputs: Vec<Value>) -> Value {
    let text = inputs[0].as_text();
    return Value::Number(text.as_bytes().len() as i64);
}

fn text_char(inputs: Vec<Value>) -> Value {
    let code = inputs[0].as_number() as u32;
    if let Some(character) = char::from_u32(code) {
        return Value::Text(character.to_string());
    }
    return Value::Text("".to_string());
}

fn text_replace(inputs: Vec<Value>) -> Value {
    let text = inputs[0].as_text();
    let old_string = inputs[1].as_text();
    let new_string = inputs[2].as_text();

    let mut result = String::new();
    let mut end = 0;
    for (begin, matched_part) in text
        .to_lowercase()
        .match_indices(&old_string.to_lowercase())
    {
        result.push_str(&text.get(end..begin).unwrap());
        result.push_str(&new_string);
        end = begin + matched_part.len();
    }
    result.push_str(&text.get(end..text.len()).unwrap());

    return Value::Text(result);
  }

fn text_substring(inputs: Vec<Value>) -> Value {
    let text = inputs[0].as_text();
    //according to the specs, a stirng starts at position 1.
    //but in Rust, the index of a string starts from 0
    let start = inputs[1].as_number() as usize - 1;
    let length = inputs[2].as_number();

    if start > text.len() || length > text.len() as i64 {
        return Value::Text(text);
    }
    if length < 0 {
        return Value::Text("".to_string());
    }

    return Value::Text(text[start..(start + length as usize)].to_string());
}

fn text_stuff(inputs: Vec<Value>) -> Value {
    let text = inputs[0].as_text();
    let start = (inputs[1].as_number() - 1) as usize;
    let length = inputs[2].as_number() as usize;
    let new_string = inputs[3].as_text();

    if text.is_empty() {
        return Value::Text(text);
    }
    if start as usize > text.len() || length as usize > text.len() {
        return Value::Text(text);
    }

    let mut text = text.chars().collect::<Vec<_>>();
    let new_string = new_string.chars().collect::<Vec<_>>();
    text.splice(start as usize..(start + length), new_string);

    return Value::Text(text.into_iter().collect());
}


// Date functions

fn date_current_date(_inputs: Vec<Value>) -> Value {
    let time_stamp = date_utils::get_unix_timestamp_ms();
    let time = date_utils::time_stamp_to_date(time_stamp);
    return Value::Text(time);
}

fn date_current_time(_inputs: Vec<Value>) -> Value {
    let time_stamp = date_utils::get_unix_timestamp_ms();
    let date = date_utils::time_stamp_to_time(time_stamp);
    return Value::Text(date);
}

fn date_current_timestamp(_inputs: Vec<Value>) -> Value {
    let time_stamp = date_utils::get_unix_timestamp_ms();
    let date_time = date_utils::time_stamp_to_date_time(time_stamp);
    return Value::Text(date_time);
}
