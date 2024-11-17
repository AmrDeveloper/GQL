use crate::diagnostic::Diagnostic;
use crate::token::{Location, Token, TokenKind};

pub fn tokenize(script: String) -> Result<Vec<Token>, Box<Diagnostic>> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut position = 0;
    let mut column_start;

    let characters: Vec<char> = script.chars().collect();
    let len = characters.len();

    while position < len {
        column_start = position;

        let char = characters[position];

        // Symbol
        if char.is_alphabetic() {
            tokens.push(consume_identifier(
                &characters,
                &mut position,
                &mut column_start,
            ));
            continue;
        }

        // @> or Global Variable Symbol
        if char == '@' {
            // @>
            if position + 1 < len && characters[position + 1] == '>' {
                position += 2;
                let location = Location::new(column_start, position);
                tokens.push(Token::new(TokenKind::AtRightArrow, location));
                continue;
            }

            tokens.push(consume_global_variable_name(
                &characters,
                &mut position,
                &mut column_start,
            )?);
            continue;
        }

        // Number
        if char.is_numeric() {
            if char == '0' && position + 1 < len {
                if characters[position + 1] == 'x' {
                    position += 2;
                    column_start += 2;
                    tokens.push(consume_hex_number(
                        &characters,
                        &mut position,
                        &mut column_start,
                    )?);
                    continue;
                }

                if characters[position + 1] == 'b' {
                    position += 2;
                    column_start += 2;
                    tokens.push(consume_binary_number(
                        &characters,
                        &mut position,
                        &mut column_start,
                    )?);
                    continue;
                }

                if characters[position + 1] == 'o' {
                    position += 2;
                    column_start += 2;
                    tokens.push(consume_octal_number(
                        &characters,
                        &mut position,
                        &mut column_start,
                    )?);
                    continue;
                }
            }

            tokens.push(consume_number(
                &characters,
                &mut position,
                &mut column_start,
            )?);
            continue;
        }

        // String literal between single quotes '...'
        if char == '\'' {
            tokens.push(consume_string_in_single_quotes(
                &characters,
                &mut position,
                &mut column_start,
            )?);
            continue;
        }

        // String literal between double quotes "..."
        if char == '"' {
            tokens.push(consume_string_in_double_quotes(
                &characters,
                &mut position,
                &mut column_start,
            )?);
            continue;
        }

        // All chars between two backticks should be consumed as identifier
        if char == '`' {
            tokens.push(consume_backticks_identifier(
                &characters,
                &mut position,
                &mut column_start,
            )?);
            continue;
        }

        // Plus
        if char == '+' {
            let location = Location::new(column_start, position);
            tokens.push(Token::new(TokenKind::Plus, location));
            position += 1;
            continue;
        }

        // Minus
        if char == '-' {
            // Ignore single line comment which from -- until the end of the current line
            if position + 1 < characters.len() && characters[position + 1] == '-' {
                ignore_single_line_comment(&characters, &mut position);
                continue;
            }

            let location = Location::new(column_start, position);
            tokens.push(Token::new(TokenKind::Minus, location));
            position += 1;
            continue;
        }

        // Star
        if char == '*' {
            let location = Location::new(column_start, position);
            tokens.push(Token::new(TokenKind::Star, location));
            position += 1;
            continue;
        }

        // Slash
        if char == '/' {
            // Ignore C style comment which from /* comment */
            if position + 1 < characters.len() && characters[position + 1] == '*' {
                ignore_c_style_comment(&characters, &mut position)?;
                continue;
            }

            let location = Location::new(column_start, position);
            tokens.push(Token::new(TokenKind::Slash, location));
            position += 1;
            continue;
        }

        // Percentage
        if char == '%' {
            let location = Location::new(column_start, position);
            tokens.push(Token::new(TokenKind::Percentage, location));
            position += 1;
            continue;
        }

        // Caret
        if char == '^' {
            let location = Location::new(column_start, position);
            tokens.push(Token::new(TokenKind::Caret, location));
            position += 1;
            continue;
        }

        // Bitwise NOT
        if char == '~' {
            let location = Location::new(column_start, position);
            tokens.push(Token::new(TokenKind::BitwiseNot, location));
            position += 1;
            continue;
        }

        // Or
        if char == '|' {
            let location = Location::new(column_start, position);

            position += 1;
            let kind = if position < len && characters[position] == '|' {
                position += 1;
                TokenKind::OrOr
            } else {
                TokenKind::BitwiseOr
            };

            tokens.push(Token::new(kind, location));
            continue;
        }

        // And
        if char == '&' {
            let location = Location::new(column_start, position);

            position += 1;
            let kind = if position < len && characters[position] == '&' {
                position += 1;
                TokenKind::AndAnd
            } else {
                TokenKind::BitwiseAnd
            };

            tokens.push(Token::new(kind, location));
            continue;
        }

        // xor
        if char == '#' {
            let location = Location::new(column_start, position);
            tokens.push(Token::new(TokenKind::BitwiseXor, location));
            position += 1;
            continue;
        }

        // Comma
        if char == ',' {
            let location = Location::new(column_start, position);
            tokens.push(Token::new(TokenKind::Comma, location));
            position += 1;
            continue;
        }

        // Dot
        if char == '.' {
            let location = Location::new(column_start, position);
            tokens.push(Token::new(TokenKind::Dot, location));
            position += 1;
            continue;
        }

        // Greater or GreaterEqual
        if char == '>' {
            let location = Location::new(column_start, position);

            position += 1;
            let kind = if position < len && characters[position] == '=' {
                position += 1;
                TokenKind::GreaterEqual
            } else if position < len && characters[position] == '>' {
                position += 1;
                TokenKind::BitwiseRightShift
            } else {
                TokenKind::Greater
            };

            tokens.push(Token::new(kind, location));
            continue;
        }

        // Less, LessEqual or NULL-safe equal
        if char == '<' {
            let location = Location::new(column_start, position);

            position += 1;
            let kind = if position < len && characters[position] == '=' {
                position += 1;
                if position < len && characters[position] == '>' {
                    position += 1;
                    TokenKind::NullSafeEqual
                } else {
                    TokenKind::LessEqual
                }
            } else if position < len && characters[position] == '<' {
                position += 1;
                TokenKind::BitwiseLeftShift
            } else if position < len && characters[position] == '>' {
                position += 1;
                TokenKind::BangEqual
            } else if position < len && characters[position] == '@' {
                position += 1;
                TokenKind::ArrowRightAt
            } else {
                TokenKind::Less
            };

            tokens.push(Token::new(kind, location));
            continue;
        }

        // Equal
        if char == '=' {
            let location = Location::new(column_start, position);
            tokens.push(Token::new(TokenKind::Equal, location));
            position += 1;
            continue;
        }

        // Colon , ColonColon or Colon Equal
        if char == ':' {
            let location = Location::new(column_start, position);

            // :=
            if position + 1 < len && characters[position + 1] == '=' {
                tokens.push(Token::new(TokenKind::ColonEqual, location));
                position += 2;
                continue;
            }

            // ::
            if position + 1 < len && characters[position + 1] == ':' {
                tokens.push(Token::new(TokenKind::ColonColon, location));
                position += 2;
                continue;
            }

            tokens.push(Token::new(TokenKind::Colon, location));
            position += 1;
            continue;
        }

        // Bang or Bang Equal
        if char == '!' {
            let location = Location::new(column_start, position);

            position += 1;
            let kind = if position < len && characters[position] == '=' {
                TokenKind::BangEqual
            } else {
                TokenKind::Bang
            };

            tokens.push(Token::new(kind, location));
            continue;
        }

        // Left Paren
        if char == '(' {
            let location = Location::new(column_start, position);
            tokens.push(Token::new(TokenKind::LeftParen, location));
            position += 1;
            continue;
        }

        // Right Paren
        if char == ')' {
            let location = Location::new(column_start, position);
            tokens.push(Token::new(TokenKind::RightParen, location));
            position += 1;
            continue;
        }

        // Left Bracket
        if char == '[' {
            let location = Location::new(column_start, position);
            tokens.push(Token::new(TokenKind::LeftBracket, location));
            position += 1;
            continue;
        }

        // Right Bracket
        if char == ']' {
            let location = Location::new(column_start, position);
            tokens.push(Token::new(TokenKind::RightBracket, location));
            position += 1;
            continue;
        }

        // Semicolon
        if char == ';' {
            let location = Location::new(column_start, position);
            tokens.push(Token::new(TokenKind::Semicolon, location));
            position += 1;
            continue;
        }

        // Characters to ignoring
        if char == ' ' || char == '\n' || char == '\t' {
            position += 1;
            continue;
        }

        return Err(Diagnostic::error("Unexpected character")
            .with_location_span(column_start, position)
            .as_boxed());
    }

    Ok(tokens)
}

