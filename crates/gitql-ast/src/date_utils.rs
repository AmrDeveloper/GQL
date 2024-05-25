extern crate chrono;

use chrono::DateTime;
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
    let datetime = DateTime::from_timestamp(time_stamp, 0).unwrap();
    datetime.format(CHRONO_DATE_FORMAT).to_string()
}

pub fn time_stamp_to_time(time_stamp: i64) -> String {
    let datetime = DateTime::from_timestamp(time_stamp, 0).unwrap();
    datetime.format(CHRONO_TIME_FORMAT).to_string()
}

pub fn time_stamp_to_date_time(time_stamp: i64) -> String {
    let datetime = DateTime::from_timestamp(time_stamp, 0).unwrap();
    datetime.format(CHRONO_DATE_TIME_FULL_FORMAT).to_string()
}

pub fn date_to_time_stamp(date: &str) -> i64 {
    let date_time = NaiveDate::parse_from_str(date, CHRONO_DATE_FORMAT).ok();
    if let Some(date) = date_time {
        let zero_time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
        return date.and_time(zero_time).and_utc().timestamp();
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
    date_time.ok().unwrap().and_utc().timestamp()
}

pub fn date_time_to_hour(date: i64) -> i64 {
    let date_time = DateTime::from_timestamp(date, 0);
    let dt = date_time.unwrap().time();
    dt.hour() as i64
}

pub fn date_time_to_minute(date: i64) -> i64 {
    let date_time = DateTime::from_timestamp(date, 0);
    let dt = date_time.unwrap().time();
    dt.minute() as i64
}

pub fn date_to_day_number_in_week(date: i64) -> u32 {
    let parsed_date = DateTime::from_timestamp(date, 0).unwrap();
    parsed_date.weekday().number_from_sunday()
}

pub fn date_to_day_number_in_month(date: i64) -> u32 {
    let parsed_date = DateTime::from_timestamp(date, 0).unwrap();
    parsed_date.day()
}

pub fn date_to_day_number_in_year(date: i64) -> u32 {
    let parsed_date = DateTime::from_timestamp(date, 0).unwrap();
    parsed_date.ordinal()
}

pub fn date_to_week_number_in_year(date: i64) -> u32 {
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
    (days_with_offset / 7) as u32 + 1
}

pub fn date_to_day_name(date: i64) -> String {
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

    day_name.to_string()
}

pub fn date_to_month_name(date: i64) -> String {
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

    month_name.to_string()
}

pub fn date_to_quarter_index(date: i64) -> i64 {
    let parsed_date = DateTime::from_timestamp(date, 0).unwrap();
    let month = parsed_date.month() as i64;
    (month - 1) / 3 + 1
}

pub fn date_to_year(date: i64) -> i32 {
    let parsed_date = DateTime::from_timestamp(date, 0).unwrap();
    parsed_date.year()
}

pub fn date_to_month(date: i64) -> u32 {
    let parsed_date = DateTime::from_timestamp(date, 0).unwrap();
    parsed_date.month()
}

pub fn date_to_weekday(date: i64) -> u32 {
    let parsed_date = DateTime::from_timestamp(date, 0).unwrap();
    parsed_date.weekday().number_from_monday() - 1
}

pub fn date_to_days_count(date: i64) -> i64 {
    let parsed_date = DateTime::from_timestamp(date, 0).unwrap();
    let days_since_year_0 = parsed_date.ordinal0() as i64;
    let year = parsed_date.year() as i64;
    let leap_years = (year - 1) / 4 - (year - 1) / 100 + (year - 1) / 400;
    let non_leap_years = year - leap_years;
    365 * non_leap_years + 366 * leap_years + days_since_year_0
}

pub fn date_last_day(date: i64) -> i64 {
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
    last_day.timestamp()
}

pub fn time_stamp_from_year_and_day(year: i32, day_of_year: u32) -> i64 {
    let date = NaiveDate::from_yo_opt(year, day_of_year).unwrap();
    let datetime = date.and_hms_opt(0, 0, 0).unwrap();
    Utc.from_utc_datetime(&datetime).timestamp()
}
