extern crate chrono;

use chrono::Datelike;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::NaiveTime;
use chrono::TimeZone;
use chrono::Timelike;
use chrono::Utc;
use chrono::Weekday;

static CHRONO_TIME_FORMAT: &str = "%H:%M:%S";
static CHRONO_DATE_FORMAT: &str = "%Y-%m-%d";
static CHRONO_DATE_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
static CHRONO_DATE_TIME_FULL_FORMAT: &str = "%Y-%m-%d %H:%M:%S%.3f";

pub fn get_unix_timestamp_ms() -> i64 {
    Utc::now().timestamp()
}

pub fn time_stamp_to_date(time_stamp: i64) -> String {
    let utc = NaiveDateTime::from_timestamp_opt(time_stamp, 0).unwrap();
    let datetime = Utc.from_utc_datetime(&utc);
    datetime.format(CHRONO_DATE_FORMAT).to_string()
}

pub fn time_stamp_to_time(time_stamp: i64) -> String {
    let utc = NaiveDateTime::from_timestamp_opt(time_stamp, 0).unwrap();
    let datetime = Utc.from_utc_datetime(&utc);
    datetime.format(CHRONO_TIME_FORMAT).to_string()
}

pub fn time_stamp_to_date_time(time_stamp: i64) -> String {
    let utc = NaiveDateTime::from_timestamp_opt(time_stamp, 0).unwrap();
    let datetime = Utc.from_utc_datetime(&utc);
    datetime.format(CHRONO_DATE_TIME_FULL_FORMAT).to_string()
}

pub fn date_to_time_stamp(date: &str) -> i64 {
    let date_time = NaiveDate::parse_from_str(date, CHRONO_DATE_FORMAT).ok();
    if let Some(date) = date_time {
        let zero_time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
        return date.and_time(zero_time).timestamp();
    }
    0
}

pub fn date_time_to_time_stamp(date: &str) -> i64 {
    let date_time_format = if date.contains('.') {
        CHRONO_DATE_TIME_FULL_FORMAT
    } else {
        CHRONO_DATE_TIME_FORMAT
    };

    let date_time = NaiveDateTime::parse_from_str(date, date_time_format);
    if date_time.is_err() {
        return 0;
    }
    date_time.ok().unwrap().timestamp()
}

pub fn date_time_to_hour(date: i64) -> i64 {
    let date_time = NaiveDateTime::from_timestamp_opt(date, 0);
    let dt = date_time.unwrap().time();
    dt.hour() as i64
}

pub fn date_to_day_number_in_month(date: i64) -> u32 {
    let parsed_date = NaiveDateTime::from_timestamp_opt(date, 0).unwrap();
    parsed_date.day()
}

pub fn date_to_day_name(date: i64) -> String {
    let parsed_date = NaiveDateTime::from_timestamp_opt(date, 0).unwrap();

    let day_name = match parsed_date.weekday() {
        Weekday::Mon => "Monday",
        Weekday::Tue => "Tuesday",
        Weekday::Wed => "Wednesday",
        Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday",
        Weekday::Sat => "Saturday",
        Weekday::Sun => "Sunday",
    };

    day_name.to_string()
}

pub fn date_to_month_name(date: i64) -> String {
    let parsed_date = NaiveDateTime::from_timestamp_opt(date, 0).unwrap();

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

    month_name.to_string()
}

pub fn time_stamp_from_year_and_day(year: i32, day_of_year: u32) -> i64 {
    let date = NaiveDate::from_yo_opt(year, day_of_year).unwrap();
    let datetime = date.and_hms_opt(0, 0, 0).unwrap();
    Utc.from_utc_datetime(&datetime).timestamp()
}

/// Check if String literal is matching SQL time format: HH:MM:SS or HH:MM:SS.SSS
pub fn is_valid_time_format(time_str: &str) -> bool {
    // Check length of the string
    if !(8..=12).contains(&time_str.len()) {
        return false;
    }

    // Split the string into hours, minutes, seconds, and optional milliseconds
    let parts: Vec<&str> = time_str.split(':').collect();
    if parts.len() < 3 || parts.len() > 4 {
        return false;
    }

    // Extract hours, minutes, seconds, and optionally milliseconds
    let hours = parts[0].parse::<u32>().ok();
    let minutes = parts[1].parse::<u32>().ok();
    let seconds_parts: Vec<&str> = parts[2].split('.').collect();
    let seconds = seconds_parts[0].parse::<u32>().ok();
    let milliseconds = if seconds_parts.len() == 2 {
        seconds_parts[1].parse::<u32>().ok()
    } else {
        Some(0)
    };

    // Validate the parsed values
    hours.is_some()
        && minutes.is_some()
        && seconds.is_some()
        && milliseconds.is_some()
        && hours.unwrap() < 24
        && minutes.unwrap() < 60
        && seconds.unwrap() < 60
        && milliseconds.unwrap() < 1000
}

/// Check if String literal is matching SQL Date format: YYYY-MM-DD
pub fn is_valid_date_format(date_str: &str) -> bool {
    // Check length of the string
    if date_str.len() != 10 {
        return false;
    }

    // Split the string into year, month, and day
    let parts: Vec<&str> = date_str.split('-').collect();
    if parts.len() != 3 {
        return false;
    }

    // Extract year, month, and day
    let year = parts[0].parse::<u32>().ok();
    let month = parts[1].parse::<u32>().ok();
    let day = parts[2].parse::<u32>().ok();

    // Validate the parsed values
    year.is_some()
        && month.is_some()
        && day.is_some()
        && year.unwrap() >= 1
        && month.unwrap() >= 1
        && month.unwrap() <= 12
        && day.unwrap() >= 1
        && day.unwrap() <= 31
}

/// Check if String literal is matching SQL Date format: YYYY-MM-DD HH:MM:SS or YYYY-MM-DD HH:MM:SS.SSS
pub fn is_valid_datetime_format(datetime_str: &str) -> bool {
    // Check length of the string
    if !(19..=23).contains(&datetime_str.len()) {
        return false;
    }

    // Split the string into date and time components
    let parts: Vec<&str> = datetime_str.split_whitespace().collect();
    if parts.len() != 2 {
        return false;
    }

    // Check the validity of date and time components
    is_valid_date_format(parts[0]) && is_valid_time_format(parts[1])
}