fn consume_global_variable_name(
    chars: &[char],
    pos: &mut usize,
    start: &mut usize,
) -> Result<Token, Box<Diagnostic>> {
    // Consume `@`
    *pos += 1;

    // Make sure first character is  alphabetic
    if *pos < chars.len() && !chars[*pos].is_alphabetic() {
        return Err(
            Diagnostic::error("Global variable name must start with alphabetic character")
                .add_help("Add at least one alphabetic character after @")
                .with_location_span(*start, *pos)
                .as_boxed(),
        );
    }

    while *pos < chars.len() && (chars[*pos] == '_' || chars[*pos].is_alphanumeric()) {
        *pos += 1;
    }

    // Identifier is being case-insensitive by default, convert to lowercase to be easy to compare and lookup
    let literal = &chars[*start..*pos];
    let mut string: String = literal.iter().collect();
    string = string.to_lowercase();

    let location = Location::new(*start, *pos);
    Ok(Token::new(TokenKind::GlobalVariable(string), location))
}

fn consume_identifier(chars: &[char], pos: &mut usize, start: &mut usize) -> Token {
    while *pos < chars.len() && (chars[*pos] == '_' || chars[*pos].is_alphanumeric()) {
        *pos += 1;
    }

    // Identifier is being case-insensitive by default, convert to lowercase to be easy to compare and lookup
    let literal = &chars[*start..*pos];
    let mut string: String = literal.iter().collect();
    string = string.to_lowercase();

    let location = Location::new(*start, *pos);
    Token::new_symbol(string, location)
}

