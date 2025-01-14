use std::fmt;

#[derive(PartialEq, Default, Clone)]
pub struct Interval {
    pub years: i32,
    pub months: i32,
    pub days: i32,
    pub hours: i32,
    pub minutes: i32,
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
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
