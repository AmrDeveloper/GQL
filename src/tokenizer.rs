#[derive(PartialEq)]
pub enum TokenKind {
    Select,
    From,
    Where,
    Limit,
    Offset,
    Order,
    By,

    Equal,
    Or,
    And,

    Symbol,
    Number,
    String,

    Star,

    Comma,
}

pub struct Location {
    pub start: usize,
    pub end: usize,
}

pub struct Token {
    pub location: Location,
    pub kind: TokenKind,
    pub literal: String,
}

pub fn tokenize(script: String) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut position = 0;
    let mut column_start = 0;

    let characters: Vec<char> = script.chars().collect();
    let len = characters.len();

    while position < len {
        column_start = position;

        let char = characters[position];

        // Tokenize Symbol
        if char.is_alphabetic() {
            while position < len && characters[position].is_alphabetic() {
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

        // Tokenize Number
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

        // Characters to ignoring
        if char == ' ' || char == '\n' || char == '\t' {
            position += 1;
            continue;
        }

        return Err("Un expected character".to_owned());
    }

    return Ok(tokens);
}

fn resolve_symbol_kind(literal: String) -> TokenKind {
    return match literal.as_str() {
        "select" => TokenKind::Select,
        "from" => TokenKind::From,
        "where" => TokenKind::Where,
        "limit" => TokenKind::Limit,
        "offset" => TokenKind::Offset,
        "order" => TokenKind::Order,
        "by" => TokenKind::By,
        _ => TokenKind::Symbol,
    };
}
