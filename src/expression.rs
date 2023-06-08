use crate::object::GQLObject;
use regex::Regex;

pub trait Expression {
    fn evaluate(&self, object: &GQLObject) -> String;
}

pub struct StringExpression {
    pub value: String,
}

impl Expression for StringExpression {
    fn evaluate(&self, _object: &GQLObject) -> String {
        return self.value.to_owned();
    }
}

pub struct SymbolExpression {
    pub value: String,
}

impl Expression for SymbolExpression {
    fn evaluate(&self, _object: &GQLObject) -> String {
        return self.value.to_owned();
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
        let key = self.left.evaluate(object);
        let expected = self.right.evaluate(object);

        if object.attributes.contains_key(&key) {
            let field_value = object.attributes.get(&key).unwrap();
            let result = field_value.cmp(&expected);
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
        return "false".to_owned();
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
        let key = self.left.evaluate(object);
        let expected = self.right.evaluate(object);
        if object.attributes.contains_key(&key) {
            let value = object.attributes.get(&key).unwrap();
            return match self.operator {
                CheckOperator::Contains => value.contains(&expected),
                CheckOperator::StartsWith => value.starts_with(&expected),
                CheckOperator::EndsWith => value.ends_with(&expected),
                CheckOperator::Matches => {
                    let regex = Regex::new(&expected);
                    if regex.is_err() {
                        return "false".to_owned();
                    }
                    regex.unwrap().is_match(value)
                }
            }
            .to_string();
        }
        return "false".to_owned();
    }
}

#[derive(PartialEq)]
pub enum LogicalOperator {
    Or,
    And,
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
        }
        .to_string();
    }
}
