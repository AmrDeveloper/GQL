use gitql_ast::expression::Expr;
use gitql_ast::expression::IntervalExpr;
use gitql_ast::Interval;

use crate::diagnostic::Diagnostic;
use crate::parser::consume_conditional_token_or_errors;
use crate::parser::consume_token_or_error;
use crate::token::SourceLocation;
use crate::token::Token;
use crate::token::TokenKind;

const INTERVAL_MAX_VALUE: i64 = 170_000_000;

pub(crate) fn parse_interval_expression(
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Expr>, Box<Diagnostic>> {
    consume_token_or_error(
        tokens,
        position,
        TokenKind::Interval,
        "Expect 'Interval' Keyword",
    )?;

    let interval_value_token = consume_conditional_token_or_errors(
        tokens,
        position,
        |t| matches!(t.kind, TokenKind::String(_)),
        "Expect String after 'Interval' Keyword as interval value",
    )?;

    let interval = parse_interval_literal(
        &interval_value_token.to_string(),
        interval_value_token.location,
    )?;

    Ok(Box::new(IntervalExpr::new(interval)))
}

/// Parse string into Interval expression.
///
/// [@] <quantity> <unit> [quantity unit...] <direction>
///
/// quantity: is a number (possibly signed)
/// unit: is
///     microsecond, millisecond, second,
///     minute, hour, day, week, month, year,
///     decade, century, millennium, or abbreviations or plurals of these units
/// direction: can be ago or empty
/// The at sign (@) is optional noise
///
/// Ref: https://www.postgresql.org/docs/current/datatype-datetime.html#DATATYPE-INTERVAL-INPUT
fn parse_interval_literal(
    interval_str: &str,
    location: SourceLocation,
) -> Result<Interval, Box<Diagnostic>> {
    let tokens = interval_str.split_whitespace().collect::<Vec<&str>>();
    if tokens.is_empty() {
        return Err(Diagnostic::error("Interval value can't be empty")
            .add_help("Please check the documentation for help")
            .with_location(location)
            .as_boxed());
    }

    let mut position = 0;
    let mut interval = Interval::default();

    // Date part
    let mut has_millenniums = false;
    let mut has_centuries = false;
    let mut has_decades = false;
    let mut has_years = false;
    let mut has_months = false;
    let mut has_week = false;
    let mut has_days = false;

    // Time part
    let mut has_any_time_part = false;
    let mut has_hours: bool = false;
    let mut has_minutes = false;
    let mut has_seconds = false;

    let mut has_direction_ago = false;

    while position < tokens.len() {
        let token = tokens[position].trim();
        if token.is_empty() {
            position += 1;
            continue;
        }

        if token == "ago" {
            if has_direction_ago {
                return Err(
                    Diagnostic::error("Interval can't contains more than one `ago`")
                        .add_help("Please keep at most only one `ago` direction")
                        .with_location(location)
                        .as_boxed(),
                );
            }

            has_direction_ago = true;
            position += 1;
            continue;
        }

        // Parse Millienniums, Centuries, Decades, Years, Weeks, Months and Days
        if let Ok(value) = token.parse::<i64>() {
            // Consume value
            position += 1;

            if position >= tokens.len() {
                return Err(Diagnostic::error(&format!(
                    "Missing interval unit after value {value}",
                ))
                .with_location(location)
                .as_boxed());
            }

            // Parse the unit
            let mut maybe_unit = tokens[position];
            let unit_lower = &maybe_unit.to_lowercase();
            maybe_unit = unit_lower.as_str();

            if matches!(maybe_unit, "millennium" | "millenniums") {
                check_interval_value_and_unit(&mut has_millenniums, value, maybe_unit, &location)?;
                interval.years += value * 1000;
                position += 1;
                continue;
            }

            if matches!(maybe_unit, "century" | "centuries") {
                check_interval_value_and_unit(&mut has_centuries, value, maybe_unit, &location)?;
                interval.years += value * 100;
                position += 1;
                continue;
            }

            if matches!(maybe_unit, "decade" | "decades") {
                check_interval_value_and_unit(&mut has_decades, value, maybe_unit, &location)?;
                interval.years += value * 10;
                position += 1;
                continue;
            }

            if matches!(maybe_unit, "y" | "year" | "years") {
                check_interval_value_and_unit(&mut has_years, value, maybe_unit, &location)?;
                interval.years += value;
                position += 1;
                continue;
            }

            if matches!(maybe_unit, "m" | "mon" | "mons" | "months") {
                check_interval_value_and_unit(&mut has_months, value, maybe_unit, &location)?;
                interval.months += value;
                position += 1;
                continue;
            }

            if matches!(maybe_unit, "w" | "week" | "weeks") {
                check_interval_value_and_unit(&mut has_week, value, maybe_unit, &location)?;
                interval.days += value * 7;
                position += 1;
                continue;
            }

            if matches!(maybe_unit, "d" | "day" | "days") {
                check_interval_value_and_unit(&mut has_days, value, maybe_unit, &location)?;
                interval.days += value;
                position += 1;
                continue;
            }

            if matches!(maybe_unit, "h" | "hour" | "hours") {
                check_interval_value_and_unit(&mut has_hours, value, maybe_unit, &location)?;
                has_any_time_part = true;
                interval.hours += value;
                position += 1;
                continue;
            }

            if matches!(maybe_unit, "minute" | "minutes") {
                check_interval_value_and_unit(&mut has_minutes, value, maybe_unit, &location)?;
                has_any_time_part = true;
                interval.minutes += value;
                position += 1;
                continue;
            }
        }

        // Parse Seconds
        if let Ok(value) = token.parse::<f64>() {
            // Consume value
            position += 1;

            if position >= tokens.len() {
                return Err(Diagnostic::error(&format!(
                    "Missing interval unit after value {value}",
                ))
                .with_location(location)
                .as_boxed());
            }

            // Parse the unit
            let mut maybe_unit = tokens[position];
            let unit_lower = &maybe_unit.to_lowercase();
            maybe_unit = unit_lower.as_str();

            if matches!(maybe_unit, "second" | "seconds") {
                check_interval_value_and_unit(
                    &mut has_seconds,
                    value as i64,
                    maybe_unit,
                    &location,
                )?;
                has_any_time_part = true;
                interval.seconds += value;
                position += 1;
                continue;
            }

            return Err(Diagnostic::error(&format!(
                "Invalid input syntax for interval unit `{maybe_unit}`",
            ))
            .add_help(
                "Interval date unit can be `[year | years | mon | mons | months | day or days]`",
            )
            .with_location(location)
            .as_boxed());
        }

        // Parse the optional time part without explicit unit markings (Seconds, Minutes or Hours)
        if token.contains(':') {
            if has_any_time_part {
                return Err(
                    Diagnostic::error("You can't have time value twice in same interval")
                        .with_location(location)
                        .as_boxed(),
                );
            }

            let time_parts: Vec<&str> = token.split(':').collect();
            if !matches!(time_parts.len(), 2 | 3) {
                return Err(Diagnostic::error("Invalid input syntax for type interval")
                    .with_location(location)
                    .as_boxed());
            }

            match time_parts[0].parse::<i64>() {
                Ok(hours) => {
                    check_interval_value_and_unit(&mut has_hours, hours, time_parts[0], &location)?;
                    interval.hours = hours;
                }
                Err(_) => {
                    return Err(Diagnostic::error("Invalid input syntax for type interval")
                        .with_location(location)
                        .as_boxed());
                }
            }

            match time_parts[1].parse::<i64>() {
                Ok(minutes) => {
                    check_interval_value_and_unit(
                        &mut has_minutes,
                        minutes,
                        time_parts[1],
                        &location,
                    )?;
                    interval.minutes = minutes;
                }
                Err(_) => {
                    return Err(Diagnostic::error("Invalid input syntax for type interval")
                        .with_location(location)
                        .as_boxed());
                }
            }

            if time_parts.len() == 3 {
                match time_parts[2].parse::<f64>() {
                    Ok(seconds) => {
                        check_interval_value_and_unit(
                            &mut has_seconds,
                            seconds as i64,
                            time_parts[2],
                            &location,
                        )?;
                        interval.seconds = seconds;
                    }
                    Err(_) => {
                        return Err(Diagnostic::error("Invalid input syntax for type interval")
                            .with_location(location)
                            .as_boxed());
                    }
                }
            }

            // Consume time token
            position += 1;
            continue;
        }

        return Err(Diagnostic::error("Invalid input syntax for type interval")
            .add_note("Expect numeric value before each unit in interval value")
            .with_location(location)
            .as_boxed());
    }

    // ago directon negates all the fields
    if has_direction_ago {
        interval = interval.mul(-1).unwrap_or(interval);
    }

    Ok(interval)
}

fn check_interval_value_and_unit(
    is_used_twice: &mut bool,
    interval_value: i64,
    unit_name: &str,
    location: &SourceLocation,
) -> Result<(), Box<Diagnostic>> {
    if !*is_used_twice {
        *is_used_twice = true;
        return Ok(());
    }

    if !(-INTERVAL_MAX_VALUE..=INTERVAL_MAX_VALUE).contains(&interval_value) {
        return Err(Diagnostic::error(&format!(
            "Interval value for unit `{unit_name}` is out of the range",
        ))
        .add_help("Interval value must be in range from -170_000_000 to 170_000_000")
        .with_location(*location)
        .as_boxed());
    }

    Err(Diagnostic::error(&format!(
        "Can't use the same interval unit `{unit_name}` twice",
    ))
    .with_location(*location)
    .as_boxed())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_hours() {
        let inputs = ["1 h", "1 hour", "1 hours", "1:00:00"];
        for input in inputs {
            let parse_result = parse_interval_literal(input, SourceLocation::default());
            assert!(parse_result.is_ok());

            if let Ok(interval) = parse_result {
                assert_eq!(interval.hours, 1);
            }
        }
    }

    #[test]
    fn valid_weeks() {
        let inputs = [
            "2 w",
            "2 week",
            "2 weeks",
            "1 week 7 day",
            "1 w 7 d",
            "1 weeks 7 days",
        ];

        for input in inputs {
            let parse_result = parse_interval_literal(input, SourceLocation::default());
            assert!(parse_result.is_ok());

            if let Ok(interval) = parse_result {
                assert_eq!(interval.days, 14);
            }
        }
    }

    #[test]
    fn valid_seconds() {
        let inputs = ["10.1 second"];

        for input in inputs {
            let parse_result = parse_interval_literal(input, SourceLocation::default());
            assert!(parse_result.is_ok());

            if let Ok(interval) = parse_result {
                assert_eq!(interval.seconds, 10.1);
            }
        }
    }

    #[test]
    fn ago_direction() {
        let parse_result = parse_interval_literal("1 y 1 m 1 w 1 d", SourceLocation::default());
        assert!(parse_result.is_ok());

        let parse_result_with_ago =
            parse_interval_literal("1 y 1 m 1 w 1 d ago", SourceLocation::default());
        assert!(parse_result_with_ago.is_ok());

        assert_eq!(
            parse_result.ok().unwrap().mul(-1).unwrap(),
            parse_result_with_ago.ok().unwrap()
        );
    }

    #[test]
    fn invalid_time() {
        let inputs = ["1 h 1:00:00", "1 h 1 h"];
        for input in inputs {
            let parse_result = parse_interval_literal(input, SourceLocation::default());
            assert!(parse_result.is_err());
        }
    }
}
