use crate::date_utils as GitQLDate;
use crate::regex_utils as GitQLRegex;
use crate::types::same_type_as_first_parameter;
use crate::types::DataType;
use crate::value::Value;

use lazy_static::lazy_static;
use rand::distributions::Uniform;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::Rem;

type Function = fn(&[Value]) -> Value;

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

        // Date functions
        map.insert("date", date_extract_date);
        map.insert("current_date", date_current_date);
        map.insert("current_time", date_current_time);
        map.insert("current_timestamp", date_current_timestamp);
        map.insert("now", date_current_timestamp);
        map.insert("makedate", date_make_date);
        map.insert("maketime", date_make_time);
        map.insert("day", date_day);
        map.insert("dayname", date_dayname);
        map.insert("monthname", date_monthname);
        map.insert("hour", date_hour);
        map.insert("minute", date_minute);
        map.insert("isdate", date_is_date);
        map.insert("dayofweek", date_day_of_week);
        map.insert("dayofmonth", date_day_of_month);
        map.insert("dayofyear", date_day_of_year);
        map.insert("weekofyear", date_week_of_year);
        map.insert("quarter", date_quarter);
        map.insert("year", date_year);
        map.insert("month", date_month);
        map.insert("weekday", date_weekday);
        map.insert("to_days", date_to_days);
        map.insert("last_day", date_last_day);
        map.insert("yearweek", date_year_and_week);

        // Numeric functions
        map.insert("abs", numeric_abs);
        map.insert("pi", numeric_pi);
        map.insert("floor", numeric_floor);
        map.insert("round", numeric_round);
        map.insert("square", numeric_square);
        map.insert("sin", numeric_sin);
        map.insert("asin", numeric_asin);
        map.insert("cos", numeric_cos);
        map.insert("acos", numeric_acos);
        map.insert("tan", numeric_tan);
        map.insert("atan", numeric_atan);
        map.insert("atn2", numeric_atn2);
        map.insert("sign", numeric_sign);
        map.insert("mod", numeric_mod);
        map.insert("rand", numeric_rand);

        // Other Functions
        map.insert("isnull", general_is_null);
        map.insert("isnumeric", general_is_numeric);
        map.insert("typeof", general_type_of);
        map.insert("greatest", general_greatest);
        map.insert("least", general_least);

        // Regex Functions
        map.insert("regexp_instr", regexp_instr);
        map.insert("regexp_like", regexp_like);
        map.insert("regexp_replace", regexp_replace);
        map.insert("regexp_substr", regexp_substr);
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
                parameters: vec![DataType::Text, DataType::Integer],
                result: DataType::Text,
            },
        );
        map.insert(
            "space",
            Prototype {
                parameters: vec![DataType::Integer],
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
                result: DataType::Integer,
            },
        );
        map.insert(
            "ascii",
            Prototype {
                parameters: vec![DataType::Text],
                result: DataType::Integer,
            },
        );
        map.insert(
            "left",
            Prototype {
                parameters: vec![DataType::Text, DataType::Integer],
                result: DataType::Text,
            },
        );
        map.insert(
            "datalength",
            Prototype {
                parameters: vec![DataType::Text],
                result: DataType::Integer,
            },
        );
        map.insert(
            "char",
            Prototype {
                parameters: vec![DataType::Integer],
                result: DataType::Text,
            },
        );
        map.insert(
            "nchar",
            Prototype {
                parameters: vec![DataType::Integer],
                result: DataType::Text,
            },
        );
        map.insert(
            "charindex",
            Prototype {
                parameters: vec![DataType::Text, DataType::Text],
                result: DataType::Integer,
            }
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
                parameters: vec![DataType::Text, DataType::Integer, DataType::Integer],
                result: DataType::Text,
            },
        );
        map.insert(
            "stuff",
            Prototype {
                parameters: vec![DataType::Text, DataType::Integer, DataType::Integer, DataType::Text],
                result: DataType::Text,
            },
        );
        map.insert(
            "right",
            Prototype {
                parameters: vec![DataType::Text, DataType::Integer],
                result: DataType::Text
             },
        );
        map.insert(
            "translate",
            Prototype {
                parameters: vec![DataType::Text, DataType::Text, DataType::Text],
                result: DataType::Text
             },
        );
        map.insert(
            "soundex",
            Prototype {
                parameters: vec![DataType::Text],
                result: DataType::Text,
            },
        );
        map.insert(
            "concat",
            Prototype {
                parameters: vec![DataType::Any, DataType::Any, DataType::Varargs(Box::new(DataType::Any))],
                result: DataType::Text
             },
        );
        map.insert(
            "concat_ws",
            Prototype {
                parameters: vec![DataType::Text, DataType::Any, DataType::Any, DataType::Varargs(Box::new(DataType::Any))],
                result: DataType::Text
             },
        );
        map.insert(
            "unicode",
            Prototype {
                parameters: vec![DataType::Text],
                result: DataType::Integer
             },
        );
        map.insert("strcmp", Prototype { parameters: vec![DataType::Text, DataType::Text], result: DataType::Integer });
        map.insert(
            "quotename",
            Prototype {
                parameters: vec![DataType::Text, DataType::Optional(Box::new(DataType::Text))],
                result: DataType::Text
            }
        );

        // Date functions
        map.insert(
            "date",
            Prototype {
                parameters: vec![DataType::Variant(vec![DataType::Date, DataType::DateTime])],
                result: DataType::Date,
            },
        );
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
        map.insert(
            "now",
            Prototype {
                parameters: vec![],
                result: DataType::DateTime,
            },
        );
        map.insert(
            "makedate",
            Prototype {
                parameters: vec![DataType::Integer, DataType::Integer],
                result: DataType::Date,
            },
        );
        map.insert(
            "maketime",
            Prototype {
                parameters: vec![DataType::Integer, DataType::Integer, DataType::Integer],
                result: DataType::Time,
            },
        );
        map.insert(
            "dayname",
            Prototype {
                parameters: vec![DataType::Date],
                result: DataType::Text,
            }
        );
        map.insert(
            "day",
            Prototype {
                parameters: vec![DataType::Date],
                result: DataType::Integer,
            }
        );
        map.insert(
            "monthname",
            Prototype {
                parameters: vec![DataType::Date],
                result: DataType::Text,
            }
        );
        map.insert(
            "hour",
            Prototype {
                parameters: vec![DataType::DateTime],
                result: DataType::Integer,
            }
        );
        map.insert(
            "minute",
            Prototype {
                parameters: vec![DataType::DateTime],
                result: DataType::Integer,
            }
        );
        map.insert(
            "isdate",
            Prototype {
                parameters: vec![DataType::Any],
                result: DataType::Boolean,
            }
        );
        map.insert(
            "dayofweek",
            Prototype {
                parameters: vec![DataType::Date],
                result: DataType::Integer,
            }
        );
        map.insert(
            "dayofmonth",
            Prototype {
                parameters: vec![DataType::Date],
                result: DataType::Integer,
            }
        );
        map.insert(
            "dayofyear",
            Prototype {
                parameters: vec![DataType::Date],
                result: DataType::Integer,
            }
        );
        map.insert(
            "weekofyear",
            Prototype {
                parameters: vec![DataType::Date],
                result: DataType::Integer,
            }
        );
        map.insert(
            "quarter",
            Prototype {
                parameters: vec![DataType::Date],
                result: DataType::Integer,
            }
        );
        map.insert(
            "year",
            Prototype {
                parameters: vec![DataType::Date],
                result: DataType::Integer,
            }
        );
        map.insert(
            "month",
            Prototype {
                parameters: vec![DataType::Date],
                result: DataType::Integer,
            }
        );
        map.insert(
            "weekday",
            Prototype {
                parameters: vec![DataType::Date],
                result: DataType::Integer,
            }
        );
        map.insert(
            "to_days",
            Prototype {
                parameters: vec![DataType::Date],
                result: DataType::Integer,
            }
        );
        map.insert(
            "last_day",
            Prototype {
                parameters: vec![DataType::Date],
                result: DataType::Date,
            }
        );
        map.insert(
            "yearweek",
            Prototype {
                parameters: vec![DataType::Date],
                result: DataType::Text,
            }
        );
        // Numeric functions
        map.insert(
            "abs",
            Prototype {
                parameters: vec![DataType::Variant(vec![DataType::Integer, DataType::Float])],
                result: DataType::Dynamic(same_type_as_first_parameter),
            },
        );
        map.insert(
            "pi",
            Prototype {
                parameters: vec![],
                result: DataType::Float,
            },
        );
        map.insert(
            "floor",
            Prototype {
                parameters: vec![DataType::Float],
                result: DataType::Integer,
            },
        );
        map.insert(
            "round",
            Prototype {
                parameters: vec![DataType::Float, DataType::Optional(Box::new(DataType::Integer))],
                result: DataType::Float,
            },
        );
        map.insert(
            "square",
            Prototype {
                parameters: vec![DataType::Integer],
                result: DataType::Integer,
            },
        );
        map.insert(
            "sin",
            Prototype {
                parameters: vec![DataType::Float],
                result: DataType::Float,
            },
        );
        map.insert(
            "asin",
            Prototype {
                parameters: vec![DataType::Float],
                result: DataType::Float,
            },
        );
        map.insert(
            "cos",
            Prototype {
                parameters: vec![DataType::Float],
                result: DataType::Float,
            },
        );
        map.insert(
            "acos",
            Prototype {
                parameters: vec![DataType::Float],
                result: DataType::Float,
            },
        );
        map.insert(
            "tan",
            Prototype {
                parameters: vec![DataType::Float],
                result: DataType::Float,
            },
        );
        map.insert(
            "atan",
            Prototype {
                parameters: vec![DataType::Float],
                result: DataType::Float,
            },
        );
        map.insert(
            "atn2",
            Prototype {
                parameters: vec![DataType::Float, DataType::Float],
                result: DataType::Float,
            },
        );
        map.insert(
            "sign",
            Prototype {
                parameters: vec![DataType::Variant(vec![DataType::Integer, DataType::Float])],
                result: DataType::Integer,
            },
        );
        map.insert(
            "mod",
            Prototype {
                parameters: vec![DataType::Integer, DataType::Integer],
                result: DataType::Integer,
            },
        );
        map.insert(
            "rand",
            Prototype {
                parameters: vec![DataType::Optional(Box::new(DataType::Float))],
                result: DataType::Float,
            },
        );
        // General functions
        map.insert(
            "isnull",
            Prototype {
                parameters: vec![DataType::Any],
                result: DataType::Boolean,
            },
        );
        map.insert(
            "isnumeric",
            Prototype {
                parameters: vec![DataType::Any],
                result: DataType::Boolean,
            },
        );
        map.insert(
            "typeof",
            Prototype {
                parameters: vec![DataType::Any],
                result: DataType::Text,
            },
        );
        map.insert(
            "greatest",
            Prototype {
                parameters: vec![DataType::Any, DataType::Any, DataType::Varargs(Box::new(DataType::Any))],
                result: DataType::Any
             },
        );
        map.insert(
            "least",
            Prototype {
                parameters: vec![DataType::Any, DataType::Any, DataType::Varargs(Box::new(DataType::Any))],
                result: DataType::Any
             },
        );

        // Regex functions
        map.insert(
            "regexp_instr",
            Prototype {
                parameters: vec![DataType::Text, DataType::Text],
                result: DataType::Integer,
            },
        );
        map.insert(
            "regexp_like",
            Prototype {
                parameters: vec![DataType::Text, DataType::Text],
                result: DataType::Boolean,
            },
        );
        map.insert(
            "regexp_replace",
            Prototype {
                parameters: vec![DataType::Text, DataType::Text, DataType::Text],
                result: DataType::Text,
            },
        );
        map.insert(
            "regexp_substr",
            Prototype {
                parameters: vec![DataType::Text, DataType::Text],
                result: DataType::Text,
            },
        );
        map
    };
}

