use std::collections::HashMap;

extern crate chrono;
use chrono::DateTime;
use chrono::Datelike;
use chrono::NaiveDate;
use chrono::TimeZone;
use chrono::Timelike;
use chrono::Utc;
use chrono::Weekday;

use gitql_ast::types::any::AnyType;
use gitql_ast::types::boolean::BoolType;
use gitql_ast::types::date::DateType;
use gitql_ast::types::datetime::DateTimeType;
use gitql_ast::types::integer::IntType;
use gitql_ast::types::interval::IntervalType;
use gitql_ast::types::text::TextType;
use gitql_ast::types::time::TimeType;
use gitql_ast::types::variant::VariantType;
use gitql_core::signature::Signature;
use gitql_core::signature::StandardFunction;
use gitql_core::values::base::Value;
use gitql_core::values::boolean::BoolValue;
use gitql_core::values::date::DateValue;
use gitql_core::values::datetime::DateTimeValue;
use gitql_core::values::integer::IntValue;
use gitql_core::values::interval::IntervalValue;
use gitql_core::values::text::TextValue;
use gitql_core::values::time::TimeValue;

#[inline(always)]
pub fn register_std_datetime_functions(map: &mut HashMap<&'static str, StandardFunction>) {
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

    map.insert("justify_days", interval_justify_days);
    map.insert("justify_hours", interval_justify_hours);
}

#[inline(always)]
pub fn register_std_datetime_function_signatures(map: &mut HashMap<&'static str, Signature>) {
    map.insert(
        "date",
        Signature {
            parameters: vec![Box::new(VariantType {
                variants: vec![Box::new(DateType), Box::new(DateTimeType)],
            })],
            return_type: Box::new(DateType),
        },
    );
    map.insert(
        "current_date",
        Signature {
            parameters: vec![],
            return_type: Box::new(DateType),
        },
    );
    map.insert(
        "current_time",
        Signature {
            parameters: vec![],
            return_type: Box::new(TimeType),
        },
    );
    map.insert(
        "current_timestamp",
        Signature {
            parameters: vec![],
            return_type: Box::new(DateTimeType),
        },
    );
    map.insert(
        "now",
        Signature {
            parameters: vec![],
            return_type: Box::new(DateTimeType),
        },
    );
    map.insert(
        "makedate",
        Signature {
            parameters: vec![Box::new(IntType), Box::new(IntType)],
            return_type: Box::new(DateType),
        },
    );
    map.insert(
        "maketime",
        Signature {
            parameters: vec![Box::new(IntType), Box::new(IntType), Box::new(IntType)],
            return_type: Box::new(TimeType),
        },
    );
    map.insert(
        "dayname",
        Signature {
            parameters: vec![Box::new(DateType)],
            return_type: Box::new(TextType),
        },
    );
    map.insert(
        "day",
        Signature {
            parameters: vec![Box::new(DateType)],
            return_type: Box::new(IntType),
        },
    );
    map.insert(
        "monthname",
        Signature {
            parameters: vec![Box::new(DateType)],
            return_type: Box::new(TextType),
        },
    );
    map.insert(
        "hour",
        Signature {
            parameters: vec![Box::new(DateTimeType)],
            return_type: Box::new(IntType),
        },
    );
    map.insert(
        "minute",
        Signature {
            parameters: vec![Box::new(DateTimeType)],
            return_type: Box::new(IntType),
        },
    );
    map.insert(
        "isdate",
        Signature {
            parameters: vec![Box::new(AnyType)],
            return_type: Box::new(BoolType),
        },
    );
    map.insert(
        "dayofweek",
        Signature {
            parameters: vec![Box::new(DateType)],
            return_type: Box::new(IntType),
        },
    );
    map.insert(
        "dayofmonth",
        Signature {
            parameters: vec![Box::new(DateType)],
            return_type: Box::new(IntType),
        },
    );
    map.insert(
        "dayofyear",
        Signature {
            parameters: vec![Box::new(DateType)],
            return_type: Box::new(IntType),
        },
    );
    map.insert(
        "weekofyear",
        Signature {
            parameters: vec![Box::new(DateType)],
            return_type: Box::new(IntType),
        },
    );
    map.insert(
        "quarter",
        Signature {
            parameters: vec![Box::new(DateType)],
            return_type: Box::new(IntType),
        },
    );
    map.insert(
        "year",
        Signature {
            parameters: vec![Box::new(DateType)],
            return_type: Box::new(IntType),
        },
    );
    map.insert(
        "month",
        Signature {
            parameters: vec![Box::new(DateType)],
            return_type: Box::new(IntType),
        },
    );
    map.insert(
        "weekday",
        Signature {
            parameters: vec![Box::new(DateType)],
            return_type: Box::new(IntType),
        },
    );
    map.insert(
        "to_days",
        Signature {
            parameters: vec![Box::new(DateType)],
            return_type: Box::new(IntType),
        },
    );
    map.insert(
        "last_day",
        Signature {
            parameters: vec![Box::new(DateType)],
            return_type: Box::new(DateType),
        },
    );
    map.insert(
        "yearweek",
        Signature {
            parameters: vec![Box::new(DateType)],
            return_type: Box::new(TextType),
        },
    );

    map.insert(
        "justify_days",
        Signature {
            parameters: vec![Box::new(IntervalType)],
            return_type: Box::new(IntervalType),
        },
    );

    map.insert(
        "justify_hours",
        Signature {
            parameters: vec![Box::new(IntervalType)],
            return_type: Box::new(IntervalType),
        },
    );
}

