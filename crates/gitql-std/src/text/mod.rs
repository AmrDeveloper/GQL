use gitql_ast::types::any::AnyType;
use gitql_ast::types::float::FloatType;
use gitql_ast::types::integer::IntType;
use gitql_ast::types::optional::OptionType;
use gitql_ast::types::text::TextType;
use gitql_ast::types::variant::VariantType;
use gitql_core::signature::Signature;
use gitql_core::signature::StandardFunction;
use gitql_core::values::integer::IntValue;
use gitql_core::values::null::NullValue;
use gitql_core::values::text::TextValue;
use gitql_core::values::Value;

use std::collections::HashMap;

#[inline(always)]
pub fn register_std_text_functions(map: &mut HashMap<&'static str, StandardFunction>) {
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
    map.insert("to_hex", text_to_hex);
}

#[inline(always)]
pub fn register_std_text_function_signatures(map: &mut HashMap<&'static str, Signature>) {
    map.insert(
        "bin",
        Signature {
            parameters: vec![Box::new(IntType)],
            return_type: Box::new(TextType),
        },
    );
    map.insert(
        "lower",
        Signature {
            parameters: vec![Box::new(TextType)],
            return_type: Box::new(TextType),
        },
    );
    map.insert(
        "upper",
        Signature {
            parameters: vec![Box::new(TextType)],
            return_type: Box::new(TextType),
        },
    );
    map.insert(
        "reverse",
        Signature {
            parameters: vec![Box::new(TextType)],
            return_type: Box::new(TextType),
        },
    );
    map.insert(
        "replicate",
        Signature {
            parameters: vec![Box::new(TextType), Box::new(IntType)],
            return_type: Box::new(TextType),
        },
    );
    map.insert(
        "space",
        Signature {
            parameters: vec![Box::new(IntType)],
            return_type: Box::new(TextType),
        },
    );
    map.insert(
        "trim",
        Signature {
            parameters: vec![Box::new(TextType)],
            return_type: Box::new(TextType),
        },
    );
    map.insert(
        "ltrim",
        Signature {
            parameters: vec![Box::new(TextType)],
            return_type: Box::new(TextType),
        },
    );
    map.insert(
        "rtrim",
        Signature {
            parameters: vec![Box::new(TextType)],
            return_type: Box::new(TextType),
        },
    );
    map.insert(
        "len",
        Signature {
            parameters: vec![Box::new(TextType)],
            return_type: Box::new(IntType),
        },
    );
    map.insert(
        "ascii",
        Signature {
            parameters: vec![Box::new(TextType)],
            return_type: Box::new(IntType),
        },
    );
    map.insert(
        "left",
        Signature {
            parameters: vec![Box::new(TextType), Box::new(IntType)],
            return_type: Box::new(TextType),
        },
    );
    map.insert(
        "datalength",
        Signature {
            parameters: vec![Box::new(TextType)],
            return_type: Box::new(IntType),
        },
    );
    map.insert(
        "char",
        Signature {
            parameters: vec![Box::new(IntType)],
            return_type: Box::new(TextType),
        },
    );
    map.insert(
        "nchar",
        Signature {
            parameters: vec![Box::new(IntType)],
            return_type: Box::new(TextType),
        },
    );
    map.insert(
        "charindex",
        Signature {
            parameters: vec![Box::new(TextType), Box::new(TextType)],
            return_type: Box::new(IntType),
        },
    );
    map.insert(
        "replace",
        Signature {
            parameters: vec![Box::new(TextType), Box::new(TextType), Box::new(TextType)],
            return_type: Box::new(TextType),
        },
    );
    map.insert(
        "substring",
        Signature {
            parameters: vec![Box::new(TextType), Box::new(IntType), Box::new(IntType)],
            return_type: Box::new(TextType),
        },
    );
    map.insert(
        "stuff",
        Signature {
            parameters: vec![
                Box::new(TextType),
                Box::new(IntType),
                Box::new(IntType),
                Box::new(TextType),
            ],
            return_type: Box::new(TextType),
        },
    );
    map.insert(
        "right",
        Signature {
            parameters: vec![Box::new(TextType), Box::new(IntType)],
            return_type: Box::new(TextType),
        },
    );
    map.insert(
        "translate",
        Signature {
            parameters: vec![Box::new(TextType), Box::new(TextType), Box::new(TextType)],
            return_type: Box::new(TextType),
        },
    );
    map.insert(
        "soundex",
        Signature {
            parameters: vec![Box::new(TextType)],
            return_type: Box::new(TextType),
        },
    );
    map.insert(
        "concat",
        Signature {
            parameters: vec![
                Box::new(AnyType),
                Box::new(AnyType),
                Box::new(VariantType {
                    variants: vec![Box::new(AnyType)],
                }),
            ],
            return_type: Box::new(TextType),
        },
    );
    map.insert(
        "concat_ws",
        Signature {
            parameters: vec![
                Box::new(TextType),
                Box::new(AnyType),
                Box::new(AnyType),
                Box::new(VariantType {
                    variants: vec![Box::new(AnyType)],
                }),
            ],
            return_type: Box::new(TextType),
        },
    );
    map.insert(
        "unicode",
        Signature {
            parameters: vec![Box::new(TextType)],
            return_type: Box::new(IntType),
        },
    );
    map.insert(
        "strcmp",
        Signature {
            parameters: vec![Box::new(TextType), Box::new(TextType)],
            return_type: Box::new(IntType),
        },
    );
    map.insert(
        "quotename",
        Signature {
            parameters: vec![
                Box::new(TextType),
                Box::new(OptionType {
                    base: Some(Box::new(TextType)),
                }),
            ],
            return_type: Box::new(TextType),
        },
    );
    map.insert(
        "str",
        Signature {
            parameters: vec![
                Box::new(VariantType {
                    variants: vec![Box::new(IntType), Box::new(FloatType)],
                }),
                Box::new(OptionType {
                    base: Some(Box::new(IntType)),
                }),
                Box::new(OptionType {
                    base: Some(Box::new(IntType)),
                }),
            ],
            return_type: Box::new(TextType),
        },
    );
    map.insert(
        "text_to_hex",
        Signature {
            parameters: vec![Box::new(IntType)],
            return_type: Box::new(TextType),
        },
    );
}

