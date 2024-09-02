extern crate chrono;

use gitql_core::signature::Function;
use gitql_core::signature::Signature;
use gitql_core::types::DataType;
use gitql_core::value::Value;

use std::collections::HashMap;

use chrono::DateTime;
use chrono::Datelike;
use chrono::NaiveDate;
use chrono::TimeZone;
use chrono::Timelike;
use chrono::Utc;
use chrono::Weekday;

#[inline(always)]
pub fn register_std_datetime_functions(map: &mut HashMap<&'static str, Function>) {
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
}

#[inline(always)]
pub fn register_std_datetime_function_signatures(map: &mut HashMap<&'static str, Signature>) {
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
}

pub fn date_extract_date(inputs: &[Value]) -> Value {
    let argument_type = inputs[0].data_type();
    if argument_type.is_date() {
        return inputs[0].clone();
    }
    let timestamp = inputs[0].as_date_time();
    Value::Date(timestamp)
}

pub fn date_current_date(_inputs: &[Value]) -> Value {
    let time_stamp = Utc::now().timestamp();
    Value::Date(time_stamp)
}

pub fn date_current_time(_inputs: &[Value]) -> Value {
    let time_stamp = Utc::now().timestamp();
    let datetime = DateTime::from_timestamp(time_stamp, 0).unwrap();
    let time = datetime.format("%H:%M:%S").to_string();
    Value::Time(time)
}

pub fn date_current_timestamp(_inputs: &[Value]) -> Value {
    let time_stamp = Utc::now().timestamp();
    Value::DateTime(time_stamp)
}

pub fn date_make_date(inputs: &[Value]) -> Value {
    let year = inputs[0].as_int() as i32;
    let day_of_year = inputs[1].as_int() as u32;
    let date = NaiveDate::from_yo_opt(year, day_of_year).unwrap();
    let datetime = date.and_hms_opt(0, 0, 0).unwrap();
    Value::Date(Utc.from_utc_datetime(&datetime).timestamp())
}

pub fn date_make_time(inputs: &[Value]) -> Value {
    let hour = inputs[0].as_int();
    let minute = inputs[1].as_int();
    let second = inputs[2].as_int();
    Value::Time(format!("{}:{:02}:{:02}", hour, minute, second))
}

pub fn date_day(inputs: &[Value]) -> Value {
    let date = inputs[0].as_date();
    let parsed_date = DateTime::from_timestamp(date, 0).unwrap();
    Value::Integer(parsed_date.day().into())
}

pub fn date_dayname(inputs: &[Value]) -> Value {
    let date = inputs[0].as_date();
    let parsed_date = DateTime::from_timestamp(date, 0).unwrap();
    let day_name = match parsed_date.weekday() {
        Weekday::Mon => "Monday",
        Weekday::Tue => "Tuesday",
        Weekday::Wed => "Wednesday",
        Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday",
        Weekday::Sat => "Saturday",
        Weekday::Sun => "Sunday",
    };
    Value::Text(day_name.to_string())
}

pub fn date_monthname(inputs: &[Value]) -> Value {
    let date = inputs[0].as_date();
    let parsed_date = DateTime::from_timestamp(date, 0).unwrap();
    let month_name = match parsed_date.month() {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "",
    };
    Value::Text(month_name.to_string())
}

pub fn date_hour(inputs: &[Value]) -> Value {
    let date = inputs[0].as_date_time();
    let date_time = DateTime::from_timestamp(date, 0);
    let dt = date_time.unwrap().time();
    Value::Integer(dt.hour() as i64)
}

pub fn date_minute(inputs: &[Value]) -> Value {
    let date = inputs[0].as_date_time();
    let date_time = DateTime::from_timestamp(date, 0);
    let dt = date_time.unwrap().time();
    Value::Integer(dt.minute() as i64)
}

pub fn date_is_date(inputs: &[Value]) -> Value {
    Value::Boolean(inputs[0].data_type().is_date())
}

pub fn date_day_of_week(inputs: &[Value]) -> Value {
    let date = inputs[0].as_date();
    let parsed_date = DateTime::from_timestamp(date, 0).unwrap();
    Value::Integer(parsed_date.weekday().number_from_sunday().into())
}

