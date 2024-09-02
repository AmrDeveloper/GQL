use gitql_core::signature::Function;
use gitql_core::signature::Signature;
use gitql_core::types::DataType;
use gitql_core::value::Value;

use std::collections::HashMap;

#[inline(always)]
pub fn register_std_text_functions(map: &mut HashMap<&'static str, Function>) {
    map.insert("bin", text_bin);
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
    map.insert("nchar", text_char);
    map.insert("charindex", text_charindex);
    map.insert("replace", text_replace);
    map.insert("substring", text_substring);
    map.insert("stuff", text_stuff);
    map.insert("right", text_right);
    map.insert("translate", text_translate);
    map.insert("soundex", text_soundex);
    map.insert("concat", text_concat);
    map.insert("concat_ws", text_concat_ws);
    map.insert("unicode", text_unicode);
    map.insert("strcmp", text_strcmp);
    map.insert("quotename", text_quotename);
    map.insert("str", text_str);
}

#[inline(always)]
pub fn register_std_text_function_signatures(map: &mut HashMap<&'static str, Signature>) {
    map.insert(
        "bin",
        Signature {
            parameters: vec![DataType::Integer],
            return_type: DataType::Text,
        },
    );
    map.insert(
        "lower",
        Signature {
            parameters: vec![DataType::Text],
            return_type: DataType::Text,
        },
    );
    map.insert(
        "upper",
        Signature {
            parameters: vec![DataType::Text],
            return_type: DataType::Text,
        },
    );
    map.insert(
        "reverse",
        Signature {
            parameters: vec![DataType::Text],
            return_type: DataType::Text,
        },
    );
    map.insert(
        "replicate",
        Signature {
            parameters: vec![DataType::Text, DataType::Integer],
            return_type: DataType::Text,
        },
    );
    map.insert(
        "space",
        Signature {
            parameters: vec![DataType::Integer],
            return_type: DataType::Text,
        },
    );
    map.insert(
        "trim",
        Signature {
            parameters: vec![DataType::Text],
            return_type: DataType::Text,
        },
    );
    map.insert(
        "ltrim",
        Signature {
            parameters: vec![DataType::Text],
            return_type: DataType::Text,
        },
    );
    map.insert(
        "rtrim",
        Signature {
            parameters: vec![DataType::Text],
            return_type: DataType::Text,
        },
    );
    map.insert(
        "len",
        Signature {
            parameters: vec![DataType::Text],
            return_type: DataType::Integer,
        },
    );
    map.insert(
        "ascii",
        Signature {
            parameters: vec![DataType::Text],
            return_type: DataType::Integer,
        },
    );
    map.insert(
        "left",
        Signature {
            parameters: vec![DataType::Text, DataType::Integer],
            return_type: DataType::Text,
        },
    );
    map.insert(
        "datalength",
        Signature {
            parameters: vec![DataType::Text],
            return_type: DataType::Integer,
        },
    );
    map.insert(
        "char",
        Signature {
            parameters: vec![DataType::Integer],
            return_type: DataType::Text,
        },
    );
    map.insert(
        "nchar",
        Signature {
            parameters: vec![DataType::Integer],
            return_type: DataType::Text,
        },
    );
    map.insert(
        "charindex",
        Signature {
            parameters: vec![DataType::Text, DataType::Text],
            return_type: DataType::Integer,
        },
    );
    map.insert(
        "replace",
        Signature {
            parameters: vec![DataType::Text, DataType::Text, DataType::Text],
            return_type: DataType::Text,
        },
    );
    map.insert(
        "substring",
        Signature {
            parameters: vec![DataType::Text, DataType::Integer, DataType::Integer],
            return_type: DataType::Text,
        },
    );
    map.insert(
        "stuff",
        Signature {
            parameters: vec![
                DataType::Text,
                DataType::Integer,
                DataType::Integer,
                DataType::Text,
            ],
            return_type: DataType::Text,
        },
    );
    map.insert(
        "right",
        Signature {
            parameters: vec![DataType::Text, DataType::Integer],
            return_type: DataType::Text,
        },
    );
    map.insert(
        "translate",
        Signature {
            parameters: vec![DataType::Text, DataType::Text, DataType::Text],
            return_type: DataType::Text,
        },
    );
    map.insert(
        "soundex",
        Signature {
            parameters: vec![DataType::Text],
            return_type: DataType::Text,
        },
    );
    map.insert(
        "concat",
        Signature {
            parameters: vec![
                DataType::Any,
                DataType::Any,
                DataType::Varargs(Box::new(DataType::Any)),
            ],
            return_type: DataType::Text,
        },
    );
    map.insert(
        "concat_ws",
        Signature {
            parameters: vec![
                DataType::Text,
                DataType::Any,
                DataType::Any,
                DataType::Varargs(Box::new(DataType::Any)),
            ],
            return_type: DataType::Text,
        },
    );
    map.insert(
        "unicode",
        Signature {
            parameters: vec![DataType::Text],
            return_type: DataType::Integer,
        },
    );
    map.insert(
        "strcmp",
        Signature {
            parameters: vec![DataType::Text, DataType::Text],
            return_type: DataType::Integer,
        },
    );
    map.insert(
        "quotename",
        Signature {
            parameters: vec![DataType::Text, DataType::Optional(Box::new(DataType::Text))],
            return_type: DataType::Text,
        },
    );
    map.insert(
        "str",
        Signature {
            parameters: vec![
                DataType::Variant(vec![DataType::Integer, DataType::Float]),
                DataType::Optional(Box::new(DataType::Integer)),
                DataType::Optional(Box::new(DataType::Integer)),
            ],
            return_type: DataType::Text,
        },
    );
}

