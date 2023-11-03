use std::cmp::Ordering;
use std::ops::Mul;

use crate::date_utils::time_stamp_to_date;
use crate::date_utils::time_stamp_to_date_time;
use crate::types::DataType;

#[derive(PartialOrd, Clone)]
pub enum Value {
    Integer(i64),
    Float(f64),
    Text(String),
    Boolean(bool),
    DateTime(i64),
    Date(i64),
    Time(String),
    Null,
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        if self.data_type() != other.data_type() {
            return false;
        }

        match self.data_type() {
            DataType::Any => true,
            DataType::Text => self.as_text() == other.as_text(),
            DataType::Integer => self.as_int() == other.as_int(),
            DataType::Float => self.as_float() == other.as_float(),
            DataType::Boolean => self.as_bool() == other.as_bool(),
            DataType::DateTime => self.as_date() == other.as_date(),
            DataType::Date => self.as_date() == other.as_date(),
            DataType::Time => self.as_date() == other.as_date(),
            DataType::Undefined => true,
            DataType::Null => true,
        }
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_type = self.data_type();
        let other_type = other.data_type();

        if self_type.is_type(DataType::Integer) && other_type.is_type(DataType::Integer) {
            return other.as_int().cmp(&self.as_int());
        }

        if self_type.is_type(DataType::Float) && other_type.is_type(DataType::Float) {
            return other.as_float().total_cmp(&self.as_float());
        }

        if self_type.is_type(DataType::Text) && other_type.is_type(DataType::Text) {
            return other.as_text().cmp(&self.as_text());
        }

        if self_type.is_type(DataType::DateTime) && other_type.is_type(DataType::DateTime) {
            return other.as_date_time().cmp(&self.as_date_time());
        }

        if self_type.is_type(DataType::Date) && other_type.is_type(DataType::Date) {
            return other.as_date().cmp(&self.as_date());
        }

        if self_type.is_type(DataType::Time) && other_type.is_type(DataType::Time) {
            return other.as_time().cmp(&self.as_time());
        }

        Ordering::Equal
    }
}

impl Eq for Value {}

impl Value {
    pub fn plus(&self, other: &Value) -> Value {
        let self_type = self.data_type();
        let other_type = other.data_type();

        if self_type == DataType::Integer && other_type == DataType::Integer {
            return Value::Integer(self.as_int() + other.as_int());
        }

        if self_type == DataType::Float && other_type == DataType::Float {
            return Value::Float(self.as_float() + other.as_float());
        }

        if self_type == DataType::Integer && other_type == DataType::Float {
            return Value::Float((self.as_int() as f64) + other.as_float());
        }

        if self_type == DataType::Float && other_type == DataType::Integer {
            return Value::Float(self.as_float() + (other.as_int() as f64));
        }

        Value::Integer(0)
    }

    pub fn minus(&self, other: &Value) -> Value {
        let self_type = self.data_type();
        let other_type = other.data_type();

        if self_type == DataType::Integer && other_type == DataType::Integer {
            return Value::Integer(self.as_int() - other.as_int());
        }

        if self_type == DataType::Float && other_type == DataType::Float {
            return Value::Float(self.as_float() - other.as_float());
        }

        if self_type == DataType::Integer && other_type == DataType::Float {
            return Value::Float((self.as_int() as f64) - other.as_float());
        }

        if self_type == DataType::Float && other_type == DataType::Integer {
            return Value::Float(self.as_float() - (other.as_int() as f64));
        }

        Value::Integer(0)
    }

    pub fn mul(&self, other: &Value) -> Result<Value, String> {
        let self_type = self.data_type();
        let other_type = other.data_type();

        if self_type == DataType::Integer && other_type == DataType::Integer {
            let lhs = self.as_int();
            let rhs = other.as_int();
            let multi_result = lhs.overflowing_mul(rhs);
            if multi_result.1 {
                return Err(format!(
                    "Attempt to compute `{} * {}`, which would overflow",
                    lhs, rhs
                ));
            }
            return Ok(Value::Integer(multi_result.0));
        }

        if self_type == DataType::Float && other_type == DataType::Float {
            return Ok(Value::Float(self.as_float() * other.as_float()));
        }

        if self_type == DataType::Integer && other_type == DataType::Float {
            return Ok(Value::Float(other.as_float().mul(self.as_int() as f64)));
        }

        if self_type == DataType::Float && other_type == DataType::Integer {
            return Ok(Value::Float(self.as_float().mul(other.as_int() as f64)));
        }

        Ok(Value::Integer(0))
    }

