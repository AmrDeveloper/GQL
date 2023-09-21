extern crate chrono;

use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::TimeZone;
use chrono::Utc;

static CHRONO_TIME_FORMAT: &str = "%H:%M:%S";
static CHRONO_DATE_FORMAT: &str = "%Y-%m-%d";
static CHRONO_DATE_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub fn get_unix_timestamp_ms() -> i64 {
    let now = Utc::now();
    return now.timestamp();
}

pub fn time_stamp_to_date(time_stamp: i64) -> String {
    let utc = NaiveDateTime::from_timestamp_opt(time_stamp, 0).unwrap();
    let datetime = Utc.from_utc_datetime(&utc);
    let date_str = datetime.format(CHRONO_DATE_FORMAT).to_string();
    return date_str;
}

pub fn time_stamp_to_time(time_stamp: i64) -> String {
    let utc = NaiveDateTime::from_timestamp_opt(time_stamp, 0).unwrap();
    let datetime = Utc.from_utc_datetime(&utc);
    let time_str = datetime.format(CHRONO_TIME_FORMAT).to_string();
    return time_str;
}

pub fn time_stamp_to_date_time(time_stamp: i64) -> String {
    let utc = NaiveDateTime::from_timestamp_opt(time_stamp, 0).unwrap();
    let datetime = Utc.from_utc_datetime(&utc);
    let date_time_str = datetime.format(CHRONO_DATE_TIME_FORMAT).to_string();
    return date_time_str;
}

pub fn date_time_to_time_stamp(date: &str) -> i64 {
    let date_time = NaiveDateTime::parse_from_str(date, CHRONO_DATE_TIME_FORMAT);
    if date_time.is_err() {
        return 0;
    }
    return date_time.ok().unwrap().timestamp();
}

pub fn time_stamp_from_year_and_day(year: i32, day_of_year: u32) -> i64 {
    let date = NaiveDate::from_yo_opt(year, day_of_year).unwrap();
    let datetime = date.and_hms_opt(0, 0, 0).unwrap();
    let time_stamp = Utc.from_utc_datetime(&datetime).timestamp();
    return time_stamp;
}