pub fn text_bin(inputs: &[Value]) -> Value {
    let number = inputs[0].as_int();
    Value::Text(format!("{number:b}"))
}

pub fn text_lowercase(inputs: &[Value]) -> Value {
    Value::Text(inputs[0].as_text().to_lowercase())
}

pub fn text_uppercase(inputs: &[Value]) -> Value {
    Value::Text(inputs[0].as_text().to_uppercase())
}

pub fn text_reverse(inputs: &[Value]) -> Value {
    Value::Text(inputs[0].as_text().chars().rev().collect::<String>())
}

pub fn text_replicate(inputs: &[Value]) -> Value {
    let str = inputs[0].as_text();
    let count = inputs[1].as_int() as usize;
    Value::Text(str.repeat(count))
}

pub fn text_space(inputs: &[Value]) -> Value {
    let n = inputs[0].as_int() as usize;
    Value::Text(" ".repeat(n))
}

pub fn text_trim(inputs: &[Value]) -> Value {
    Value::Text(inputs[0].as_text().trim().to_string())
}

pub fn text_left_trim(inputs: &[Value]) -> Value {
    Value::Text(inputs[0].as_text().trim_start().to_string())
}

pub fn text_right_trim(inputs: &[Value]) -> Value {
    Value::Text(inputs[0].as_text().trim_end().to_string())
}

pub fn text_len(inputs: &[Value]) -> Value {
    Value::Integer(inputs[0].as_text().len() as i64)
}

pub fn text_ascii(inputs: &[Value]) -> Value {
    let text = inputs[0].as_text();
    if text.is_empty() {
        return Value::Integer(0);
    }
    Value::Integer(text.chars().next().unwrap() as i64)
}

pub fn text_left(inputs: &[Value]) -> Value {
    let text = inputs[0].as_text();
    if text.is_empty() {
        return Value::Text("".to_string());
    }

    let number_of_chars = inputs[1].as_int();
    if number_of_chars > text.len() as i64 {
        return Value::Text(text);
    }

    let substring = text
        .chars()
        .take(number_of_chars as usize)
        .collect::<String>();
    Value::Text(substring)
}

pub fn text_datalength(inputs: &[Value]) -> Value {
    let text = inputs[0].as_text();
    Value::Integer(text.as_bytes().len() as i64)
}

pub fn text_char(inputs: &[Value]) -> Value {
    let code = inputs[0].as_int() as u32;
    if let Some(character) = char::from_u32(code) {
        return Value::Text(character.to_string());
    }
    Value::Text("".to_string())
}

pub fn text_charindex(inputs: &[Value]) -> Value {
    let substr = inputs[0].as_text();
    let input = inputs[1].as_text();

    if let Some(index) = input.to_lowercase().find(&substr.to_lowercase()) {
        Value::Integer(index as i64 + 1)
    } else {
        Value::Integer(0)
    }
}

pub fn text_replace(inputs: &[Value]) -> Value {
    let text = inputs[0].as_text();
    let old_string = inputs[1].as_text();
    let new_string = inputs[2].as_text();

    let mut result = String::new();
    let mut end = 0;
    for (begin, matched_part) in text
        .to_lowercase()
        .match_indices(&old_string.to_lowercase())
    {
        result.push_str(text.get(end..begin).unwrap());
        result.push_str(&new_string);
        end = begin + matched_part.len();
    }

    result.push_str(text.get(end..text.len()).unwrap());
    Value::Text(result)
}

pub fn text_substring(inputs: &[Value]) -> Value {
    let text = inputs[0].as_text();
    // According to the specs, a string starts at position 1.
    // but in Rust, the index of a string starts from 0
    let start = inputs[1].as_int() as usize - 1;
    let length = inputs[2].as_int();

    if start > text.len() || length > text.len() as i64 {
        return Value::Text(text);
    }
    if length < 0 {
        return Value::Text("".to_string());
    }

    // Convert it to Vec<Char> to be easy to substring with support of unicode
    let chars: Vec<char> = text.chars().collect();
    let slice = &chars[start..(start + length as usize)];
    Value::Text(slice.iter().collect())
}

