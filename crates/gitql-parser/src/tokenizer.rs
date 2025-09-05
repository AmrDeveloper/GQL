use crate::diagnostic::Diagnostic;
use crate::token::SourceLocation;
use crate::token::Token;
use crate::token::TokenKind;

pub struct Tokenizer {
    pub(crate) content: Vec<char>,
    pub(crate) content_len: usize,
    pub(crate) index: usize,

    pub(crate) line_start: u32,
    pub(crate) line_end: u32,
    pub(crate) column_start: u32,
    pub(crate) column_end: u32,
}

impl Tokenizer {
    pub(crate) fn new(chars: Vec<char>) -> Tokenizer {
        let content_len = chars.len();
        Tokenizer {
            content: chars,
            content_len,
            index: 0,

            line_start: 1,
            line_end: 1,
            column_start: 0,
            column_end: 0,
        }
    }

    pub fn tokenize(content: String) -> Result<Vec<Token>, Box<Diagnostic>> {
        let mut tokenizer = Tokenizer::new(content.chars().collect());
        tokenizer.tokenize_characters()
    }

    fn current_source_location(&self) -> SourceLocation {
        SourceLocation {
            line_start: self.line_start,
            line_end: self.line_end,
            column_start: self.column_start,
            column_end: self.column_end,
        }
    }