pub fn date_extract_date(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let argument_type = inputs[0].data_type();
    if argument_type.is_date() {
        return inputs[0].clone();
    }
    let timestamp = inputs[0].as_date_time().unwrap();
    Box::new(DateValue::new(timestamp))
}

pub fn date_current_date(_inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let timestamp = Utc::now().timestamp();
    Box::new(DateValue::new(timestamp))
}

pub fn date_current_time(_inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let time_stamp = Utc::now().timestamp();
    let datetime = DateTime::from_timestamp(time_stamp, 0).unwrap();
    let time = datetime.format("%H:%M:%S").to_string();
    Box::new(TimeValue::new(time))
}

pub fn date_current_timestamp(_inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let timestamp = Utc::now().timestamp();
    Box::new(DateTimeValue::new(timestamp))
}

pub fn date_make_date(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let year = inputs[0].as_int().unwrap() as i32;
    let day_of_year = inputs[1].as_int().unwrap() as u32;
    let date = NaiveDate::from_yo_opt(year, day_of_year).unwrap();
    let datetime = date.and_hms_opt(0, 0, 0).unwrap();
    let timestamp = Utc.from_utc_datetime(&datetime).timestamp();
    Box::new(DateValue::new(timestamp))
}

pub fn date_make_time(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let hour = inputs[0].as_int().unwrap();
    let minute = inputs[1].as_int().unwrap();
    let second = inputs[2].as_int().unwrap();
    let time = format!("{}:{:02}:{:02}", hour, minute, second);
    Box::new(TimeValue::new(time))
}

pub fn date_day(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let date = inputs[0].as_date().unwrap();
    let parsed_date = DateTime::from_timestamp(date, 0).unwrap();
    Box::new(IntValue::new(parsed_date.day().into()))
}

pub fn date_dayname(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let date = inputs[0].as_date().unwrap();
    let parsed_date = DateTime::from_timestamp(date, 0).unwrap();
    let day_name = match parsed_date.weekday() {
        Weekday::Mon => "Monday",
        Weekday::Tue => "Tuesday",
        Weekday::Wed => "Wednesday",
        Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday",
        Weekday::Sat => "Saturday",
        Weekday::Sun => "Sunday",
    }
    .to_string();
    Box::new(TextValue::new(day_name))
}

pub fn date_monthname(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let date = inputs[0].as_date().unwrap();
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
    }
    .to_string();

    Box::new(TextValue::new(month_name))
}