pub fn text_stuff(inputs: &[Value]) -> Value {
    let text = inputs[0].as_text();
    let start = (inputs[1].as_int() - 1) as usize;
    let length = inputs[2].as_int() as usize;
    let new_string = inputs[3].as_text();

    if text.is_empty() {
        return Value::Text(text);
    }

    if start > text.len() || length > text.len() {
        return Value::Text(text);
    }

    let mut text = text.chars().collect::<Vec<_>>();
    let new_string = new_string.chars().collect::<Vec<_>>();
    text.splice(start..(start + length), new_string);
    Value::Text(text.into_iter().collect())
}

pub fn text_right(inputs: &[Value]) -> Value {
    let text = inputs[0].as_text();
    if text.is_empty() {
        return Value::Text("".to_string());
    }

    let number_of_chars = inputs[1].as_int() as usize;
    if number_of_chars > text.len() {
        return Value::Text(text);
    }

    let text = text.as_str();
    Value::Text(text[text.len() - number_of_chars..text.len()].to_string())
}

pub fn text_translate(inputs: &[Value]) -> Value {
    let mut text = inputs[0].as_text();
    let characters = inputs[1].as_text();
    let translations = inputs[2].as_text();

    if translations.len() != characters.len() {
        return Value::Text("".to_string());
    }

    let translations = translations.chars().collect::<Vec<_>>();
    for (idx, letter) in characters.char_indices() {
        text = text.replace(letter, &char::to_string(&translations[idx]));
    }

    Value::Text(text)
}

pub fn text_unicode(inputs: &[Value]) -> Value {
    if let Some(c) = inputs[0].as_text().chars().next() {
        return Value::Integer((c as u32).into());
    }
    Value::Integer(0)
}

pub fn text_soundex(inputs: &[Value]) -> Value {
    let text = inputs[0].as_text();
    if text.is_empty() {
        return Value::Text("".to_string());
    }

    let mut result = String::from(text.chars().next().unwrap());

    for (idx, letter) in text.char_indices() {
        if idx != 0 {
            let letter = letter.to_ascii_uppercase();
            if !matches!(letter, 'A' | 'E' | 'I' | 'O' | 'U' | 'H' | 'W' | 'Y') {
                let int = match letter {
                    'B' | 'F' | 'P' | 'V' => 1,
                    'C' | 'G' | 'J' | 'K' | 'Q' | 'S' | 'X' | 'Z' => 2,
                    'D' | 'T' => 3,
                    'L' => 4,
                    'M' | 'N' => 5,
                    'R' => 6,
                    _ => 0,
                };
                result.push_str(&int.to_string());

                if result.len() == 4 {
                    return Value::Text(result);
                }
            }
        }
    }

    if result.len() < 4 {
        let diff = 4 - result.len();
        for _i in 0..diff {
            result.push_str(&0.to_string());
        }
    }

    Value::Text(result)
}

pub fn text_concat(inputs: &[Value]) -> Value {
    let text: Vec<String> = inputs.iter().map(|v| v.to_string()).collect();
    Value::Text(text.concat())
}

pub fn text_concat_ws(inputs: &[Value]) -> Value {
    let separator = inputs[0].as_text();
    let text: Vec<String> = inputs.iter().skip(1).map(|v| v.to_string()).collect();
    Value::Text(text.join(&separator))
}

pub fn text_strcmp(inputs: &[Value]) -> Value {
    Value::Integer(match inputs[0].as_text().cmp(&inputs[1].as_text()) {
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Equal => 2,
        std::cmp::Ordering::Greater => 0,
    })
}

pub fn text_quotename(inputs: &[Value]) -> Value {
    let str = inputs[0].as_text();
    let quote = inputs
        .get(1)
        .map(Value::as_text)
        .map(|str| str.chars().collect())
        .unwrap_or_else(|| vec!['[', ']']);

    match quote.as_slice() {
        [single] => Value::Text(format!("{single}{str}{single}")),
        [start, end] => Value::Text(format!("{start}{str}{end}")),
        _ => Value::Null,
    }
}

pub fn text_str(inputs: &[Value]) -> Value {
    let value = &inputs[0];
    let length = if inputs.len() == 3 {
        inputs[1].as_int()
    } else {
        10
    };

    let decimals = if inputs.len() == 3 {
        inputs[2].as_int()
    } else {
        0
    };

    if value.data_type().is_int() {
        let int_value = value.as_int();
        let number_string = format!("{:.dec$}", int_value, dec = decimals as usize);
        if length > 0 {
            if (length as usize) < number_string.len() {
                return Value::Text(number_string[..length as usize].to_owned());
            } else {
                return Value::Text(format!("{:<len$}", number_string, len = length as usize));
            }
        }
        return Value::Text(number_string.clone());
    }

    let float_value = value.as_float();
    let number_string = format!("{:.dec$}", float_value, dec = decimals as usize);
    if length > 0 {
        if (length as usize) < number_string.len() {
            return Value::Text(number_string[..length as usize].to_owned());
        } else {
            return Value::Text(format!("{:<len$}", number_string, len = length as usize));
        }
    }

    Value::Text(number_string.clone())
}
