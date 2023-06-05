use crate::object::GQLObject;

pub trait Expression {
    fn evaluate(&self, object: &GQLObject) -> bool;
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
    pub field_name: String,
    pub operator: ComparisonOperator,
    pub expected_value: String,
}

impl Expression for ComparisonExpression {
    fn evaluate(&self, object: &GQLObject) -> bool {
        if object.attributes.contains_key(&self.field_name) {
            let value = object.attributes.get(&self.field_name).unwrap();
            let result = value.cmp(&self.expected_value);
            return match self.operator {
                ComparisonOperator::Greater => result.is_gt(),
                ComparisonOperator::GreaterEqual => result.is_ge(),
                ComparisonOperator::Less => result.is_lt(),
                ComparisonOperator::LessEqual => result.is_le(),
                ComparisonOperator::Equal => result.is_eq(),
                ComparisonOperator::NotEqual => !result.is_eq(),
            };
        }
        return false;
    }
}

#[derive(PartialEq)]
pub enum CheckOperator {
    Contains,
    StartsWith,
    EndsWith,
}

pub struct CheckExpression {
    pub field_name: String,
    pub operator: CheckOperator,
    pub expected_value: String,
}

impl Expression for CheckExpression {
    fn evaluate(&self, object: &GQLObject) -> bool {
        if object.attributes.contains_key(&self.field_name) {
            let value = object.attributes.get(&self.field_name).unwrap();
            return match self.operator {
                CheckOperator::Contains => value.contains(&self.expected_value),
                CheckOperator::StartsWith => value.starts_with(&self.expected_value),
                CheckOperator::EndsWith => value.ends_with(&self.expected_value),
            };
        }
        return false;
    }
}

#[derive(PartialEq)]
pub enum LogicalOperator {
    Or,
    And,
}

pub struct BinaryExpression {
    pub right: Box<dyn Expression>,
    pub operator: LogicalOperator,
    pub left: Box<dyn Expression>,
}

impl Expression for BinaryExpression {
    fn evaluate(&self, object: &GQLObject) -> bool {
        let lhs = self.left.evaluate(object);

        if self.operator == LogicalOperator::And && !lhs {
            return false;
        }

        if self.operator == LogicalOperator::Or && lhs {
            return true;
        }

        let rhs = self.right.evaluate(object);

        return match self.operator {
            LogicalOperator::And => lhs && rhs,
            LogicalOperator::Or => lhs || rhs,
        };
    }
}