pub fn date_hour(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let date = inputs[0].as_date_time().unwrap();
    let date_time = DateTime::from_timestamp(date, 0);
    let dt = date_time.unwrap().time();
    Box::new(IntValue::new(dt.hour() as i64))
}

pub fn date_minute(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let date = inputs[0].as_date_time().unwrap();
    let date_time = DateTime::from_timestamp(date, 0);
    let dt = date_time.unwrap().time();
    Box::new(IntValue::new(dt.minute() as i64))
}

pub fn date_is_date(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let is_date = inputs[0].data_type().is_date();
    Box::new(BoolValue::new(is_date))
}

pub fn date_day_of_week(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let date = inputs[0].as_date().unwrap();
    let parsed_date = DateTime::from_timestamp(date, 0).unwrap();
    let value = parsed_date.weekday().number_from_sunday().into();
    Box::new(IntValue::new(value))
}

pub fn date_day_of_month(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let date = inputs[0].as_date().unwrap();
    let parsed_date = DateTime::from_timestamp(date, 0).unwrap();
    Box::new(IntValue::new(parsed_date.day().into()))
}

pub fn date_day_of_year(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let date = inputs[0].as_date().unwrap();
    let parsed_date = DateTime::from_timestamp(date, 0).unwrap();
    Box::new(IntValue::new(parsed_date.ordinal().into()))
}

pub fn date_week_of_year(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let date = inputs[0].as_date().unwrap();

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
    let value = ((days_with_offset / 7) as u32 + 1).into();
    Box::new(IntValue::new(value))
}

pub fn date_year_and_week(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let date = inputs[0].as_date().unwrap();
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
    let formatted_value = format!("{}{}", year, week_number);
    Box::new(TextValue::new(formatted_value))
}

pub fn date_quarter(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let date = inputs[0].as_date().unwrap();
    let parsed_date = DateTime::from_timestamp(date, 0).unwrap();
    let month = parsed_date.month() as i64;
    Box::new(IntValue::new((month - 1) / 3 + 1))
}

pub fn date_year(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let date = inputs[0].as_date().unwrap();
    let parsed_date = DateTime::from_timestamp(date, 0).unwrap();
    Box::new(IntValue::new(parsed_date.year().into()))
}

pub fn date_month(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let date = inputs[0].as_date().unwrap();
    let parsed_date = DateTime::from_timestamp(date, 0).unwrap();
    Box::new(IntValue::new(parsed_date.month().into()))
}

pub fn date_weekday(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let date = inputs[0].as_date().unwrap();
    let parsed_date = DateTime::from_timestamp(date, 0).unwrap();
    let value = (parsed_date.weekday().number_from_monday() - 1) as i64;
    Box::new(IntValue::new(value))
}

pub fn date_to_days(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let date = inputs[0].as_date().unwrap();
    let parsed_date = DateTime::from_timestamp(date, 0).unwrap();
    let days_since_year_0 = parsed_date.ordinal0() as i64;
    let year = parsed_date.year() as i64;
    let leap_years = (year - 1) / 4 - (year - 1) / 100 + (year - 1) / 400;
    let non_leap_years = year - leap_years;
    let days = 365 * non_leap_years + 366 * leap_years + days_since_year_0;
    Box::new(IntValue::new(days))
}

pub fn date_last_day(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let date = inputs[0].as_date().unwrap();
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

    Box::new(DateValue::new(last_day.timestamp()))
}

pub fn interval_justify_days(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let mut input_interval = inputs[0].as_interval().unwrap();
    while input_interval.days >= 30 {
        input_interval.months += 1;
        input_interval.days -= 30;
    }
    Box::new(IntervalValue::new(input_interval))
}

pub fn interval_justify_hours(inputs: &[Box<dyn Value>]) -> Box<dyn Value> {
    let mut input_interval = inputs[0].as_interval().unwrap();
    while input_interval.days >= 24 {
        input_interval.days += 1;
        input_interval.hours -= 24;
    }
    Box::new(IntervalValue::new(input_interval))
}
