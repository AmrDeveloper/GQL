use gitql_ast::expression::Expr;
use gitql_ast::expression::IntervalExpr;
use gitql_ast::Interval;

use crate::diagnostic::Diagnostic;
use crate::parser::consume_conditional_token_or_errors;
use crate::parser::consume_token_or_error;
use crate::token::SourceLocation;
use crate::token::Token;
use crate::token::TokenKind;

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
    let tokens: Vec<&str> = interval_str.split_whitespace().collect();

    let mut position = 0;
    let mut interval = Interval::default();
    while position < tokens.len() {
        let token = tokens[position].trim();
        if token.is_empty() {
            position += 1;
            continue;
        }

        // Parse Days, Months or Years
        if let Ok(value) = token.parse::<i32>() {
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
                interval.years = value;
                position += 1;
                continue;
            }

            if matches!(maybe_unit, "mon" | "mons") {
                interval.months = value;
                position += 1;
                continue;
            }

            if matches!(maybe_unit, "day" | "days") {
                interval.days = value;
                position += 1;
                continue;
            }

            return Err(Diagnostic::error(&format!(
                "Invalid input syntax for interval unit `{}`",
                maybe_unit
            ))
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

            match time_parts[0].parse::<i32>() {
                Ok(hours) => {
                    interval.hours = hours;
                }
                Err(_) => {
                    return Err(Diagnostic::error("Invalid input syntax for type interval")
                        .with_location(location)
                        .as_boxed());
                }
            }

            match time_parts[1].parse::<i32>() {
                Ok(hours) => {
                    interval.minutes = hours;
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
                        interval.seconds = seconds;
                    }
                    Err(_) => {
                        return Err(Diagnostic::error("Invalid input syntax for type interval")
                            .with_location(location)
                            .as_boxed());
                    }
                }
            }

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