pub fn text_bin(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let number = inputs[0].as_int().unwrap();
    Box::new(TextValue::new(format!("{number:b}")))
}

pub fn text_lowercase(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    Box::new(TextValue::new(inputs[0].as_text().unwrap().to_lowercase()))
}

pub fn text_uppercase(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    Box::new(TextValue::new(inputs[0].as_text().unwrap().to_uppercase()))
}

pub fn text_reverse(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    Box::new(TextValue::new(
        inputs[0]
            .as_text()
            .unwrap()
            .chars()
            .rev()
            .collect::<String>(),
    ))
}

pub fn text_replicate(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let str = inputs[0].as_text().unwrap();
    let count = inputs[1].as_int().unwrap() as usize;
    Box::new(TextValue::new(str.repeat(count)))
}

pub fn text_space(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let n = inputs[0].as_int().unwrap() as usize;
    Box::new(TextValue::new(" ".repeat(n)))
}

pub fn text_trim(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    Box::new(TextValue::new(
        inputs[0].as_text().unwrap().trim().to_string(),
    ))
}

pub fn text_left_trim(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    Box::new(TextValue::new(
        inputs[0].as_text().unwrap().trim_start().to_string(),
    ))
}

pub fn text_right_trim(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    Box::new(TextValue::new(
        inputs[0].as_text().unwrap().trim_end().to_string(),
    ))
}

pub fn text_len(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    Box::new(IntValue::new(inputs[0].as_text().unwrap().len() as i64))
}

pub fn text_ascii(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let text = inputs[0].as_text().unwrap();
    let value = if text.is_empty() {
        0
    } else {
        text.chars().next().unwrap() as i64
    };
    Box::new(IntValue::new(value))
}

pub fn text_left(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let text = inputs[0].as_text().unwrap();
    if text.is_empty() {
        return Box::new(TextValue::empty());
    }

    let number_of_chars = inputs[1].as_int().unwrap();
    if number_of_chars > text.len() as i64 {
        return Box::new(TextValue::new(text));
    }

    let substring = text
        .chars()
        .take(number_of_chars as usize)
        .collect::<String>();
    Box::new(TextValue { value: substring })
}

pub fn text_datalength(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let text = inputs[0].as_text().unwrap();
    Box::new(IntValue::new(text.len() as i64))
}

pub fn text_char(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let code = inputs[0].as_int().unwrap() as u32;
    if let Some(character) = char::from_u32(code) {
        Box::new(TextValue::new(character.to_string()))
    } else {
        Box::new(TextValue::empty())
    }
}

pub fn text_charindex(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let substr = inputs[0].as_text().unwrap();
    let input = inputs[1].as_text().unwrap();
    if let Some(index) = input.to_lowercase().find(&substr.to_lowercase()) {
        Box::new(IntValue::new(index as i64 + 1))
    } else {
        Box::new(IntValue::new_zero())
    }
}

pub fn text_replace(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let text = inputs[0].as_text().unwrap();
    let old_string = inputs[1].as_text().unwrap();
    let new_string = inputs[2].as_text().unwrap();

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
    Box::new(TextValue::new(result))
}

pub fn text_substring(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let text = inputs[0].as_text().unwrap();
    // According to the specs, a string starts at position 1.
    // but in Rust, the index of a string starts from 0
    let start = inputs[1].as_int().unwrap() as usize - 1;
    let length = inputs[2].as_int().unwrap();

    if start > text.len() || length > text.len() as i64 {
        return Box::new(TextValue::new(text));
    }
    if length < 0 {
        return Box::new(TextValue::empty());
    }

    // Convert it to Vec<Char> to be easy to substring with support of unicode
    let chars: Vec<char> = text.chars().collect();
    let slice = &chars[start..(start + length as usize)];
    Box::new(TextValue::new(slice.iter().collect()))
}

