use crate::array::*;
use crate::datetime::*;
use crate::general::*;
use crate::number::*;
use crate::regex::*;
use crate::text::*;

use gitql_core::signature::Function;
use gitql_core::signature::Signature;
use gitql_core::types::same_type_as_first_parameter;
use gitql_core::types::DataType;
use std::collections::HashMap;
use std::sync::OnceLock;

pub fn standard_functions() -> &'static HashMap<&'static str, Function> {
    static HASHMAP: OnceLock<HashMap<&'static str, Function>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut map: HashMap<&'static str, Function> = HashMap::new();
        // String functions
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

        // General Functions
        map.insert("isnull", general_is_null);
        map.insert("isnumeric", general_is_numeric);
        map.insert("typeof", general_type_of);
        map.insert("greatest", general_greatest);
        map.insert("least", general_least);
        map.insert("uuid", general_uuid);

        // Regex Functions
        map.insert("regexp_instr", regexp_instr);
        map.insert("regexp_like", regexp_like);
        map.insert("regexp_replace", regexp_replace);
        map.insert("regexp_substr", regexp_substr);

        // Array Functions
        map.insert("array_length", array_length);
        map.insert("array_shuffle", array_shuffle);
        map.insert("array_position", array_position);
        map
    })
}

pub fn standard_function_signatures() -> &'static HashMap<&'static str, Signature> {
    static HASHMAP: OnceLock<HashMap<&'static str, Signature>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut map: HashMap<&'static str, Signature> = HashMap::new();
        // String functions
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

        // Date functions
        map.insert(
            "date",
            Signature {
                parameters: vec![DataType::Variant(vec![DataType::Date, DataType::DateTime])],
                return_type: DataType::Date,
            },
        );
        map.insert(
            "current_date",
            Signature {
                parameters: vec![],
                return_type: DataType::Date,
            },
        );
        map.insert(
            "current_time",
            Signature {
                parameters: vec![],
                return_type: DataType::Time,
            },
        );
        map.insert(
            "current_timestamp",
            Signature {
                parameters: vec![],
                return_type: DataType::DateTime,
            },
        );
        map.insert(
            "now",
            Signature {
                parameters: vec![],
                return_type: DataType::DateTime,
            },
        );
        map.insert(
            "makedate",
            Signature {
                parameters: vec![DataType::Integer, DataType::Integer],
                return_type: DataType::Date,
            },
        );
        map.insert(
            "maketime",
            Signature {
                parameters: vec![DataType::Integer, DataType::Integer, DataType::Integer],
                return_type: DataType::Time,
            },
        );
        map.insert(
            "dayname",
            Signature {
                parameters: vec![DataType::Date],
                return_type: DataType::Text,
            },
        );
        map.insert(
            "day",
            Signature {
                parameters: vec![DataType::Date],
                return_type: DataType::Integer,
            },
        );
        map.insert(
            "monthname",
            Signature {
                parameters: vec![DataType::Date],
                return_type: DataType::Text,
            },
        );
        map.insert(
            "hour",
            Signature {
                parameters: vec![DataType::DateTime],
                return_type: DataType::Integer,
            },
        );
        map.insert(
            "minute",
            Signature {
                parameters: vec![DataType::DateTime],
                return_type: DataType::Integer,
            },
        );
        map.insert(
            "isdate",
            Signature {
                parameters: vec![DataType::Any],
                return_type: DataType::Boolean,
            },
        );
        map.insert(
            "dayofweek",
            Signature {
                parameters: vec![DataType::Date],
                return_type: DataType::Integer,
            },
        );
        map.insert(
            "dayofmonth",
            Signature {
                parameters: vec![DataType::Date],
                return_type: DataType::Integer,
            },
        );
        map.insert(
            "dayofyear",
            Signature {
                parameters: vec![DataType::Date],
                return_type: DataType::Integer,
            },
        );
        map.insert(
            "weekofyear",
            Signature {
                parameters: vec![DataType::Date],
                return_type: DataType::Integer,
            },
        );
        map.insert(
            "quarter",
            Signature {
                parameters: vec![DataType::Date],
                return_type: DataType::Integer,
            },
        );
        map.insert(
            "year",
            Signature {
                parameters: vec![DataType::Date],
                return_type: DataType::Integer,
            },
        );
        map.insert(
            "month",
            Signature {
                parameters: vec![DataType::Date],
                return_type: DataType::Integer,
            },
        );
        map.insert(
            "weekday",
            Signature {
                parameters: vec![DataType::Date],
                return_type: DataType::Integer,
            },
        );
        map.insert(
            "to_days",
            Signature {
                parameters: vec![DataType::Date],
                return_type: DataType::Integer,
            },
        );
        map.insert(
            "last_day",
            Signature {
                parameters: vec![DataType::Date],
                return_type: DataType::Date,
            },
        );
        map.insert(
            "yearweek",
            Signature {
                parameters: vec![DataType::Date],
                return_type: DataType::Text,
            },
        );
        // Numeric functions
        map.insert(
            "abs",
            Signature {
                parameters: vec![DataType::Variant(vec![DataType::Integer, DataType::Float])],
                return_type: DataType::Dynamic(same_type_as_first_parameter),
            },
        );
        map.insert(
            "pi",
            Signature {
                parameters: vec![],
                return_type: DataType::Float,
            },
        );
        map.insert(
            "floor",
            Signature {
                parameters: vec![DataType::Float],
                return_type: DataType::Integer,
            },
        );
        map.insert(
            "round",
            Signature {
                parameters: vec![
                    DataType::Float,
                    DataType::Optional(Box::new(DataType::Integer)),
                ],
                return_type: DataType::Float,
            },
        );
        map.insert(
            "square",
            Signature {
                parameters: vec![DataType::Integer],
                return_type: DataType::Integer,
            },
        );
        map.insert(
            "sin",
            Signature {
                parameters: vec![DataType::Float],
                return_type: DataType::Float,
            },
        );
        map.insert(
            "asin",
            Signature {
                parameters: vec![DataType::Float],
                return_type: DataType::Float,
            },
        );
        map.insert(
            "cos",
            Signature {
                parameters: vec![DataType::Float],
                return_type: DataType::Float,
            },
        );
        map.insert(
            "acos",
            Signature {
                parameters: vec![DataType::Float],
                return_type: DataType::Float,
            },
        );
        map.insert(
            "tan",
            Signature {
                parameters: vec![DataType::Float],
                return_type: DataType::Float,
            },
        );
        map.insert(
            "atan",
            Signature {
                parameters: vec![DataType::Float],
                return_type: DataType::Float,
            },
        );
        map.insert(
            "atn2",
            Signature {
                parameters: vec![DataType::Float, DataType::Float],
                return_type: DataType::Float,
            },
        );
        map.insert(
            "sign",
            Signature {
                parameters: vec![DataType::Variant(vec![DataType::Integer, DataType::Float])],
                return_type: DataType::Integer,
            },
        );
        map.insert(
            "mod",
            Signature {
                parameters: vec![DataType::Integer, DataType::Integer],
                return_type: DataType::Integer,
            },
        );
        map.insert(
            "rand",
            Signature {
                parameters: vec![DataType::Optional(Box::new(DataType::Float))],
                return_type: DataType::Float,
            },
        );
        // General functions
        map.insert(
            "isnull",
            Signature {
                parameters: vec![DataType::Any],
                return_type: DataType::Boolean,
            },
        );
        map.insert(
            "isnumeric",
            Signature {
                parameters: vec![DataType::Any],
                return_type: DataType::Boolean,
            },
        );
        map.insert(
            "typeof",
            Signature {
                parameters: vec![DataType::Any],
                return_type: DataType::Text,
            },
        );
        map.insert(
            "greatest",
            Signature {
                parameters: vec![
                    DataType::Any,
                    DataType::Any,
                    DataType::Varargs(Box::new(DataType::Any)),
                ],
                return_type: DataType::Any,
            },
        );
        map.insert(
            "least",
            Signature {
                parameters: vec![
                    DataType::Any,
                    DataType::Any,
                    DataType::Varargs(Box::new(DataType::Any)),
                ],
                return_type: DataType::Any,
            },
        );
        map.insert(
            "uuid",
            Signature {
                parameters: vec![],
                return_type: DataType::Text,
            },
        );

        // Regex functions
        map.insert(
            "regexp_instr",
            Signature {
                parameters: vec![DataType::Text, DataType::Text],
                return_type: DataType::Integer,
            },
        );
        map.insert(
            "regexp_like",
            Signature {
                parameters: vec![DataType::Text, DataType::Text],
                return_type: DataType::Boolean,
            },
        );
        map.insert(
            "regexp_replace",
            Signature {
                parameters: vec![DataType::Text, DataType::Text, DataType::Text],
                return_type: DataType::Text,
            },
        );
        map.insert(
            "regexp_substr",
            Signature {
                parameters: vec![DataType::Text, DataType::Text],
                return_type: DataType::Text,
            },
        );

        // Array functions
        map.insert(
            "array_length",
            Signature {
                parameters: vec![DataType::Array(Box::new(DataType::Any))],
                return_type: DataType::Integer,
            },
        );
        map.insert(
            "array_shuffle",
            Signature {
                parameters: vec![DataType::Array(Box::new(DataType::Any))],
                return_type: DataType::Dynamic(same_type_as_first_parameter),
            },
        );
        map.insert(
            "array_position",
            Signature {
                parameters: vec![DataType::Array(Box::new(DataType::Any)), DataType::Any],
                return_type: DataType::Integer,
            },
        );
        map
    })
}
