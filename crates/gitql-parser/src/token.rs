use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(PartialEq)]
pub enum TokenKind {
    Do,
    Set,
    Select,
    Distinct,
    From,
    Group,
    Where,
    Having,
    Limit,
    Offset,
    Order,
    Using,
    Like,
    Glob,
    Describe,
    Show,
    RegExp,
    Array,
    Cast,
    Benchmark,
    Join,
    Left,
    Right,
    Cross,
    Inner,
    Outer,
    Case,
    When,
    Then,
    Else,
    End,
    Into,
    Outfile,
    Dumpfile,
    Lines,
    Fields,
    Enclosed,
    Terminated,
    Between,
    By,
    In,
    Is,
    On,
    Not,
    As,
    With,
    Rollup,
    OrKeyword,
    AndKeyword,
    XorKeyword,
    Ascending,
    Descending,
    Symmetric,
    Asymmetric,
    Window,
    Over,
    Partition,
    First,
    Last,
    Interval,

    // Values
    Symbol(String),
    GlobalVariable(String),
    String(String),
    Integer(i64),
    Float(f64),
    True,
    False,
    Null,
    Nulls,
    Infinity,
    NaN,

    All,
    Some,
    Any,

    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Equal,
    Bang,
    BangEqual,
    NullSafeEqual,
    AtRightArrow,
    ArrowRightAt,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    OrOr,
    AndAnd,
    BitwiseNot,
    BitwiseXor,
    BitwiseOr,
    BitwiseAnd,
    BitwiseRightShift,
    BitwiseLeftShift,
    Colon,
    ColonColon,
    ColonEqual,
    Plus,
    Minus,
    Star,
    Slash,
    Percentage,
    Caret,
    Comma,
    Dot,
    Semicolon,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let literal = match self {
            // Reserved Keywords
            TokenKind::Do => "DO",
            TokenKind::Set => "SET",
            TokenKind::Select => "SELECT",
            TokenKind::Distinct => "DISTINCT",
            TokenKind::From => "FROM",
            TokenKind::Group => "GROUP",
            TokenKind::Where => "WHERE",
            TokenKind::Having => "HAVING",
            TokenKind::Limit => "LIMIT",
            TokenKind::Offset => "OFFSET",
            TokenKind::Order => "ORDER",
            TokenKind::Using => "USING",
            TokenKind::Like => "LIKE",
            TokenKind::Glob => "GLOB",
            TokenKind::Describe => "DESCRIBE",
            TokenKind::Show => "SHOW",
            TokenKind::RegExp => "REGEXP",
            TokenKind::Array => "ARRAY",
            TokenKind::Cast => "CAST",
            TokenKind::Benchmark => "BENCHMARK",
            TokenKind::Join => "JOIN",
            TokenKind::Left => "LEFT",
            TokenKind::Right => "RIGHT",
            TokenKind::Cross => "CROSS",
            TokenKind::Inner => "INNER",
            TokenKind::Outer => "OUTER",
            TokenKind::Case => "CASE",
            TokenKind::When => "WHEN",
            TokenKind::Then => "THEN",
            TokenKind::Else => "ELSE",
            TokenKind::End => "END",
            TokenKind::Into => "INTO",
            TokenKind::Outfile => "OUTFILE",
            TokenKind::Dumpfile => "DUMPFILE",
            TokenKind::Lines => "LINES",
            TokenKind::Fields => "FIELDS",
            TokenKind::Enclosed => "ENCLOSED",
            TokenKind::Terminated => "TERMINATED",
            TokenKind::Between => "BETWEEN",
            TokenKind::By => "BY",
            TokenKind::In => "IN",
            TokenKind::Is => "IS",
            TokenKind::On => "ON",
            TokenKind::Not => "NOT",
            TokenKind::As => "AS",
            TokenKind::With => "WITH",
            TokenKind::Rollup => "ROLLUP",
            TokenKind::OrKeyword => "OR",
            TokenKind::AndKeyword => "AND",
            TokenKind::XorKeyword => "XOE",
            TokenKind::Ascending => "ASC",
            TokenKind::Descending => "DESC",
            TokenKind::Symmetric => "SYMMETRIC",
            TokenKind::Asymmetric => "ASYMMETRIC",
            TokenKind::Window => "WINDOW",
            TokenKind::Over => "OVER",
            TokenKind::Partition => "PARTITION",
            TokenKind::Nulls => "NULLS",
            TokenKind::First => "FIRST",
            TokenKind::Last => "LAST",
            TokenKind::Interval => "INTERVAL",

            // Values
            TokenKind::Symbol(literal) => literal,
            TokenKind::GlobalVariable(literal) => literal,
            TokenKind::String(string) => string,
            TokenKind::Integer(integer) => &integer.to_string(),
            TokenKind::Float(float) => &float.to_string(),
            TokenKind::True => "True",
            TokenKind::False => "False",
            TokenKind::Null => "Null",
            TokenKind::Infinity => "Infinity",
            TokenKind::NaN => "NaN",

            // Group Operators
            TokenKind::All => "ALL",
            TokenKind::Some => "Some",
            TokenKind::Any => "Any",

            // Others
            TokenKind::Greater => ">",
            TokenKind::GreaterEqual => ">=",
            TokenKind::Less => "<",
            TokenKind::LessEqual => "<=",
            TokenKind::Equal => "=",
            TokenKind::Bang => "!",
            TokenKind::BangEqual => "!=",
            TokenKind::NullSafeEqual => "<=>",
            TokenKind::AtRightArrow => "@>",
            TokenKind::ArrowRightAt => "<@",
            TokenKind::LeftParen => "(",
            TokenKind::RightParen => ")",
            TokenKind::LeftBracket => "[",
            TokenKind::RightBracket => "]",
            TokenKind::OrOr => "||",
            TokenKind::AndAnd => "&&",
            TokenKind::BitwiseNot => "~",
            TokenKind::BitwiseXor => "#",
            TokenKind::BitwiseOr => "|",
            TokenKind::BitwiseAnd => "&",
            TokenKind::BitwiseRightShift => ">>",
            TokenKind::BitwiseLeftShift => "<<",
            TokenKind::Colon => ":",
            TokenKind::ColonColon => "::",
            TokenKind::ColonEqual => ":=",
            TokenKind::Plus => "+",
            TokenKind::Minus => "-",
            TokenKind::Star => "*",
            TokenKind::Slash => "/",
            TokenKind::Percentage => "%",
            TokenKind::Caret => "^",
            TokenKind::Comma => ",",
            TokenKind::Dot => ".",
            TokenKind::Semicolon => ";",
        };
        f.write_str(literal)
    }
}