// String functions

fn text_lowercase(inputs: &[Value]) -> Value {
    Value::Text(inputs[0].as_text().to_lowercase())
}

fn text_uppercase(inputs: &[Value]) -> Value {
    Value::Text(inputs[0].as_text().to_uppercase())
}

fn text_reverse(inputs: &[Value]) -> Value {
    Value::Text(inputs[0].as_text().chars().rev().collect::<String>())
}

fn text_replicate(inputs: &[Value]) -> Value {
    let str = inputs[0].as_text();
    let count = inputs[1].as_int() as usize;
    Value::Text(str.repeat(count))
}

fn text_space(inputs: &[Value]) -> Value {
    let n = inputs[0].as_int() as usize;
    Value::Text(" ".repeat(n))
}

fn text_trim(inputs: &[Value]) -> Value {
    Value::Text(inputs[0].as_text().trim().to_string())
}

fn text_left_trim(inputs: &[Value]) -> Value {
    Value::Text(inputs[0].as_text().trim_start().to_string())
}

fn text_right_trim(inputs: &[Value]) -> Value {
    Value::Text(inputs[0].as_text().trim_end().to_string())
}

fn text_len(inputs: &[Value]) -> Value {
    Value::Integer(inputs[0].as_text().len() as i64)
}

fn text_ascii(inputs: &[Value]) -> Value {
    let text = inputs[0].as_text();
    if text.is_empty() {
        return Value::Integer(0);
    }
    Value::Integer(text.chars().next().unwrap() as i64)
}