    fn tokenize_characters(&mut self) -> Result<Vec<Token>, Box<Diagnostic>> {
        let mut tokens: Vec<Token> = Vec::new();
        let len = self.content_len;

        while self.has_next() {
            self.column_start = self.column_end;
            self.line_start = self.line_end;

            let char = self.content[self.index];

            // Symbol
            if char.is_alphabetic() {
                tokens.push(self.consume_identifier());
                continue;
            }

            // @> or Global Variable Symbol
            if char == '@' {
                // @>
                if self.index + 1 < len && self.content[self.index + 1] == '>' {
                    self.index += 2;
                    let location = self.current_source_location();
                    tokens.push(Token::new(TokenKind::AtRightArrow, location));
                    continue;
                }

                tokens.push(self.consume_global_variable_name()?);
                continue;
            }

            // Number
            if char.is_numeric() {
                if char == '0' && self.index + 1 < len {
                    match self.content[self.index + 1] {
                        // bindigits
                        'b' | 'B' => {
                            self.index += 2;
                            self.column_start += 2;
                            tokens.push(self.consume_binary_number()?);
                            continue;
                        }
                        // hexdigits
                        'x' | 'X' => {
                            self.index += 2;
                            self.column_start += 2;
                            tokens.push(self.consume_hex_number()?);
                            continue;
                        }
                        // octdigits
                        'o' | 'O' => {
                            self.index += 2;
                            self.column_start += 2;
                            tokens.push(self.consume_octal_number()?);
                            continue;
                        }
                        _ => {
                            tokens.push(self.consume_number()?);
                            continue;
                        }
                    }
                }

                tokens.push(self.consume_number()?);
                continue;
            }

            // String literal between single quotes '...'
            if char == '\'' {
                tokens.push(self.consume_string_in_single_quotes()?);
                continue;
            }

            // String literal between double quotes "..."
            if char == '"' {
                tokens.push(self.consume_string_in_double_quotes()?);
                continue;
            }

            // All chars between two backticks should be consumed as identifier
            if char == '`' {
                tokens.push(self.consume_backticks_identifier()?);
                continue;
            }

            // Plus
            if char == '+' {
                let location = self.current_source_location();
                tokens.push(Token::new(TokenKind::Plus, location));
                self.advance();
                continue;
            }

            // Minus
            if char == '-' {
                // Ignore single line comment which from -- until the end of the current line
                if self.index + 1 < self.content_len && self.content[self.index + 1] == '-' {
                    self.ignore_single_line_comment();
                    continue;
                }

                let location = self.current_source_location();
                tokens.push(Token::new(TokenKind::Minus, location));
                self.advance();
                continue;
            }

            // Star
            if char == '*' {
                let location = self.current_source_location();
                tokens.push(Token::new(TokenKind::Star, location));
                self.advance();
                continue;
            }

            // Slash
            if char == '/' {
                // Ignore C style comment which from /* comment */
                if self.index + 1 < self.content_len && self.content[self.index + 1] == '*' {
                    self.ignore_c_style_comment()?;
                    continue;
                }

                let location = self.current_source_location();
                tokens.push(Token::new(TokenKind::Slash, location));
                self.advance();
                continue;
            }

            // Percentage
            if char == '%' {
                let location = self.current_source_location();
                tokens.push(Token::new(TokenKind::Percentage, location));
                self.advance();
                continue;
            }

            // Caret
            if char == '^' {
                let location = self.current_source_location();
                tokens.push(Token::new(TokenKind::Caret, location));
                self.advance();
                continue;
            }

            // Bitwise NOT
            if char == '~' {
                let location = self.current_source_location();
                tokens.push(Token::new(TokenKind::BitwiseNot, location));
                self.advance();
                continue;
            }

            // Or
            if char == '|' {
                let location = self.current_source_location();

                self.advance();
                let kind = if self.index < len && self.content[self.index] == '|' {
                    self.advance();
                    TokenKind::OrOr
                } else {
                    TokenKind::BitwiseOr
                };

                tokens.push(Token::new(kind, location));
                continue;
            }

            // And
            if char == '&' {
                let location = self.current_source_location();

                self.advance();
                let kind = if self.index < len && self.content[self.index] == '&' {
                    self.advance();
                    TokenKind::AndAnd
                } else {
                    TokenKind::BitwiseAnd
                };

                tokens.push(Token::new(kind, location));
                continue;
            }

            // xor
            if char == '#' {
                let location = self.current_source_location();
                tokens.push(Token::new(TokenKind::BitwiseXor, location));
                self.advance();
                continue;
            }

            // Comma
            if char == ',' {
                let location = self.current_source_location();
                tokens.push(Token::new(TokenKind::Comma, location));
                self.advance();
                continue;
            }

            // Dot
            if char == '.' {
                let location = self.current_source_location();
                tokens.push(Token::new(TokenKind::Dot, location));
                self.advance();
                continue;
            }

            // Greater or GreaterEqual
            if char == '>' {
                let location = self.current_source_location();

                self.advance();
                let kind = if self.index < len && self.content[self.index] == '=' {
                    self.advance();
                    TokenKind::GreaterEqual
                } else if self.index < len && self.content[self.index] == '>' {
                    self.advance();
                    TokenKind::BitwiseRightShift
                } else {
                    TokenKind::Greater
                };

                tokens.push(Token::new(kind, location));
                continue;
            }

            // Less, LessEqual or NULL-safe equal
            if char == '<' {
                let location = self.current_source_location();

                self.advance();
                let kind = if self.index < len && self.content[self.index] == '=' {
                    self.advance();
                    if self.index < len && self.content[self.index] == '>' {
                        self.advance();
                        TokenKind::NullSafeEqual
                    } else {
                        TokenKind::LessEqual
                    }
                } else if self.index < len && self.content[self.index] == '<' {
                    self.advance();
                    TokenKind::BitwiseLeftShift
                } else if self.index < len && self.content[self.index] == '>' {
                    self.advance();
                    TokenKind::BangEqual
                } else if self.index < len && self.content[self.index] == '@' {
                    self.advance();
                    TokenKind::ArrowRightAt
                } else {
                    TokenKind::Less
                };

                tokens.push(Token::new(kind, location));
                continue;
            }

            // Equal
            if char == '=' {
                let location = self.current_source_location();
                tokens.push(Token::new(TokenKind::Equal, location));
                self.advance();
                continue;
            }

            // Colon , ColonColon or Colon Equal
            if char == ':' {
                let location = self.current_source_location();

                // :=
                if self.index + 1 < len && self.content[self.index + 1] == '=' {
                    tokens.push(Token::new(TokenKind::ColonEqual, location));
                    // Advance `:=`
                    self.advance_n(2);
                    continue;
                }

                // ::
                if self.index + 1 < len && self.content[self.index + 1] == ':' {
                    tokens.push(Token::new(TokenKind::ColonColon, location));
                    // Advance `::`
                    self.advance_n(2);
                    continue;
                }

                tokens.push(Token::new(TokenKind::Colon, location));
                self.advance();
                continue;
            }

            // Bang or Bang Equal
            if char == '!' {
                let location = self.current_source_location();

                // Consume `!`
                self.advance();
                let kind = if self.index < len && self.content[self.index] == '=' {
                    // Consume `=`
                    self.advance();
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                };

                tokens.push(Token::new(kind, location));
                continue;
            }

            // Left Paren
            if char == '(' {
                let location = self.current_source_location();
                tokens.push(Token::new(TokenKind::LeftParen, location));
                self.advance();
                continue;
            }

            // Right Paren
            if char == ')' {
                let location = self.current_source_location();
                tokens.push(Token::new(TokenKind::RightParen, location));
                self.advance();
                continue;
            }

            // Left Bracket
            if char == '[' {
                let location = self.current_source_location();
                tokens.push(Token::new(TokenKind::LeftBracket, location));
                self.advance();
                continue;
            }

            // Right Bracket
            if char == ']' {
                let location = self.current_source_location();
                tokens.push(Token::new(TokenKind::RightBracket, location));
                self.advance();
                continue;
            }

            // Semicolon
            if char == ';' {
                let location = self.current_source_location();
                tokens.push(Token::new(TokenKind::Semicolon, location));
                self.advance();
                continue;
            }

            // Characters to ignoring
            if char == ' ' || char == '\t' {
                self.advance();
                continue;
            }

            if char == '\n' {
                self.advance();
                self.column_end = 0;
                self.line_end += 1;
                continue;
            }

            return Err(Diagnostic::error("Unexpected character")
                .with_location(self.current_source_location())
                .as_boxed());
        }

        Ok(tokens)
    }

