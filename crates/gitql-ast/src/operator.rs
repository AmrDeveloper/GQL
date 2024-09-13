#[derive(PartialEq)]
pub enum PrefixUnaryOperator {
    Minus,
    Bang,
    Not,
}

#[derive(PartialEq)]
pub enum ArithmeticOperator {
    Plus,
    Minus,
    Star,
    Slash,
    Modulus,
    Exponentiation,
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
pub enum BinaryLogicalOperator {
    Or,
    And,
    Xor,
}

#[derive(PartialEq)]
pub enum BinaryBitwiseOperator {
    Or,
    And,
    RightShift,
    LeftShift,
}

#[derive(PartialEq)]
pub enum ContainsOperator {
    RangeContainsElement,
    RangeContainsRange,
}
