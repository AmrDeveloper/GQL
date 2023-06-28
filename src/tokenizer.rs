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

    Or,
    And,
    Xor,

    Symbol,
    Number,
    String,

    True,
    False,

    Plus,
    Minus,
    Star,
    Slash,

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
            while position < len
                && (characters[position] == '_' || characters[position].is_alphabetic())
            {
                position += 1;
            }

            let literal = &script[column_start..position];
            let location = Location {
                start: column_start,
                end: position,
            };

            let token = Token {
                location: location,
                kind: resolve_symbol_kind(literal.to_string()),
                literal: literal.to_string(),
            };

            tokens.push(token);
            continue;
        }

        // Number
        if char.is_numeric() {
            while position < len && characters[position].is_numeric() {
                position += 1;
            }

            let literal = &script[column_start..position];
            let location = Location {
                start: column_start,
                end: position,
            };

            let token = Token {
                location: location,
                kind: TokenKind::Number,
                literal: literal.to_string(),
            };

            tokens.push(token);
            continue;
        }

        // String literal
        if char == '"' {
            position += 1;
            while position < len && characters[position] != '"' {
                position += 1;
            }
            position += 1;

            let literal = &script[column_start + 1..position - 1];

            let location = Location {
                start: column_start,
                end: position,
            };

            let token = Token {
                location: location,
                kind: TokenKind::String,
                literal: literal.to_string(),
            };

            tokens.push(token);
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

        // Plus
        if char == '-' {
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

        // Plus
        if char == '/' {
            let location = Location {
                start: column_start,
                end: position,
            };

            let token = Token {
                location: location,
                kind: TokenKind::Slash,
                literal: "/".to_owned(),
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

            let token = Token {
                location: location,
                kind: TokenKind::Or,
                literal: "|".to_owned(),
            };

            tokens.push(token);
            position += 1;
            continue;
        }

        // And
        if char == '&' {
            let location = Location {
                start: column_start,
                end: position,
            };

            let token = Token {
                location: location,
                kind: TokenKind::And,
                literal: "&".to_owned(),
            };

            tokens.push(token);
            position += 1;
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
                kind: TokenKind::Xor,
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

        // Dot
        if char == '.' {
            let location = Location {
                start: column_start,
                end: position,
            };

            let token = Token {
                location: location,
                kind: TokenKind::Dot,
                literal: ".".to_owned(),
            };

            tokens.push(token);
            position += 1;
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

        // Logical Operators
        "or" => TokenKind::Or,
        "and" => TokenKind::And,
        "xor" => TokenKind::Xor,

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
