#[derive(PartialEq)]
pub enum TokenKind {
    Select,
    From,
    Group,
    Where,
    Having,
    Limit,
    Offset,
    Order,
    By,

    Between,
    DotDot,

    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Equal,
    Bang,

    Contains,
    StartsWith,
    EndsWith,
    Matches,

    As,

    LeftParen,
    RightParen,

    LogicalOr,
    LogicalAnd,
    LogicalXor,

    BitwiseOr,
    BitwiseAnd,
    BitwiseRightShift,
    BitwiseLeftShift,

    Symbol,
    Number,
    String,

    True,
    False,

    Plus,
    Minus,
    Star,
    Slash,
    Percentage,

    Comma,
    Dot,

    Ascending,
    Descending,
}

#[derive(Copy, Clone)]
pub struct Location {
    pub start: usize,
    pub end: usize,
}

pub struct Token {
    pub location: Location,
    pub kind: TokenKind,
    pub literal: String,
}

use crate::diagnostic::GQLError;

pub fn tokenize(script: String) -> Result<Vec<Token>, GQLError> {
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
            let identifier = consume_identifier(&characters, &mut position, &mut column_start);
            tokens.push(identifier);
            continue;
        }

        // Number
        if char.is_numeric() {
            if char == '0' && position + 1 < len {
                if characters[position + 1] == 'x' {
                    position += 2;
                    column_start += 2;
                    let number = consume_hex_number(&characters, &mut position, &mut column_start);
                    if number.is_err() {
                        return Err(number.err().unwrap());
                    }
                    tokens.push(number.ok().unwrap());
                    continue;
                }
            }

            let number = consume_number(&characters, &mut position, &mut column_start);
            tokens.push(number);
            continue;
        }

        // String literal
        if char == '"' {
            let result = consume_string(&characters, &mut position, &mut column_start);
            if result.is_err() {
                return Err(result.err().unwrap());
            }

            tokens.push(result.ok().unwrap());
            continue;
        }

        // Plus
        if char == '+' {
            let location = Location {
                start: column_start,
                end: position,
            };

            let token = Token {
                location: location,
                kind: TokenKind::Plus,
                literal: "+".to_owned(),
            };

            tokens.push(token);
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

            let location = Location {
                start: column_start,
                end: position,
            };

            let token = Token {
                location: location,
                kind: TokenKind::Minus,
                literal: "-".to_owned(),
            };

            tokens.push(token);
            position += 1;
            continue;
        }

        // Star
        if char == '*' {
            let location = Location {
                start: column_start,
                end: position,
            };

            let token = Token {
                location: location,
                kind: TokenKind::Star,
                literal: "*".to_owned(),
            };

            tokens.push(token);
            position += 1;
            continue;
        }

        // Slash
        if char == '/' {
            // Ignore C style comment which from /* comment */
            if position + 1 < characters.len() && characters[position + 1] == '*' {
                let result = ignore_c_style_comment(&characters, &mut position);
                if result.is_err() {
                    return Err(result.err().unwrap());
                }
                continue;
            }

            let location = Location {
                start: column_start,
                end: position,
            };

            let token = Token {
                location,
                kind: TokenKind::Slash,
                literal: "/".to_owned(),
            };

            tokens.push(token);
            position += 1;
            continue;
        }

        // Percentage
        if char == '%' {
            let location = Location {
                start: column_start,
                end: position,
            };

            let token = Token {
                location: location,
                kind: TokenKind::Percentage,
                literal: "%".to_owned(),
            };

            tokens.push(token);
            position += 1;
            continue;
        }

        // Or
        if char == '|' {
            let location = Location {
                start: column_start,
                end: position,
            };

            position += 1;

            let mut kind = TokenKind::BitwiseOr;
            let mut literal = "|";

            if position < len && characters[position] == '|' {
                position += 1;
                kind = TokenKind::LogicalOr;
                literal = "||";
            }

            let token = Token {
                location,
                kind,
                literal: literal.to_string(),
            };

            tokens.push(token);
            continue;
        }

        // And
        if char == '&' {
            let location = Location {
                start: column_start,
                end: position,
            };

            position += 1;
            let mut kind = TokenKind::BitwiseAnd;
            let mut literal = "&";

            if position < len && characters[position] == '&' {
                position += 1;
                kind = TokenKind::LogicalAnd;
                literal = "&&";
            }

            let token = Token {
                location,
                kind,
                literal: literal.to_string(),
            };

            tokens.push(token);
            continue;
        }

        // xor
        if char == '^' {
            let location = Location {
                start: column_start,
                end: position,
            };

            let token = Token {
                location: location,
                kind: TokenKind::LogicalXor,
                literal: "^".to_owned(),
            };

            tokens.push(token);
            position += 1;
            continue;
        }

        // Comma
        if char == ',' {
            let location = Location {
                start: column_start,
                end: position,
            };

            let token = Token {
                location: location,
                kind: TokenKind::Comma,
                literal: ",".to_owned(),
            };

            tokens.push(token);
            position += 1;
            continue;
        }

        // Dot or Range (DotDot)
        if char == '.' {
            let location = Location {
                start: column_start,
                end: position,
            };

            position += 1;

            let mut kind = TokenKind::Dot;
            let mut literal = ".";

            if position < len && characters[position] == '.' {
                position += 1;
                kind = TokenKind::DotDot;
                literal = "..";
            }

            let token = Token {
                location: location,
                kind,
                literal: literal.to_string(),
            };

            tokens.push(token);
            continue;
        }

        // Greater or GreaterEqual
        if char == '>' {
            let location = Location {
                start: column_start,
                end: position,
            };

            position += 1;

            let mut kind = TokenKind::Greater;
            let mut literal = ">";

            if position < len && characters[position] == '=' {
                position += 1;
                kind = TokenKind::GreaterEqual;
                literal = ">=";
            }

            let token = Token {
                location: location,
                kind: kind,
                literal: literal.to_string(),
            };

            tokens.push(token);
            continue;
        }

        // Less or LessEqual
        if char == '<' {
            let location = Location {
                start: column_start,
                end: position,
            };

            position += 1;

            let mut kind = TokenKind::Less;
            let mut literal = "<";

            if position < len && characters[position] == '=' {
                position += 1;
                kind = TokenKind::LessEqual;
                literal = "<=";
            }

            let token = Token {
                location: location,
                kind: kind,
                literal: literal.to_owned(),
            };

            tokens.push(token);
            continue;
        }

        // Equal
        if char == '=' {
            let location = Location {
                start: column_start,
                end: position,
            };

            let token = Token {
                location: location,
                kind: TokenKind::Equal,
                literal: "=".to_owned(),
            };

            tokens.push(token);
            position += 1;
            continue;
        }

        // Not
        if char == '!' {
            let location = Location {
                start: column_start,
                end: position,
            };

            let token = Token {
                location: location,
                kind: TokenKind::Bang,
                literal: "!".to_owned(),
            };

            tokens.push(token);
            position += 1;
            continue;
        }

        // Left Paren
        if char == '(' {
            let location = Location {
                start: column_start,
                end: position,
            };

            let token = Token {
                location: location,
                kind: TokenKind::LeftParen,
                literal: "(".to_owned(),
            };

            tokens.push(token);
            position += 1;
            continue;
        }

        // Right Paren
        if char == ')' {
            let location = Location {
                start: column_start,
                end: position,
            };

            let token = Token {
                location: location,
                kind: TokenKind::RightParen,
                literal: ")".to_owned(),
            };

            tokens.push(token);
            position += 1;
            continue;
        }

        // Characters to ignoring
        if char == ' ' || char == '\n' || char == '\t' {
            position += 1;
            continue;
        }

        return Err(GQLError {
            message: "Un expected character".to_owned(),
            location: Location {
                start: column_start,
                end: position,
            },
        });
    }

    return Ok(tokens);
}