pub fn text_stuff(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let text = inputs[0].as_text().unwrap();
    let start = (inputs[1].as_int().unwrap() - 1) as usize;
    let length = inputs[2].as_int().unwrap() as usize;

    if text.is_empty() || start > text.len() || length > text.len() {
        return Box::new(TextValue::new(text));
    }

    let mut text = text.chars().collect::<Vec<_>>();
    let new_string = inputs[3].as_text().unwrap();
    let new_string = new_string.chars().collect::<Vec<_>>();
    text.splice(start..(start + length), new_string);
    Box::new(TextValue::new(text.into_iter().collect()))
}

pub fn text_right(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let text = inputs[0].as_text().unwrap();
    if text.is_empty() {
        return Box::new(TextValue::empty());
    }

    let number_of_chars = inputs[1].as_int().unwrap() as usize;
    if number_of_chars > text.len() {
        return Box::new(TextValue::new(text));
    }

    let text = text.as_str();
    Box::new(TextValue::new(
        text[text.len() - number_of_chars..text.len()].to_string(),
    ))
}

pub fn text_translate(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let mut text = inputs[0].as_text().unwrap();
    let characters = inputs[1].as_text().unwrap();
    let translations = inputs[2].as_text().unwrap();

    if translations.len() != characters.len() {
        return Box::new(TextValue::empty());
    }

    let translations = translations.chars().collect::<Vec<_>>();
    for (idx, letter) in characters.char_indices() {
        text = text.replace(letter, &char::to_string(&translations[idx]));
    }

    Box::new(TextValue::new(text))
}

pub fn text_unicode(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let value = if let Some(c) = inputs[0].as_text().unwrap().chars().next() {
        (c as u32).into()
    } else {
        0
    };
    Box::new(IntValue::new(value))
}

pub fn text_soundex(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let text = inputs[0].as_text().unwrap();
    if text.is_empty() {
        return Box::new(TextValue::empty());
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
                    return Box::new(TextValue::new(result));
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

    Box::new(TextValue::new(result))
}

pub fn text_concat(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let text: Vec<String> = inputs.iter().map(|v| v.to_string()).collect();
    Box::new(TextValue::new(text.concat()))
}

pub fn text_concat_ws(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let separator = inputs[0].as_text().unwrap();
    let text: Vec<String> = inputs.iter().skip(1).map(|v| v.to_string()).collect();
    let value = text.join(&separator);
    Box::new(TextValue::new(value))
}

pub fn text_strcmp(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let value = match inputs[0].as_text().cmp(&inputs[1].as_text()) {
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Equal => 2,
        std::cmp::Ordering::Greater => 0,
    };
    Box::new(IntValue::new(value))
}

pub fn text_quotename(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let str = inputs[0].as_text().unwrap();
    let quote = inputs
        .get(1)
        .map(|v| v.as_text().unwrap())
        .map(|str| str.chars().collect())
        .unwrap_or_else(|| vec!['[', ']']);

    match quote.as_slice() {
        [single] => Box::new(TextValue {
            value: format!("{single}{str}{single}"),
        }),
        [start, end] => Box::new(TextValue {
            value: format!("{start}{str}{end}"),
        }),
        _ => Box::new(NullValue),
    }
}

pub fn text_str(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let value = &inputs[0];
    let length = if inputs.len() == 3 {
        inputs[1].as_int().unwrap()
    } else {
        10
    };

    let decimals = if inputs.len() == 3 {
        inputs[2].as_int().unwrap()
    } else {
        0
    };

    if value.data_type().is_int() {
        let int_value = value.as_int().unwrap();
        let number_string = format!("{:.dec$}", int_value, dec = decimals as usize);
        if length > 0 {
            if (length as usize) < number_string.len() {
                return Box::new(TextValue::new(number_string[..length as usize].to_owned()));
            }

            return Box::new(TextValue::new(format!(
                "{:<len$}",
                number_string,
                len = length as usize
            )));
        }

        return Box::new(TextValue::new(number_string.clone()));
    }

    let float_value = value.as_float().unwrap();
    let number_string = format!("{:.dec$}", float_value, dec = decimals as usize);
    if length > 0 {
        if (length as usize) < number_string.len() {
            return Box::new(TextValue::new(number_string[..length as usize].to_owned()));
        }

        return Box::new(TextValue::new(format!(
            "{:<len$}",
            number_string,
            len = length as usize
        )));
    }

    Box::new(TextValue::new(number_string.clone()))
}

pub fn text_to_hex(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let number = inputs[0].as_int().unwrap();
    let value = format!("0x{}", number);
    Box::new(TextValue::new(value))
}