    pub fn div(&self, other: &Value) -> Result<Value, String> {
        let self_type = self.data_type();
        let other_type = other.data_type();

        if other_type == DataType::Integer {
            let other = other.as_int();
            if other == 0 {
                return Err(format!("Attempt to divide `{}` by zero", self.literal()));
            }
        }

        if self_type == DataType::Integer && other_type == DataType::Integer {
            return Ok(Value::Integer(self.as_int() / other.as_int()));
        }

        if self_type == DataType::Float && other_type == DataType::Float {
            return Ok(Value::Float(self.as_float() / other.as_float()));
        }

        if self_type == DataType::Integer && other_type == DataType::Float {
            return Ok(Value::Float(self.as_int() as f64 / other.as_float()));
        }

        if self_type == DataType::Float && other_type == DataType::Integer {
            return Ok(Value::Float(self.as_float() / other.as_int() as f64));
        }

        Ok(Value::Integer(0))
    }

    pub fn modulus(&self, other: &Value) -> Result<Value, String> {
        let self_type = self.data_type();
        let other_type = other.data_type();

        if other_type == DataType::Integer {
            let other = other.as_int();
            if other == 0 {
                return Err(format!(
                    "Attempt to calculate the remainder of `{}` with a divisor of zero",
                    self.literal()
                ));
            }
        }

        if self_type == DataType::Integer && other_type == DataType::Integer {
            return Ok(Value::Integer(self.as_int() % other.as_int()));
        }

        if self_type == DataType::Float && other_type == DataType::Float {
            return Ok(Value::Float(self.as_float() % other.as_float()));
        }

        if self_type == DataType::Integer && other_type == DataType::Float {
            return Ok(Value::Float(self.as_int() as f64 % other.as_float()));
        }

        if self_type == DataType::Float && other_type == DataType::Integer {
            return Ok(Value::Float(self.as_float() % other.as_int() as f64));
        }

        Ok(Value::Integer(0))
    }

    pub fn data_type(&self) -> DataType {
        match self {
            Value::Integer(_) => DataType::Integer,
            Value::Float(_) => DataType::Float,
            Value::Text(_) => DataType::Text,
            Value::Boolean(_) => DataType::Boolean,
            Value::DateTime(_) => DataType::DateTime,
            Value::Date(_) => DataType::Date,
            Value::Time(_) => DataType::Time,
            Value::Null => DataType::Null,
        }
    }

    pub fn literal(&self) -> String {
        match self {
            Value::Integer(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Text(s) => s.to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::DateTime(dt) => time_stamp_to_date_time(*dt),
            Value::Date(d) => time_stamp_to_date(*d),
            Value::Time(t) => t.to_string(),
            Value::Null => "Null".to_string(),
        }
    }

    pub fn as_int(&self) -> i64 {
        if let Value::Integer(n) = self {
            return *n;
        }
        0
    }

    pub fn as_float(&self) -> f64 {
        if let Value::Float(n) = self {
            return *n;
        }
        0f64
    }

    pub fn as_text(&self) -> String {
        if let Value::Text(s) = self {
            return s.to_string();
        }
        "".to_owned()
    }

    pub fn as_bool(&self) -> bool {
        if let Value::Boolean(b) = self {
            return *b;
        }
        false
    }

    pub fn as_date_time(&self) -> i64 {
        if let Value::DateTime(d) = self {
            return *d;
        }
        0
    }

    pub fn as_date(&self) -> i64 {
        if let Value::Date(d) = self {
            return *d;
        }
        0
    }

    pub fn as_time(&self) -> String {
        if let Value::Time(d) = self {
            return d.to_string();
        }
        "".to_owned()
    }
}