fn consume_identifier(chars: &Vec<char>, pos: &mut usize, start: &mut usize) -> Token {
    while *pos < chars.len() && (chars[*pos] == '_' || chars[*pos].is_alphabetic()) {
        *pos += 1;
    }

    let literal = &chars[*start..*pos];
    let string = String::from_utf8(literal.iter().map(|&c| c as u8).collect()).unwrap();

    let location = Location {
        start: *start,
        end: *pos,
    };

    return Token {
        location,
        kind: resolve_symbol_kind(string.to_string()),
        literal: string.to_string(),
    };
}

fn consume_number(chars: &Vec<char>, pos: &mut usize, start: &mut usize) -> Token {
    while *pos < chars.len() && (chars[*pos].is_numeric() || chars[*pos] == '_') {
        *pos += 1;
    }

    let literal = &chars[*start..*pos];
    let string = String::from_utf8(literal.iter().map(|&c| c as u8).collect()).unwrap();
    let literal_num = string.to_string().replace("_", "");

    let location = Location {
        start: *start,
        end: *pos,
    };

    return Token {
        location,
        kind: TokenKind::Number,
        literal: literal_num,
    };
}

fn consume_hex_number(
    chars: &Vec<char>,
    pos: &mut usize,
    start: &mut usize,
) -> Result<Token, GQLError> {
    let mut has_digit = false;
    while *pos < chars.len() && (chars[*pos].is_ascii_hexdigit() || chars[*pos] == '_') {
        *pos += 1;
        has_digit = true;
    }

    if !has_digit {
        return Err(GQLError {
            message: "Missing digits after the integer base prefix".to_owned(),
            location: Location {
                start: *start,
                end: *pos,
            },
        });
    }

    let literal = &chars[*start..*pos];
    let string = String::from_utf8(literal.iter().map(|&c| c as u8).collect()).unwrap();
    let literal_num = string.to_string().replace("_", "");
    let convert_result = i64::from_str_radix(&literal_num, 16);

    if convert_result.is_err() {
        return Err(GQLError {
            message: "Invalid hex decinmal number".to_owned(),
            location: Location {
                start: *start,
                end: *pos,
            },
        });
    }

    let location = Location {
        start: *start,
        end: *pos,
    };

    return Ok(Token {
        location,
        kind: TokenKind::Number,
        literal: convert_result.ok().unwrap().to_string(),
    });
}

