use crate::object::GQLObject;

pub trait Expression {
    fn evaluate(&self, object: &GQLObject) -> bool;
}

#[derive(PartialEq)]
pub enum Operator {
    Or,
    And,
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

pub struct BinaryExpression {
    pub right: Box<dyn Expression>,
    pub operator: Operator,
    pub left: Box<dyn Expression>,
}

impl Expression for BinaryExpression {
    fn evaluate(&self, object: &GQLObject) -> bool {
        let rhs = self.right.evaluate(object);
        let lhs = self.left.evaluate(object);

        if self.operator == Operator::And {
            return rhs && lhs;
        }

        if self.operator == Operator::Or {
            return rhs || lhs;
        }

        return false;
    }
}
