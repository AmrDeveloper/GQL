use gitql_ast::statement::{IntoStatement, Statement};

use crate::diagnostic::Diagnostic;
use crate::parser::{calculate_safe_location, consume_token_or_error};
use crate::token::{Token, TokenKind};

pub(crate) fn parse_into_statement(
    tokens: &[Token],
    position: &mut usize,
) -> Result<Box<dyn Statement>, Box<Diagnostic>> {
    // Consume `INTO` keyword
    *position += 1;

    // Make sure user define explicitly the into type
    if *position >= tokens.len()
        || (tokens[*position].kind != TokenKind::Outfile
            && tokens[*position].kind != TokenKind::Dumpfile)
    {
        return Err(Diagnostic::error(
            "Expect Keyword `OUTFILE` or `DUMPFILE` after keyword `INTO`",
        )
        .with_location(calculate_safe_location(tokens, *position))
        .as_boxed());
    }

    // Consume `OUTFILE` or `DUMPFILE` keyword
    let file_format_kind = &tokens[*position].kind;
    *position += 1;

    // Make sure user defined a file path as string literal
    if *position >= tokens.len() || !matches!(tokens[*position].kind, TokenKind::String(_)) {
        return Err(Diagnostic::error(
            "Expect String literal as file path after OUTFILE or DUMPFILE keyword",
        )
        .with_location(calculate_safe_location(tokens, *position))
        .as_boxed());
    }

    let file_path = &tokens[*position].to_string();

    // Consume File path token
    *position += 1;

    let is_dump_file = *file_format_kind == TokenKind::Dumpfile;

    let mut lines_terminated = if is_dump_file { "" } else { "\n" }.to_string();
    let mut lines_terminated_used = false;

    let mut fields_terminated = if is_dump_file { "" } else { "," }.to_string();
    let mut fields_terminated_used = false;

    let mut enclosed = String::new();
    let mut enclosed_used = false;

    while *position < tokens.len() {
        let token = &tokens[*position];

        if token.kind == TokenKind::Lines {
            if is_dump_file {
                return Err(Diagnostic::error(
                    "`LINES TERMINATED` option can't be used with INTO DUMPFILE",
                )
                .add_help("To customize the format replace `DUMPFILE` with `OUTFILE` option")
                .with_location(tokens[*position].location)
                .as_boxed());
            }

            if lines_terminated_used {
                return Err(
                    Diagnostic::error("You already used `LINES TERMINATED` option")
                        .with_location(tokens[*position].location)
                        .as_boxed(),
                );
            }

            // Consume `LINES` keyword
            *position += 1;

            // Consume `TERMINATED` KEYWORD, or Error
            consume_token_or_error(
                tokens,
                position,
                TokenKind::Terminated,
                "Expect `TERMINATED` keyword after `LINES` keyword",
            )?;

            // Consume `By` KEYWORD, or Error
            consume_token_or_error(
                tokens,
                position,
                TokenKind::By,
                "Expect `BY` after `TERMINATED` keyword",
            )?;

            if *position >= tokens.len() || !matches!(tokens[*position].kind, TokenKind::String(_))
            {
                return Err(Diagnostic::error(
                    "Expect String literal as lines terminated value after BY keyword",
                )
                .with_location(calculate_safe_location(tokens, *position))
                .as_boxed());
            }

            // Consume `LINES TERMINATED BY` Value
            lines_terminated = tokens[*position].to_string();
            lines_terminated_used = true;
            *position += 1;
            continue;
        }

        if token.kind == TokenKind::Fields {
            if is_dump_file {
                return Err(Diagnostic::error(
                    "`FIELDS TERMINATED` option can't be used with INTO DUMPFILE",
                )
                .add_help("To customize the format replace `DUMPFILE` with `OUTFILE` option")
                .with_location(tokens[*position].location)
                .as_boxed());
            }

            if fields_terminated_used {
                return Err(
                    Diagnostic::error("You already used `FIELDS TERMINATED` option")
                        .with_location(tokens[*position].location)
                        .as_boxed(),
                );
            }

            // Consume `FIELDS` keyword
            *position += 1;

            // Consume `TERMINATED` KEYWORD, or Error
            consume_token_or_error(
                tokens,
                position,
                TokenKind::Terminated,
                "Expect `TERMINATED` keyword after `LINES` keyword",
            )?;

            // Consume `By` KEYWORD, or Error
            consume_token_or_error(
                tokens,
                position,
                TokenKind::By,
                "Expect `BY` after `TERMINATED` keyword",
            )?;

            if *position >= tokens.len() || !matches!(tokens[*position].kind, TokenKind::String(_))
            {
                return Err(Diagnostic::error(
                    "Expect String literal as Field terminated value after BY keyword",
                )
                .with_location(calculate_safe_location(tokens, *position))
                .as_boxed());
            }

            // Consume `FIELD TERMINATED BY` Value
            fields_terminated = tokens[*position].to_string();
            fields_terminated_used = true;
            *position += 1;
            continue;
        }

        if token.kind == TokenKind::Enclosed {
            if is_dump_file {
                return Err(Diagnostic::error(
                    "`ENCLOSED` option can't be used with INTO DUMPFILE",
                )
                .add_help("To customize the format replace `DUMPFILE` with `OUTFILE` option")
                .with_location(tokens[*position].location)
                .as_boxed());
            }

            if enclosed_used {
                return Err(Diagnostic::error("You already used ENCLOSED option")
                    .with_location(tokens[*position].location)
                    .as_boxed());
            }

            // Consume `ENCLOSED` token
            *position += 1;

            if *position >= tokens.len() || !matches!(tokens[*position].kind, TokenKind::String(_))
            {
                return Err(Diagnostic::error(
                    "Expect String literal as enclosed value after ENCLOSED keyword",
                )
                .with_location(calculate_safe_location(tokens, *position))
                .as_boxed());
            }

            // Consume `ENCLOSED` Value
            enclosed = tokens[*position].to_string();
            enclosed_used = true;
            *position += 1;
            continue;
        }

        break;
    }

    Ok(Box::new(IntoStatement {
        file_path: file_path.to_string(),
        lines_terminated,
        fields_terminated,
        enclosed,
    }))
}