#[derive(Copy, Clone)]
pub struct SourceLocation {
    pub line_start: u32,
    pub line_end: u32,
    pub column_start: u32,
    pub column_end: u32,
}

impl SourceLocation {
    pub fn new(
        line_start: u32,
        line_end: u32,
        column_start: u32,
        column_end: u32,
    ) -> SourceLocation {
        SourceLocation {
            line_start,
            line_end,
            column_start,
            column_end,
        }
    }

    pub fn expand_until(&mut self, location: SourceLocation) {
        self.column_end = location.column_end;
        self.line_end = location.line_end;
    }
}

impl Display for SourceLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_fmt(format_args!(
            "Loc(L {}:{}, C {}:{})",
            self.line_start, self.line_end, self.column_start, self.column_end
        ))
    }
}

pub struct Token {
    pub kind: TokenKind,
    pub location: SourceLocation,
}

impl Token {
    pub fn new(kind: TokenKind, location: SourceLocation) -> Token {
        Token { kind, location }
    }

    pub fn new_symbol(symbol: String, location: SourceLocation) -> Token {
        Token {
            kind: resolve_symbol_kind(symbol),
            location,
        }
    }

    pub fn has_kind(&self, kind: TokenKind) -> bool {
        self.kind == kind
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(&self.kind.to_string())
    }
}

fn resolve_symbol_kind(symbol: String) -> TokenKind {
    match symbol.to_lowercase().as_str() {
        // Reserved keywords
        "do" => TokenKind::Do,
        "set" => TokenKind::Set,
        "select" => TokenKind::Select,
        "distinct" => TokenKind::Distinct,
        "from" => TokenKind::From,
        "where" => TokenKind::Where,
        "limit" => TokenKind::Limit,
        "offset" => TokenKind::Offset,
        "order" => TokenKind::Order,
        "using" => TokenKind::Using,
        "case" => TokenKind::Case,
        "when" => TokenKind::When,
        "then" => TokenKind::Then,
        "else" => TokenKind::Else,
        "end" => TokenKind::End,
        "between" => TokenKind::Between,
        "in" => TokenKind::In,
        "is" => TokenKind::Is,
        "on" => TokenKind::On,
        "not" => TokenKind::Not,
        "like" => TokenKind::Like,
        "glob" => TokenKind::Glob,
        "describe" => TokenKind::Describe,
        "show" => TokenKind::Show,
        "regexp" => TokenKind::RegExp,

        "cast" => TokenKind::Cast,
        "benchmark" => TokenKind::Benchmark,

        "interval" => TokenKind::Interval,

        // Select into
        "into" => TokenKind::Into,
        "outfile" => TokenKind::Outfile,
        "dumpfile" => TokenKind::Dumpfile,
        "lines" => TokenKind::Lines,
        "fields" => TokenKind::Fields,
        "enclosed" => TokenKind::Enclosed,
        "terminated" => TokenKind::Terminated,

        // Joins
        "join" => TokenKind::Join,
        "left" => TokenKind::Left,
        "right" => TokenKind::Right,
        "cross" => TokenKind::Cross,
        "inner" => TokenKind::Inner,
        "outer" => TokenKind::Outer,

        // Grouping
        "group" => TokenKind::Group,
        "by" => TokenKind::By,
        "having" => TokenKind::Having,
        "with" => TokenKind::With,
        "rollup" => TokenKind::Rollup,

        // Between kind
        "symmetric" => TokenKind::Symmetric,
        "asymmetric" => TokenKind::Asymmetric,

        // Integer division and Modulo operator
        "div" => TokenKind::Slash,
        "mod" => TokenKind::Percentage,

        // Logical Operators
        "or" => TokenKind::OrKeyword,
        "and" => TokenKind::AndKeyword,
        "xor" => TokenKind::XorKeyword,

        // Group Operators
        "all" => TokenKind::All,
        "some" => TokenKind::Some,
        "any" => TokenKind::Any,

        // True, False and Null
        "true" => TokenKind::True,
        "false" => TokenKind::False,
        "null" => TokenKind::Null,
        "nulls" => TokenKind::Nulls,

        // Infinity and NaN
        "infinity" => TokenKind::Infinity,
        "nan" => TokenKind::NaN,

        // As for alias
        "as" => TokenKind::As,

        // Order by DES and ASC
        "asc" => TokenKind::Ascending,
        "desc" => TokenKind::Descending,

        // Order by null ordering policy Null first and last
        "first" => TokenKind::First,
        "last" => TokenKind::Last,

        // Array data type
        "array" => TokenKind::Array,

        // Over
        "window" => TokenKind::Window,
        "over" => TokenKind::Over,
        "partition" => TokenKind::Partition,

        // Identifier
        _ => TokenKind::Symbol(symbol),
    }
}