fn consume_string(
    chars: &Vec<char>,
    pos: &mut usize,
    start: &mut usize,
) -> Result<Token, GQLError> {
    *pos += 1;

    while *pos < chars.len() && chars[*pos] != '"' {
        *pos += 1;
    }

    if *pos >= chars.len() {
        return Err(GQLError {
            message: "Unterminated double quote string".to_owned(),
            location: Location {
                start: *start,
                end: *pos,
            },
        });
    }

    *pos += 1;

    let literal = &chars[*start + 1..*pos - 1];
    let string = String::from_utf8(literal.iter().map(|&c| c as u8).collect()).unwrap();

    let location = Location {
        start: *start,
        end: *pos,
    };

    let string_literal = Token {
        location,
        kind: TokenKind::String,
        literal: string.to_string(),
    };

    return Ok(string_literal);
}

fn ignore_single_line_comment(chars: &Vec<char>, pos: &mut usize) {
    *pos += 2;

    while *pos < chars.len() && chars[*pos] != '\n' {
        *pos += 1;
    }

    *pos += 1;
}

fn ignore_c_style_comment(chars: &Vec<char>, pos: &mut usize) -> Result<(), GQLError> {
    *pos += 2;

    while *pos + 1 < chars.len() && (chars[*pos] != '*' && chars[*pos + 1] != '/') {
        *pos += 1;
    }

    if *pos + 2 > chars.len() {
        return Err(GQLError {
            message: "C Style comment must end with */".to_owned(),
            location: Location {
                start: *pos,
                end: *pos,
            },
        });
    }

    *pos += 2;
    return Ok(());
}

fn resolve_symbol_kind(literal: String) -> TokenKind {
    return match literal.to_lowercase().as_str() {
        // Reserved keywords
        "select" => TokenKind::Select,
        "from" => TokenKind::From,
        "group" => TokenKind::Group,
        "where" => TokenKind::Where,
        "having" => TokenKind::Having,
        "limit" => TokenKind::Limit,
        "offset" => TokenKind::Offset,
        "order" => TokenKind::Order,
        "by" => TokenKind::By,

        "between" => TokenKind::Between,

        // Logical Operators
        "or" => TokenKind::LogicalOr,
        "and" => TokenKind::LogicalAnd,
        "xor" => TokenKind::LogicalXor,

        // True and False
        "true" => TokenKind::True,
        "false" => TokenKind::False,

        // String operators
        "contains" => TokenKind::Contains,
        "starts_with" => TokenKind::StartsWith,
        "ends_with" => TokenKind::EndsWith,
        "matches" => TokenKind::Matches,

        "as" => TokenKind::As,

        // Order by DES and ASC
        "asc" => TokenKind::Ascending,
        "des" => TokenKind::Descending,

        // Identifier
        _ => TokenKind::Symbol,
    };
}