fn consume_backticks_identifier(
    chars: &[char],
    pos: &mut usize,
    start: &mut usize,
) -> Result<Token, Box<Diagnostic>> {
    *pos += 1;

    while *pos < chars.len() && chars[*pos] != '`' {
        *pos += 1;
    }

    if *pos >= chars.len() {
        return Err(Diagnostic::error("Unterminated backticks")
            .add_help("Add ` at the end of the identifier")
            .with_location_span(*start, *pos)
            .as_boxed());
    }

    *pos += 1;

    let literal = &chars[*start + 1..*pos - 1];
    let identifier: String = literal.iter().collect();
    let location = Location::new(*start, *pos);
    Ok(Token::new(TokenKind::Symbol(identifier), location))
}

fn consume_number(
    chars: &[char],
    pos: &mut usize,
    start: &mut usize,
) -> Result<Token, Box<Diagnostic>> {
    while *pos < chars.len() && (chars[*pos].is_numeric() || chars[*pos] == '_') {
        *pos += 1;
    }

    let mut is_float_value = false;
    if *pos < chars.len() && chars[*pos] == '.' {
        *pos += 1;

        is_float_value = true;
        while *pos < chars.len() && (chars[*pos].is_numeric() || chars[*pos] == '_') {
            *pos += 1;
        }
    }

    let literal = &chars[*start..*pos];
    let string: String = literal.iter().collect();
    let literal_num = string.replace('_', "");
    let location = Location::new(*start, *pos);

    if is_float_value {
        return match literal_num.parse::<f64>() {
            Ok(float) => Ok(Token::new(TokenKind::Float(float), location)),
            Err(parse_float_error) => Err(Diagnostic::error(&parse_float_error.to_string())
                .add_note(&format!(
                    "Value must be between {} and {}",
                    f64::MIN,
                    f64::MAX
                ))
                .with_location_span(*start, *pos)
                .as_boxed()),
        };
    }

    match literal_num.parse::<i64>() {
        Ok(integer) => Ok(Token::new(TokenKind::Integer(integer), location)),
        Err(parse_int_error) => Err(Diagnostic::error(&parse_int_error.to_string())
            .add_note(&format!(
                "Value must be between {} and {}",
                i64::MIN,
                i64::MAX
            ))
            .with_location_span(*start, *pos)
            .as_boxed()),
    }
}

fn consume_binary_number(
    chars: &[char],
    pos: &mut usize,
    start: &mut usize,
) -> Result<Token, Box<Diagnostic>> {
    let mut has_digit = false;
    while *pos < chars.len() && ((chars[*pos] == '0' || chars[*pos] == '1') || chars[*pos] == '_') {
        *pos += 1;
        has_digit = true;
    }

    if !has_digit {
        return Err(
            Diagnostic::error("Missing digits after the integer base prefix")
                .add_help("Expect at least one binary digits after the prefix 0b")
                .add_help("Binary digit mean 0 or 1")
                .with_location_span(*start, *pos)
                .as_boxed(),
        );
    }

    let literal = &chars[*start..*pos];
    let string: String = literal.iter().collect();
    let literal_num = string.replace('_', "");
    match i64::from_str_radix(&literal_num, 2) {
        Ok(integer) => {
            let location = Location::new(*start, *pos);
            Ok(Token::new(TokenKind::Integer(integer), location))
        }
        Err(parse_int_error) => Err(Diagnostic::error(&parse_int_error.to_string())
            .add_note(&format!(
                "Value must be between {} and {}",
                i64::MIN,
                i64::MAX
            ))
            .with_location_span(*start, *pos)
            .as_boxed()),
    }
}

