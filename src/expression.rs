use crate::object::GQLObject;

pub trait Expression {
    fn evaluate(&self, object: &GQLObject) -> bool;
}

pub struct EqualExpression {
    pub field_name: String,
    pub expected_value: String,
}

impl Expression for EqualExpression {
    fn evaluate(&self, object: &GQLObject) -> bool {
        if object.attributes.contains_key(&self.field_name) {
            let attribute_value = object.attributes.get(&self.field_name).unwrap();
            return attribute_value.to_string() == self.expected_value;
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
        let rhs = self.right.evaluate(object);
        let lhs = self.left.evaluate(object);

        return match self.operator {
            LogicalOperator::And => lhs && rhs,
            LogicalOperator::Or => lhs || rhs,
        };
    }
}
