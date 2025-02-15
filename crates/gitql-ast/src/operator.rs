#[derive(Clone, PartialEq)]
pub enum PrefixUnaryOperator {
    Negative,
    Bang,
    Not,
}

#[derive(Clone, PartialEq)]
pub enum ArithmeticOperator {
    Plus,
    Minus,
    Star,
    Slash,
    Modulus,
    Exponentiation,
}

#[derive(Clone, PartialEq)]
pub enum ComparisonOperator {
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Equal,
    NotEqual,
    NullSafeEqual,
}

#[derive(Clone, PartialEq)]
pub enum GroupComparisonOperator {
    All,
    Any,
    Some,
}

#[derive(Clone, PartialEq)]
pub enum BinaryLogicalOperator {
    Or,
    And,
    Xor,
}

#[derive(Clone, PartialEq)]
pub enum BinaryBitwiseOperator {
    Or,
    And,
    Xor,
    RightShift,
    LeftShift,
}