pub fn date_day_of_month(inputs: &[Value]) -> Value {
    let date = inputs[0].as_date();
    let parsed_date = DateTime::from_timestamp(date, 0).unwrap();
    Value::Integer(parsed_date.day().into())
}

pub fn date_day_of_year(inputs: &[Value]) -> Value {
    let date = inputs[0].as_date();
    let parsed_date = DateTime::from_timestamp(date, 0).unwrap();
    Value::Integer(parsed_date.ordinal().into())
}

pub fn date_week_of_year(inputs: &[Value]) -> Value {
    let date = inputs[0].as_date();

    let parsed_date = DateTime::from_timestamp(date, 0).unwrap();
    let native_date = parsed_date.date_naive();
    let first_day_of_year = NaiveDate::from_ymd_opt(native_date.year(), 1, 1).unwrap();
    let days_diff = native_date
        .signed_duration_since(first_day_of_year)
        .num_days();
    let week_offset = match first_day_of_year.weekday() {
        Weekday::Mon => 0,
        _ => 1,
    };

    let days_with_offset = days_diff + week_offset;
    Value::Integer(((days_with_offset / 7) as u32 + 1).into())
}

pub fn date_year_and_week(inputs: &[Value]) -> Value {
    let date = inputs[0].as_date();
    let parsed_date = DateTime::from_timestamp(date, 0).unwrap();
    let year = parsed_date.year();
    let parsed_date = DateTime::from_timestamp(date, 0).unwrap();
    let native_date = parsed_date.date_naive();
    let first_day_of_year = NaiveDate::from_ymd_opt(native_date.year(), 1, 1).unwrap();
    let days_diff = native_date
        .signed_duration_since(first_day_of_year)
        .num_days();
    let week_offset = match first_day_of_year.weekday() {
        Weekday::Mon => 0,
        _ => 1,
    };

    let days_with_offset = days_diff + week_offset;
    let week_number = (days_with_offset / 7) as u32 + 1;
    Value::Text(format!("{}{}", year, week_number))
}

pub fn date_quarter(inputs: &[Value]) -> Value {
    let date = inputs[0].as_date();
    let parsed_date = DateTime::from_timestamp(date, 0).unwrap();
    let month = parsed_date.month() as i64;
    Value::Integer((month - 1) / 3 + 1)
}

pub fn date_year(inputs: &[Value]) -> Value {
    let date = inputs[0].as_date();
    let parsed_date = DateTime::from_timestamp(date, 0).unwrap();
    Value::Integer(parsed_date.year().into())
}

pub fn date_month(inputs: &[Value]) -> Value {
    let date = inputs[0].as_date();
    let parsed_date = DateTime::from_timestamp(date, 0).unwrap();
    Value::Integer(parsed_date.month().into())
}

pub fn date_weekday(inputs: &[Value]) -> Value {
    let date = inputs[0].as_date();
    let parsed_date = DateTime::from_timestamp(date, 0).unwrap();
    Value::Integer((parsed_date.weekday().number_from_monday() - 1) as i64)
}

pub fn date_to_days(inputs: &[Value]) -> Value {
    let date = inputs[0].as_date();
    let parsed_date = DateTime::from_timestamp(date, 0).unwrap();
    let days_since_year_0 = parsed_date.ordinal0() as i64;
    let year = parsed_date.year() as i64;
    let leap_years = (year - 1) / 4 - (year - 1) / 100 + (year - 1) / 400;
    let non_leap_years = year - leap_years;
    let days = 365 * non_leap_years + 366 * leap_years + days_since_year_0;
    Value::Integer(days)
}

pub fn date_last_day(inputs: &[Value]) -> Value {
    let date = inputs[0].as_date();
    let parsed_date = DateTime::from_timestamp(date, 0).unwrap();
    let (year, month) = (parsed_date.year(), parsed_date.month());

    let curr_month_start = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let next_month_start = if month < 12 {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
    } else {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
    }
    .unwrap();
    let days_in_month = next_month_start - curr_month_start;

    let parsed_date = parsed_date.with_day(1).unwrap();
    let last_day = parsed_date + days_in_month - chrono::Duration::days(1);

    Value::Date(last_day.timestamp())
}