fn text_left(inputs: &[Value]) -> Value {
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

fn text_datalength(inputs: &[Value]) -> Value {
    let text = inputs[0].as_text();
    Value::Integer(text.as_bytes().len() as i64)
}

fn text_char(inputs: &[Value]) -> Value {
    let code = inputs[0].as_int() as u32;
    if let Some(character) = char::from_u32(code) {
        return Value::Text(character.to_string());
    }
    Value::Text("".to_string())
}

fn text_charindex(inputs: &[Value]) -> Value {
    let substr = inputs[0].as_text();
    let input = inputs[1].as_text();

    if let Some(index) = input.to_lowercase().find(&substr.to_lowercase()) {
        Value::Integer(index as i64 + 1)
    } else {
        Value::Integer(0)
    }
}

fn text_replace(inputs: &[Value]) -> Value {
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

fn text_substring(inputs: &[Value]) -> Value {
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

fn text_stuff(inputs: &[Value]) -> Value {
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

fn text_right(inputs: &[Value]) -> Value {
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

fn text_translate(inputs: &[Value]) -> Value {
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

fn text_unicode(inputs: &[Value]) -> Value {
    if let Some(c) = inputs[0].as_text().chars().next() {
        return Value::Integer((c as u32).into());
    }
    Value::Integer(0)
}

fn text_soundex(inputs: &[Value]) -> Value {
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

fn text_concat(inputs: &[Value]) -> Value {
    let text: Vec<String> = inputs.iter().map(|v| v.to_string()).collect();
    Value::Text(text.concat())
}

fn text_concat_ws(inputs: &[Value]) -> Value {
    let separator = inputs[0].as_text();
    let text: Vec<String> = inputs.iter().skip(1).map(|v| v.to_string()).collect();
    Value::Text(text.join(&separator))
}

fn text_strcmp(inputs: &[Value]) -> Value {
    Value::Integer(match inputs[0].as_text().cmp(&inputs[1].as_text()) {
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Equal => 2,
        std::cmp::Ordering::Greater => 0,
    })
}

fn text_quotename(inputs: &[Value]) -> Value {
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

// Date functions
fn date_extract_date(inputs: &[Value]) -> Value {
    let argument_type = inputs[0].data_type();
    if argument_type.is_date() {
        return inputs[0].clone();
    }
    let timestamp = inputs[0].as_date_time();
    Value::Date(timestamp)
}

fn date_current_date(_inputs: &[Value]) -> Value {
    let time_stamp = GitQLDate::get_unix_timestamp_ms();
    Value::Date(time_stamp)
}

fn date_current_time(_inputs: &[Value]) -> Value {
    let time_stamp = GitQLDate::get_unix_timestamp_ms();
    let time = GitQLDate::time_stamp_to_time(time_stamp);
    Value::Time(time)
}

fn date_current_timestamp(_inputs: &[Value]) -> Value {
    let time_stamp = GitQLDate::get_unix_timestamp_ms();
    Value::DateTime(time_stamp)
}

fn date_make_date(inputs: &[Value]) -> Value {
    let year = inputs[0].as_int() as i32;
    let day_of_year = inputs[1].as_int() as u32;
    let time_stamp = GitQLDate::time_stamp_from_year_and_day(year, day_of_year);
    Value::Date(time_stamp)
}

fn date_make_time(inputs: &[Value]) -> Value {
    let hour = inputs[0].as_int();
    let minute = inputs[1].as_int();
    let second = inputs[2].as_int();
    Value::Time(format!("{}:{:02}:{:02}", hour, minute, second))
}

fn date_day(inputs: &[Value]) -> Value {
    let date = inputs[0].as_date();
    Value::Integer(GitQLDate::date_to_day_number_in_month(date).into())
}

fn date_dayname(inputs: &[Value]) -> Value {
    let date = inputs[0].as_date();
    let date_str = GitQLDate::date_to_day_name(date);
    Value::Text(date_str)
}

fn date_monthname(inputs: &[Value]) -> Value {
    let date = inputs[0].as_date();
    let month_str = GitQLDate::date_to_month_name(date);
    Value::Text(month_str)
}

fn date_hour(inputs: &[Value]) -> Value {
    let date = inputs[0].as_date_time();
    let hour = GitQLDate::date_time_to_hour(date);
    Value::Integer(hour)
}

fn date_minute(inputs: &[Value]) -> Value {
    let date = inputs[0].as_date_time();
    let minute = GitQLDate::date_time_to_minute(date);
    Value::Integer(minute)
}

fn date_is_date(inputs: &[Value]) -> Value {
    Value::Boolean(inputs[0].data_type().is_date())
}

fn date_day_of_week(inputs: &[Value]) -> Value {
    let date = inputs[0].as_date();
    Value::Integer(GitQLDate::date_to_day_number_in_week(date).into())
}

fn date_day_of_month(inputs: &[Value]) -> Value {
    let date = inputs[0].as_date();
    Value::Integer(GitQLDate::date_to_day_number_in_month(date).into())
}

fn date_day_of_year(inputs: &[Value]) -> Value {
    let date = inputs[0].as_date();
    Value::Integer(GitQLDate::date_to_day_number_in_year(date).into())
}

fn date_week_of_year(inputs: &[Value]) -> Value {
    let date = inputs[0].as_date();
    Value::Integer(GitQLDate::date_to_week_number_in_year(date).into())
}

fn date_year_and_week(inputs: &[Value]) -> Value {
    let date = inputs[0].as_date();
    let year = GitQLDate::date_to_year(date);
    let week_number = GitQLDate::date_to_week_number_in_year(date);
    Value::Text(format!("{}{}", year, week_number))
}

fn date_quarter(inputs: &[Value]) -> Value {
    let date = inputs[0].as_date();
    Value::Integer(GitQLDate::date_to_quarter_index(date))
}

fn date_year(inputs: &[Value]) -> Value {
    let date = inputs[0].as_date();
    Value::Integer(GitQLDate::date_to_year(date).into())
}

fn date_month(inputs: &[Value]) -> Value {
    let date = inputs[0].as_date();
    Value::Integer(GitQLDate::date_to_month(date).into())
}

fn date_weekday(inputs: &[Value]) -> Value {
    let date = inputs[0].as_date();
    Value::Integer(GitQLDate::date_to_weekday(date).into())
}

fn date_to_days(inputs: &[Value]) -> Value {
    let date = inputs[0].as_date();
    Value::Integer(GitQLDate::date_to_days_count(date))
}

fn date_last_day(inputs: &[Value]) -> Value {
    let date = inputs[0].as_date();
    Value::Date(GitQLDate::date_last_day(date))
}

// Numeric functions

fn numeric_abs(inputs: &[Value]) -> Value {
    let input_type = inputs[0].data_type();
    if input_type.is_float() {
        return Value::Float(inputs[0].as_float().abs());
    }
    Value::Integer(inputs[0].as_int().abs())
}

fn numeric_pi(_inputs: &[Value]) -> Value {
    let pi = std::f64::consts::PI;
    Value::Float(pi)
}

fn numeric_floor(inputs: &[Value]) -> Value {
    let float_value = inputs[0].as_float();
    Value::Integer(float_value.floor() as i64)
}

fn numeric_round(inputs: &[Value]) -> Value {
    let number = inputs[0].as_float();
    let decimal_places = if inputs.len() == 2 {
        inputs[1].as_int()
    } else {
        0
    };
    let multiplier = 10_f64.powi(decimal_places.try_into().unwrap());
    let result = (number * multiplier).round() / multiplier;
    Value::Float(result)
}

fn numeric_square(inputs: &[Value]) -> Value {
    let int_value = inputs[0].as_int();
    Value::Integer(int_value * int_value)
}

fn numeric_sin(inputs: &[Value]) -> Value {
    let float_value = inputs[0].as_float();
    Value::Float(f64::sin(float_value))
}

fn numeric_asin(inputs: &[Value]) -> Value {
    let float_value = inputs[0].as_float();
    Value::Float(f64::asin(float_value))
}

fn numeric_cos(inputs: &[Value]) -> Value {
    let float_value = inputs[0].as_float();
    Value::Float(f64::cos(float_value))
}

fn numeric_acos(inputs: &[Value]) -> Value {
    let float_value = inputs[0].as_float();
    Value::Float(f64::acos(float_value))
}

fn numeric_tan(inputs: &[Value]) -> Value {
    let float_value = inputs[0].as_float();
    Value::Float(f64::tan(float_value))
}

fn numeric_atan(inputs: &[Value]) -> Value {
    let float_value = inputs[0].as_float();
    Value::Float(f64::atan(float_value))
}

fn numeric_atn2(inputs: &[Value]) -> Value {
    let first = inputs[0].as_float();
    let other = inputs[1].as_float();
    Value::Float(f64::atan2(first, other))
}

fn numeric_sign(inputs: &[Value]) -> Value {
    let value = &inputs[0];
    if value.data_type().is_int() {
        let int_value = value.as_int();
        return Value::Integer(int_value.signum());
    }

    let float_value = value.as_float();
    if float_value == 0.0 {
        Value::Integer(0)
    } else if float_value > 0.0 {
        Value::Integer(1)
    } else {
        Value::Integer(-1)
    }
}

fn numeric_mod(inputs: &[Value]) -> Value {
    let other = &inputs[1];
    if other.as_int() == 0 {
        return Value::Null;
    }

    let first = &inputs[0];
    Value::Integer(first.as_int().rem(other.as_int()))
}

fn numeric_rand(inputs: &[Value]) -> Value {
    let mut rng: StdRng = match inputs.first() {
        Some(s) => SeedableRng::seed_from_u64(s.as_int().try_into().unwrap()),
        None => SeedableRng::from_entropy(),
    };
    Value::Float(rng.sample(Uniform::from(0.0..1.0)))
}

// General functions

fn general_is_null(inputs: &[Value]) -> Value {
    Value::Boolean(inputs[0].data_type() == DataType::Null)
}

fn general_is_numeric(inputs: &[Value]) -> Value {
    let input_type = inputs[0].data_type();
    Value::Boolean(input_type.is_number())
}

fn general_type_of(inputs: &[Value]) -> Value {
    let input_type = inputs[0].data_type();
    Value::Text(input_type.to_string())
}

fn general_greatest(inputs: &[Value]) -> Value {
    let mut max = &inputs[0];

    for value in inputs.iter().skip(1) {
        if max.compare(value) == Ordering::Greater {
            max = value;
        }
    }

    max.to_owned()
}

fn general_least(inputs: &[Value]) -> Value {
    let mut least = &inputs[0];

    for value in inputs.iter().skip(1) {
        if least.compare(value) == Ordering::Less {
            least = value;
        }
    }

    least.to_owned()
}

// Regex functions

fn regexp_instr(inputs: &[Value]) -> Value {
    let input = inputs[0].as_text();
    let pattern = inputs[1].as_text();
    let position = GitQLRegex::regexp_pattern_position(&input, &pattern);
    Value::Integer(position)
}

fn regexp_like(inputs: &[Value]) -> Value {
    let input = inputs[0].as_text();
    let pattern = inputs[1].as_text();
    Value::Boolean(GitQLRegex::regex_is_match(&input, &pattern))
}

fn regexp_replace(inputs: &[Value]) -> Value {
    let input = inputs[0].as_text();
    let pattern = inputs[1].as_text();
    let replacement = inputs[2].as_text();
    Value::Text(GitQLRegex::regex_replace(&input, &pattern, &replacement))
}

fn regexp_substr(inputs: &[Value]) -> Value {
    let input = inputs[0].as_text();
    let pattern = inputs[1].as_text();
    Value::Text(GitQLRegex::regex_substr(&input, &pattern))
}