    fn consume_global_variable_name(&mut self) -> Result<Token, Box<Diagnostic>> {
        let start_index = self.index;

        // Advance `@`
        self.advance();

        // Make sure first character is  alphabetic
        if self.has_next() && !self.content[self.index].is_alphabetic() {
            return Err(Diagnostic::error(
                "Global variable name must start with alphabetic character",
            )
            .add_help("Add at least one alphabetic character after @")
            .with_location(self.current_source_location())
            .as_boxed());
        }

        while self.has_next() && self.is_current_char_func(|c| c == '_' || c.is_alphanumeric()) {
            self.advance();
        }

        // Identifier is being case-insensitive by default, convert to lowercase to be easy to compare and lookup
        let literal = &self.content[start_index..self.index];
        let mut string: String = literal.iter().collect();
        string = string.to_lowercase();

        let location = self.current_source_location();
        Ok(Token::new(TokenKind::GlobalVariable(string), location))
    }

    fn consume_identifier(&mut self) -> Token {
        let start_index = self.index;

        while self.has_next() && self.is_current_char_func(|c| c == '_' || c.is_alphanumeric()) {
            self.advance();
        }

        // Identifier is being case-insensitive by default, convert to lowercase to be easy to compare and lookup
        let literal = &self.content[start_index..self.index];
        let mut string: String = literal.iter().collect();
        string = string.to_lowercase();

        let location = self.current_source_location();
        Token::new_symbol(string, location)
    }

    fn consume_backticks_identifier(&mut self) -> Result<Token, Box<Diagnostic>> {
        let start_index = self.index;

        // Advance '`'
        self.advance();

        while self.has_next() && !self.is_current_char('`') {
            self.advance();
        }

        if self.index >= self.content_len {
            return Err(Diagnostic::error("Unterminated backticks")
                .add_help("Add ` at the end of the identifier")
                .with_location(self.current_source_location())
                .as_boxed());
        }

        // Advance '`'
        self.advance();

        let literal = &self.content[start_index + 1..self.index - 1];
        let identifier: String = literal.iter().collect();
        let location = self.current_source_location();
        Ok(Token::new(TokenKind::Symbol(identifier), location))
    }

