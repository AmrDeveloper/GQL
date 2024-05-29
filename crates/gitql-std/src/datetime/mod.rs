extern crate chrono;

use chrono::DateTime;
use chrono::Datelike;
use chrono::NaiveDate;
use chrono::TimeZone;
use chrono::Timelike;
use chrono::Utc;
use chrono::Weekday;
use gitql_core::value::Value;

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