fn consume_octal_number(
    chars: &[char],
    pos: &mut usize,
    start: &mut usize,
) -> Result<Token, Box<Diagnostic>> {
    let mut has_digit = false;
    while *pos < chars.len() && ((chars[*pos] >= '0' && chars[*pos] < '8') || chars[*pos] == '_') {
        *pos += 1;
        has_digit = true;
    }

    if !has_digit {
        return Err(
            Diagnostic::error("Missing digits after the integer base prefix")
                .add_help("Expect at least one octal digits after the prefix 0o")
                .add_help("Octal digit mean 0 to 8 number")
                .with_location_span(*start, *pos)
                .as_boxed(),
        );
    }

    let literal = &chars[*start..*pos];
    let string: String = literal.iter().collect();
    let literal_num = string.replace('_', "");
    match i64::from_str_radix(&literal_num, 8) {
        Ok(integer) => {
            let location = Location::new(*start, *pos);
            Ok(Token::new(TokenKind::Integer(integer), location))
        }
        Err(parse_int_error) => Err(Diagnostic::error(&parse_int_error.to_string())
            .add_note(&format!(
                "Value must be between {} and {}",
                i64::MIN,
                i64::MAX
            ))
            .with_location_span(*start, *pos)
            .as_boxed()),
    }
}

fn consume_hex_number(
    chars: &[char],
    pos: &mut usize,
    start: &mut usize,
) -> Result<Token, Box<Diagnostic>> {
    let mut has_digit = false;
    while *pos < chars.len() && (chars[*pos].is_ascii_hexdigit() || chars[*pos] == '_') {
        *pos += 1;
        has_digit = true;
    }

    if !has_digit {
        return Err(
            Diagnostic::error("Missing digits after the integer base prefix")
                .add_help("Expect at least one hex digits after the prefix 0x")
                .add_help("Hex digit mean 0 to 9 and a to f")
                .with_location_span(*start, *pos)
                .as_boxed(),
        );
    }

    let literal = &chars[*start..*pos];
    let string: String = literal.iter().collect();
    let literal_num = string.replace('_', "");

    match i64::from_str_radix(&literal_num, 16) {
        Ok(integer) => {
            let location = Location::new(*start, *pos);
            Ok(Token::new(TokenKind::Integer(integer), location))
        }
        Err(parse_int_error) => Err(Diagnostic::error(&parse_int_error.to_string())
            .add_note(&format!(
                "Value must be between {} and {}",
                i64::MIN,
                i64::MAX
            ))
            .with_location_span(*start, *pos)
            .as_boxed()),
    }
}

fn consume_string_in_single_quotes(
    chars: &[char],
    pos: &mut usize,
    start: &mut usize,
) -> Result<Token, Box<Diagnostic>> {
    *pos += 1;

    while *pos < chars.len() && chars[*pos] != '\'' {
        *pos += 1;
    }

    if *pos >= chars.len() {
        return Err(Diagnostic::error("Unterminated single quote string")
            .add_help("Add \' at the end of the String literal")
            .with_location_span(*start, *pos)
            .as_boxed());
    }

    *pos += 1;

    let literal = &chars[*start + 1..*pos - 1];
    let string: String = literal.iter().collect();
    let location = Location::new(*start, *pos);
    Ok(Token::new(TokenKind::String(string), location))
}

fn consume_string_in_double_quotes(
    chars: &[char],
    pos: &mut usize,
    start: &mut usize,
) -> Result<Token, Box<Diagnostic>> {
    *pos += 1;

    while *pos < chars.len() && chars[*pos] != '"' {
        *pos += 1;
    }

    if *pos >= chars.len() {
        return Err(Diagnostic::error("Unterminated double quote string")
            .add_help("Add \" at the end of the String literal")
            .with_location_span(*start, *pos)
            .as_boxed());
    }

    *pos += 1;

    let literal = &chars[*start + 1..*pos - 1];
    let string: String = literal.iter().collect();

    let location = Location::new(*start, *pos);
    Ok(Token::new(TokenKind::String(string), location))
}

fn ignore_single_line_comment(chars: &[char], pos: &mut usize) {
    *pos += 2;

    while *pos < chars.len() && chars[*pos] != '\n' {
        *pos += 1;
    }

    *pos += 1;
}

fn ignore_c_style_comment(chars: &[char], pos: &mut usize) -> Result<(), Box<Diagnostic>> {
    *pos += 2;

    while *pos + 1 < chars.len() && (chars[*pos] != '*' && chars[*pos + 1] != '/') {
        *pos += 1;
    }

    if *pos + 2 > chars.len() {
        return Err(Diagnostic::error("C Style comment must end with */")
            .add_help("Add */ at the end of C Style comments")
            .with_location_span(*pos, *pos)
            .as_boxed());
    }

    *pos += 2;
    Ok(())
}
