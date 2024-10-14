use crate::values::base::Value;
use crate::values::boolean::BoolValue;
use crate::values::date::DateValue;
use crate::values::datetime::DateTimeValue;
use crate::values::null::NullValue;
use crate::values::time::TimeValue;

pub fn string_literal_to_time(literal: &str) -> Box<dyn Value> {
    Box::new(TimeValue {
        value: literal.to_string(),
    })
}

pub fn string_literal_to_date(literal: &str) -> Box<dyn Value> {
    let date_time = chrono::NaiveDate::parse_from_str(literal, "%Y-%m-%d").ok();
    let timestamp = if let Some(date) = date_time {
        let zero_time = chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap();
        date.and_time(zero_time).and_utc().timestamp()
    } else {
        0
    };

    Box::new(DateValue { value: timestamp })
}

pub fn string_literal_to_date_time(literal: &str) -> Box<dyn Value> {
    let date_time_format = if literal.contains('.') {
        "%Y-%m-%d %H:%M:%S%.3f"
    } else {
        "%Y-%m-%d %H:%M:%S"
    };

    let date_time = chrono::NaiveDateTime::parse_from_str(literal, date_time_format);
    if date_time.is_err() {
        return Box::new(DateTimeValue { value: 0 });
    }

    let timestamp = date_time.ok().unwrap().and_utc().timestamp();
    Box::new(DateTimeValue { value: timestamp })
}

pub fn string_literal_to_boolean(literal: &str) -> Box<dyn Value> {
    match literal {
        // True values literal
        "t" => Box::new(BoolValue { value: true }),
        "true" => Box::new(BoolValue { value: true }),
        "y" => Box::new(BoolValue { value: true }),
        "yes" => Box::new(BoolValue { value: true }),
        "1" => Box::new(BoolValue { value: true }),
        // False values literal
        "f" => Box::new(BoolValue { value: false }),
        "false" => Box::new(BoolValue { value: false }),
        "n" => Box::new(BoolValue { value: false }),
        "no" => Box::new(BoolValue { value: false }),
        "0" => Box::new(BoolValue { value: false }),
        // Invalid value, must be unreachable
        _ => Box::new(NullValue),
    }
}