    fn consume_number(&mut self) -> Result<Token, Box<Diagnostic>> {
        let start_index = self.index;

        while self.has_next() && self.is_current_char_func(|c| c == '_' || c.is_numeric()) {
            self.advance();
        }

        let mut is_float_value = false;
        if self.has_next() && self.is_current_char('.') {
            self.advance();

            is_float_value = true;
            while self.has_next() && self.is_current_char_func(|c| c == '_' || c.is_numeric()) {
                self.advance();
            }
        }

        let literal = &self.content[start_index..self.index];
        let string: String = literal.iter().collect();
        let literal_num = string.replace('_', "");
        let location = self.current_source_location();

        if is_float_value {
            return match literal_num.parse::<f64>() {
                Ok(float) => Ok(Token::new(TokenKind::Float(float), location)),
                Err(parse_float_error) => Err(Diagnostic::error(&parse_float_error.to_string())
                    .add_note(&format!(
                        "Value must be between {} and {}",
                        f64::MIN,
                        f64::MAX
                    ))
                    .with_location(self.current_source_location())
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
                .with_location(self.current_source_location())
                .as_boxed()),
        }
    }

    fn consume_binary_number(&mut self) -> Result<Token, Box<Diagnostic>> {
        let start_index = self.index;
        while self.has_next() && self.is_current_char_func(|c| c == '_' || c == '0' || c >= '1') {
            self.advance();
        }

        if start_index == self.index {
            return Err(
                Diagnostic::error("Missing digits after the integer base prefix")
                    .add_help("Expect at least one binary digits after the prefix 0b")
                    .add_help("Binary digit mean 0 or 1")
                    .with_location(self.current_source_location())
                    .as_boxed(),
            );
        }

        let literal = &self.content[start_index..self.index];
        let string: String = literal.iter().collect();
        let literal_num = string.replace('_', "");

        const BINARY_RADIX: u32 = 2;
        match i64::from_str_radix(&literal_num, BINARY_RADIX) {
            Ok(integer) => {
                let location = self.current_source_location();
                Ok(Token::new(TokenKind::Integer(integer), location))
            }
            Err(parse_int_error) => Err(Diagnostic::error(&parse_int_error.to_string())
                .add_note(&format!(
                    "Value must be between {} and {}",
                    i64::MIN,
                    i64::MAX
                ))
                .with_location(self.current_source_location())
                .as_boxed()),
        }
    }

    fn consume_octal_number(&mut self) -> Result<Token, Box<Diagnostic>> {
        let start_index = self.index;
        while self.has_next() && self.is_current_char_func(|c| c == '_' || ('0'..='8').contains(&c))
        {
            self.advance();
        }

        if start_index == self.index {
            return Err(
                Diagnostic::error("Missing digits after the integer base prefix")
                    .add_help("Expect at least one octal digits after the prefix 0o")
                    .add_help("Octal digit mean 0 to 8 number")
                    .with_location(self.current_source_location())
                    .as_boxed(),
            );
        }

        let literal = &self.content[start_index..self.index];
        let string: String = literal.iter().collect();
        let literal_num = string.replace('_', "");

        const OCTAL_RADIX: u32 = 8;
        match i64::from_str_radix(&literal_num, OCTAL_RADIX) {
            Ok(integer) => {
                let location = self.current_source_location();
                Ok(Token::new(TokenKind::Integer(integer), location))
            }
            Err(parse_int_error) => Err(Diagnostic::error(&parse_int_error.to_string())
                .add_note(&format!(
                    "Value must be between {} and {}",
                    i64::MIN,
                    i64::MAX
                ))
                .with_location(self.current_source_location())
                .as_boxed()),
        }
    }

    fn consume_hex_number(&mut self) -> Result<Token, Box<Diagnostic>> {
        let start_index = self.index;
        while self.has_next() && self.is_current_char_func(|c| c == '_' || c.is_ascii_hexdigit()) {
            self.advance();
        }

        if start_index == self.index {
            return Err(
                Diagnostic::error("Missing digits after the integer base prefix")
                    .add_help("Expect at least one hex digits after the prefix 0x")
                    .add_help("Hex digit mean 0 to 9 and a to f")
                    .with_location(self.current_source_location())
                    .as_boxed(),
            );
        }

        let literal = &self.content[start_index..self.index];
        let string: String = literal.iter().collect();
        let literal_num = string.replace('_', "");

        const HEX_RADIX: u32 = 16;
        match i64::from_str_radix(&literal_num, HEX_RADIX) {
            Ok(integer) => {
                let location = self.current_source_location();
                Ok(Token::new(TokenKind::Integer(integer), location))
            }
            Err(parse_int_error) => Err(Diagnostic::error(&parse_int_error.to_string())
                .add_note(&format!(
                    "Value must be between {} and {}",
                    i64::MIN,
                    i64::MAX
                ))
                .with_location(self.current_source_location())
                .as_boxed()),
        }
    }

    fn consume_string_in_single_quotes(&mut self) -> Result<Token, Box<Diagnostic>> {
        let buffer = self.consume_string_with_around('\'')?;

        if self.index >= self.content_len {
            return Err(Diagnostic::error("Unterminated single quote string")
                .add_help("Add \' at the end of the String literal")
                .with_location(self.current_source_location())
                .as_boxed());
        }

        // Consume `'`
        self.advance();
        let location = self.current_source_location();
        Ok(Token::new(TokenKind::String(buffer), location))
    }

    fn consume_string_in_double_quotes(&mut self) -> Result<Token, Box<Diagnostic>> {
        let buffer = self.consume_string_with_around('"')?;

        if self.index >= self.content_len {
            return Err(Diagnostic::error("Unterminated double quote string")
                .add_help("Add \" at the end of the String literal")
                .with_location(self.current_source_location())
                .as_boxed());
        }

        // Consume `"`
        self.advance();
        let location = self.current_source_location();
        Ok(Token::new(TokenKind::String(buffer), location))
    }

    fn consume_string_with_around(&mut self, around: char) -> Result<String, Box<Diagnostic>> {
        // Consume Around start
        self.advance();

        let mut buffer = String::new();
        while self.has_next() && self.content[self.index] != around {
            if !self.is_current_char('\\') {
                buffer.push(self.content[self.index]);
                self.advance();
                continue;
            }

            // If '\\' is the last character, we don't need to escape it
            if self.is_last() {
                buffer.push(self.content[self.index]);
                self.advance();
                continue;
            }

            // Consume '\\'
            self.advance();

            // Check possible escape depending on the next character
            let next_char = self.content[self.index];
            let character_with_escape_handled = match next_char {
                // Single quote
                '\'' => {
                    self.advance();
                    '\''
                }
                // Double quote
                '\"' => {
                    self.advance();
                    '\"'
                }
                // Backslash
                '\\' => {
                    self.advance();
                    '\\'
                }
                // New line
                'n' => {
                    self.advance();
                    '\n'
                }
                // Carriage return
                'r' => {
                    self.advance();
                    '\r'
                }
                // Tab
                't' => {
                    self.advance();
                    '\t'
                }
                _ => self.content[self.index - 1],
            };

            buffer.push(character_with_escape_handled);
        }

        Ok(buffer)
    }

    fn ignore_single_line_comment(&mut self) {
        // Advance `--`
        self.advance_n(2);

        while self.has_next() && !self.is_current_char('\n') {
            self.advance();
        }

        // Advance `\n`
        self.advance();
        self.line_end += 1;
        self.column_end = 0;
    }

    fn ignore_c_style_comment(&mut self) -> Result<(), Box<Diagnostic>> {
        // Advance `/*`
        self.advance_n(2);

        while self.index + 1 < self.content_len
            && (!self.is_current_char('*') && self.content[self.index + 1] != '/')
        {
            // Advance char
            self.advance();
        }

        if self.index + 2 > self.content_len {
            return Err(Diagnostic::error("C Style comment must end with */")
                .add_help("Add */ at the end of C Style comments")
                .with_location(self.current_source_location())
                .as_boxed());
        }

        // Advance `*/`
        self.advance_n(2);
        Ok(())
    }

    fn advance(&mut self) {
        self.index += 1;
        self.column_end += 1;
    }

    fn advance_n(&mut self, n: usize) {
        self.index += n;
        self.column_end += n as u32;
    }

    fn is_current_char(&self, ch: char) -> bool {
        self.content[self.index] == ch
    }

    fn is_current_char_func(&self, func: fn(char) -> bool) -> bool {
        func(self.content[self.index])
    }

    fn has_next(&self) -> bool {
        self.index < self.content_len
    }

    fn is_last(&self) -> bool {
        self.index == self.content_len - 1
    }
}
