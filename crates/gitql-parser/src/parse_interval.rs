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

fn parse_interval_literal(
    interval_str: &str,
    location: SourceLocation,
) -> Result<Interval, Box<Diagnostic>> {
    if interval_str.is_empty() {
        return Err(Diagnostic::error("Invalid input syntax for type interval")
            .with_location(location)
            .as_boxed());
    }

    let mut position = 0;
    let mut interval = Interval::default();
    let tokens = interval_str.split_whitespace().collect::<Vec<&str>>();

    let mut has_years = false;
    let mut has_months = false;
    let mut has_days = false;
    let mut has_hours = false;
    let mut has_minutes = false;
    let mut has_seconds = false;

    while position < tokens.len() {
        let token = tokens[position].trim();
        if token.is_empty() {
            position += 1;
            continue;
        }

        // Parse Days, Months or Years
        if let Ok(value) = token.parse::<i64>() {
            // Consume value
            position += 1;

            if position >= tokens.len() {
                return Err(Diagnostic::error(&format!(
                    "Missing interval unit after value {}",
                    value
                ))
                .with_location(location)
                .as_boxed());
            }

            // Parse the unit
            let maybe_unit = tokens[position];
            if matches!(maybe_unit, "year" | "years") {
                check_interval_value_and_unit(&mut has_years, value, maybe_unit, location)?;
                interval.years = value;
                position += 1;
                continue;
            }

            if matches!(maybe_unit, "mon" | "mons" | "months") {
                check_interval_value_and_unit(&mut has_months, value, maybe_unit, location)?;
                interval.months = value;
                position += 1;
                continue;
            }

            if matches!(maybe_unit, "day" | "days") {
                check_interval_value_and_unit(&mut has_days, value, maybe_unit, location)?;
                interval.days = value;
                position += 1;
                continue;
            }

            return Err(Diagnostic::error(&format!(
                "Invalid input syntax for interval unit `{}`",
                maybe_unit
            ))
            .add_help(
                "Interval date unit can be `[year | years | mon | mons | months | day or days]`",
            )
            .with_location(location)
            .as_boxed());
        }

        // Parse Seconds, Minutes or Hours
        if token.contains(':') {
            let time_parts: Vec<&str> = token.split(':').collect();
            if time_parts.len() != 3 && time_parts.len() != 2 {
                return Err(Diagnostic::error("Invalid input syntax for type interval")
                    .with_location(location)
                    .as_boxed());
            }

            match time_parts[0].parse::<i64>() {
                Ok(hours) => {
                    check_interval_value_and_unit(&mut has_hours, hours, time_parts[0], location)?;
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
                        location,
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
                            location,
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

    Ok(interval)
}

fn check_interval_value_and_unit(
    is_used_twice: &mut bool,
    interval_value: i64,
    unit_name: &str,
    location: SourceLocation,
) -> Result<(), Box<Diagnostic>> {
    if !*is_used_twice {
        *is_used_twice = true;
        return Ok(());
    }

    if !(-INTERVAL_MAX_VALUE..=INTERVAL_MAX_VALUE).contains(&interval_value) {
        return Err(Diagnostic::error(&format!(
            "Interval value for unit `{}` is out of the range",
            unit_name
        ))
        .add_help("Interval value must be in range from -170_000_000 to 170_000_000")
        .with_location(location)
        .as_boxed());
    }

    Err(Diagnostic::error(&format!(
        "Can't use the same interval unit `{}` twice",
        unit_name
    ))
    .with_location(location)
    .as_boxed())
}
