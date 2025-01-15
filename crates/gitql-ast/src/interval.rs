use std::cmp::Ordering;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

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
    pub fn add(&self, interval: &Interval) -> Interval {
        let mut result = self.clone();
        result.years += interval.years;
        result.months += interval.months;
        result.days += interval.days;
        result.hours += interval.hours;
        result.minutes += interval.minutes;
        result.seconds += interval.seconds;
        result
    }

    pub fn sub(&self, interval: &Interval) -> Interval {
        let mut result = self.clone();
        result.years -= interval.years;
        result.months -= interval.months;
        result.days -= interval.days;
        result.hours -= interval.hours;
        result.minutes -= interval.minutes;
        result.seconds -= interval.seconds;
        result
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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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
                "{} mon{}",
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
