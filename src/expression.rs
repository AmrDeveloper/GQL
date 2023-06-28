use crate::object::GQLObject;
use crate::types::{DataType, TABLES_FIELDS_TYPES};
use regex::Regex;

use crate::transformation::{TRANSFORMATIONS, TRANSFORMATIONS_PROTOS};

pub trait Expression {
    fn evaluate(&self, object: &GQLObject) -> String;
    fn expr_type(&self) -> DataType;
}

pub struct StringExpression {
    pub value: String,
}

impl Expression for StringExpression {
    fn evaluate(&self, _object: &GQLObject) -> String {
        return self.value.to_owned();
    }

    fn expr_type(&self) -> DataType {
        return DataType::Text;
    }
}

pub struct SymbolExpression {
    pub value: String,
}

impl Expression for SymbolExpression {
    fn evaluate(&self, object: &GQLObject) -> String {
        return object.attributes.get(&self.value).unwrap().to_string();
    }

    fn expr_type(&self) -> DataType {
        return TABLES_FIELDS_TYPES
            .get(self.value.as_str())
            .unwrap()
            .clone();
    }
}

pub struct NumberExpression {
    pub value: i64,
}

impl Expression for NumberExpression {
    fn evaluate(&self, _object: &GQLObject) -> String {
        return self.value.to_string();
    }

    fn expr_type(&self) -> DataType {
        return DataType::Number;
    }
}

pub struct BooleanExpression {
    pub is_true: bool,
}

impl Expression for BooleanExpression {
    fn evaluate(&self, _object: &GQLObject) -> String {
        return if self.is_true {
            "true".to_owned()
        } else {
            "false".to_owned()
        };
    }

    fn expr_type(&self) -> DataType {
        return DataType::Boolean;
    }
}
pub struct NotExpression {
    pub right: Box<dyn Expression>,
}

impl Expression for NotExpression {
    fn evaluate(&self, object: &GQLObject) -> String {
        let value = self.right.evaluate(object);
        return (!value.eq("true")).to_string();
    }

    fn expr_type(&self) -> DataType {
        return DataType::Boolean;
    }
}

#[derive(PartialEq)]
pub enum ArithmeticOperator {
    Plus,
    Minus,
    Star,
    Slash,
}

pub struct ArithmeticExpression {
    pub left: Box<dyn Expression>,
    pub operator: ArithmeticOperator,
    pub right: Box<dyn Expression>,
}

impl Expression for ArithmeticExpression {
    fn evaluate(&self, object: &GQLObject) -> String {
        let lhs = self.left.evaluate(object).parse::<i64>().unwrap();
        let rhs = self.right.evaluate(object).parse::<i64>().unwrap();
        return match self.operator {
            ArithmeticOperator::Plus => lhs + rhs,
            ArithmeticOperator::Minus => lhs - rhs,
            ArithmeticOperator::Star => lhs * rhs,
            ArithmeticOperator::Slash => lhs / rhs,
        }
        .to_string();
    }

    fn expr_type(&self) -> DataType {
        return DataType::Number;
    }
}

#[derive(PartialEq)]
pub enum ComparisonOperator {
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Equal,
    NotEqual,
}

pub struct ComparisonExpression {
    pub left: Box<dyn Expression>,
    pub operator: ComparisonOperator,
    pub right: Box<dyn Expression>,
}

impl Expression for ComparisonExpression {
    fn evaluate(&self, object: &GQLObject) -> String {
        let value = self.left.evaluate(object);
        let expected = self.right.evaluate(object);

        let is_numbers = self.left.expr_type() == DataType::Text;
        let result = if is_numbers {
            value.cmp(&expected)
        } else {
            value
                .parse::<i64>()
                .unwrap()
                .cmp(&expected.parse::<i64>().unwrap())
        };

        return match self.operator {
            ComparisonOperator::Greater => result.is_gt(),
            ComparisonOperator::GreaterEqual => result.is_ge(),
            ComparisonOperator::Less => result.is_lt(),
            ComparisonOperator::LessEqual => result.is_le(),
            ComparisonOperator::Equal => result.is_eq(),
            ComparisonOperator::NotEqual => !result.is_eq(),
        }
        .to_string();
    }

    fn expr_type(&self) -> DataType {
        return DataType::Boolean;
    }
}

#[derive(PartialEq)]
pub enum CheckOperator {
    Contains,
    StartsWith,
    EndsWith,
    Matches,
}

pub struct CheckExpression {
    pub left: Box<dyn Expression>,
    pub operator: CheckOperator,
    pub right: Box<dyn Expression>,
}

impl Expression for CheckExpression {
    fn evaluate(&self, object: &GQLObject) -> String {
        let value = self.left.evaluate(object);
        let expected = self.right.evaluate(object);

        return match self.operator {
            CheckOperator::Contains => value.contains(&expected),
            CheckOperator::StartsWith => value.starts_with(&expected),
            CheckOperator::EndsWith => value.ends_with(&expected),
            CheckOperator::Matches => {
                let regex = Regex::new(&expected);
                if regex.is_err() {
                    return "false".to_owned();
                }
                regex.unwrap().is_match(&value)
            }
        }
        .to_string();
    }

    fn expr_type(&self) -> DataType {
        return DataType::Boolean;
    }
}

#[derive(PartialEq)]
pub enum LogicalOperator {
    Or,
    And,
    Xor,
}

pub struct LogicalExpression {
    pub left: Box<dyn Expression>,
    pub operator: LogicalOperator,
    pub right: Box<dyn Expression>,
}

impl Expression for LogicalExpression {
    fn evaluate(&self, object: &GQLObject) -> String {
        let lhs = self.left.evaluate(object).eq("true");

        if self.operator == LogicalOperator::And && !lhs {
            return "false".to_owned();
        }

        if self.operator == LogicalOperator::Or && lhs {
            return "true".to_owned();
        }

        let rhs = self.right.evaluate(object).eq("true");

        return match self.operator {
            LogicalOperator::And => lhs && rhs,
            LogicalOperator::Or => lhs || rhs,
            LogicalOperator::Xor => lhs ^ rhs,
        }
        .to_string();
    }

    fn expr_type(&self) -> DataType {
        return DataType::Boolean;
    }
}

pub struct CallExpression {
    pub callee: Box<dyn Expression>,
    pub arguments: Vec<Box<dyn Expression>>,
    pub function_name: String,
}

impl Expression for CallExpression {
    fn evaluate(&self, object: &GQLObject) -> String {
        let lhs = self.callee.evaluate(object);
        let transformation = TRANSFORMATIONS.get(self.function_name.as_str()).unwrap();
        return transformation(lhs);
    }

    fn expr_type(&self) -> DataType {
        let prototype = TRANSFORMATIONS_PROTOS
            .get(&self.function_name.as_str())
            .unwrap();
        return prototype.result.clone();
    }
}
