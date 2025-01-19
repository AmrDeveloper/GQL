use std::cmp::Ordering;
use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Div;
use std::ops::Mul;

const INTERVAL_MAX_VALUE_I: i64 = 170_000_000;
const INTERVAL_MAX_VALUE_F: f64 = 170_000_000.0;

#[derive(Default, PartialEq, Clone)]
pub struct Interval {
    pub years: i64,
    pub months: i64,
    pub days: i64,
    pub hours: i64,
    pub minutes: i64,
    pub seconds: f64,
}

impl Interval {
    pub fn add(&self, other: &Interval) -> Result<Interval, String> {
        let mut result = self.clone();
        result.years = interval_value_or_error_i64(result.years + other.years)?;
        result.months = interval_value_or_error_i64(result.months + other.months)?;
        result.days = interval_value_or_error_i64(result.days + other.days)?;
        result.hours = interval_value_or_error_i64(result.hours + other.hours)?;
        result.minutes = interval_value_or_error_i64(result.minutes + other.minutes)?;
        result.seconds = interval_value_or_error_f64(result.seconds + other.seconds)?;
        Ok(result)
    }

    pub fn sub(&self, other: &Interval) -> Result<Interval, String> {
        let mut result = self.clone();
        result.years = interval_value_or_error_i64(result.years - other.years)?;
        result.months = interval_value_or_error_i64(result.months - other.months)?;
        result.days = interval_value_or_error_i64(result.days - other.days)?;
        result.hours = interval_value_or_error_i64(result.hours - other.hours)?;
        result.minutes = interval_value_or_error_i64(result.minutes - other.minutes)?;
        result.seconds = interval_value_or_error_f64(result.seconds - other.seconds)?;
        Ok(result)
    }

    pub fn mul(&self, other: i64) -> Result<Interval, String> {
        let mut result = self.clone();
        result.years = interval_value_or_error_i64(result.years * other)?;
        result.months = interval_value_or_error_i64(result.months * other)?;
        result.days = interval_value_or_error_i64(result.days * other)?;
        result.hours = interval_value_or_error_i64(result.hours * other)?;
        result.minutes = interval_value_or_error_i64(result.minutes * other)?;
        result.seconds = interval_value_or_error_f64(result.seconds.mul(other as f64))?;
        Ok(result)
    }

    pub fn div(&self, other: i64) -> Result<Interval, String> {
        let mut result = self.clone();
        result.years = interval_value_or_error_i64(result.years / other)?;
        result.months = interval_value_or_error_i64(result.months / other)?;
        result.days = interval_value_or_error_i64(result.days / other)?;
        result.hours = interval_value_or_error_i64(result.hours / other)?;
        result.minutes = interval_value_or_error_i64(result.minutes / other)?;
        result.seconds = interval_value_or_error_f64(result.seconds.div(other as f64))?;
        Ok(result)
    }

    pub fn to_seconds(&self) -> i64 {
        let days =
            self.years as f64 * 365.25 + self.months as f64 * (365.25 / 12.0) + self.days as f64;

        let seconds = days * 24.0 * 60.0 * 60.0
            + self.hours as f64 * 60.0 * 60.0
            + self.minutes as f64 * 60.0
            + self.seconds;

        seconds as i64
    }
}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_seconds = self.to_seconds();
        let other_seconds = other.to_seconds();
        self_seconds.partial_cmp(&other_seconds)
    }
}

impl Display for Interval {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut parts = Vec::new();

        if self.years != 0 {
            parts.push(format!(
                "{} year{}",
                self.years,
                if self.years > 1 { "s" } else { "" }
            ));
        }
        if self.months != 0 {
            parts.push(format!(
                "{} month{}",
                self.months,
                if self.months > 1 { "s" } else { "" }
            ));
        }
        if self.days != 0 {
            parts.push(format!(
                "{} day{}",
                self.days,
                if self.days > 1 { "s" } else { "" }
            ));
        }

        let mut time_parts = Vec::new();
        if self.hours != 0 {
            time_parts.push(format!("{}", self.hours));
        }
        if self.minutes != 0 {
            time_parts.push(format!("{}", self.minutes));
        }
        if self.seconds != 0.0 {
            time_parts.push(format!("{}", self.seconds));
        }

        if !time_parts.is_empty() {
            parts.push(time_parts.join(":"));
        }

        if parts.is_empty() {
            write!(f, "0 seconds")?;
        } else {
            write!(f, "{}", parts.join(" "))?;
        }

        Ok(())
    }
}

fn interval_value_or_error_i64(value: i64) -> Result<i64, String> {
    if (-INTERVAL_MAX_VALUE_I..=INTERVAL_MAX_VALUE_I).contains(&value) {
        return Ok(value);
    }
    Err(format!("Interval value out of range {}", value))
}

fn interval_value_or_error_f64(value: f64) -> Result<f64, String> {
    if (-INTERVAL_MAX_VALUE_F..=INTERVAL_MAX_VALUE_F).contains(&value) {
        return Ok(value);
    }
    Err("Interval value out of range".to_string())
}
