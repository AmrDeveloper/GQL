#[derive(PartialEq)]
pub enum PrefixUnaryOperator {
    Minus,
    Bang,
}

#[derive(PartialEq)]
pub enum ArithmeticOperator {
    Plus,
    Minus,
    Star,
    Slash,
    Modulus,
}

#[derive(PartialEq)]
pub enum ComparisonOperator {
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Equal,
    NotEqual,
    NullSafeEqual,
}

#[derive(PartialEq)]
pub enum LogicalOperator {
    Or,
    And,
    Xor,
}

#[derive(PartialEq)]
pub enum BitwiseOperator {
    Or,
    And,
    RightShift,
    LeftShift,
}
