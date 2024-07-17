use chrono::DateTime;
use std::cmp::Ordering;
use std::fmt;
use std::ops::Mul;

use crate::types::DataType;

const VALUE_DATE_FORMAT: &str = "%Y-%m-%d";
const VALUE_DATE_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S%.3f";

#[derive(Clone)]
pub enum Value {
    Integer(i64),
    Float(f64),
    Text(String),
    Boolean(bool),
    DateTime(i64),
    Date(i64),
    Time(String),
    Array(DataType, Vec<Value>),
    Null,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Integer(i64) => write!(f, "{}", i64),
            Value::Float(f64) => write!(f, "{}", f64),
            Value::Text(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Array(_data_type, elements) => {
                let last_position = elements.len() - 1;
                write!(f, "[")?;
                for (pos, element) in elements.iter().enumerate() {
                    write!(f, "{}", element)?;
                    if pos != last_position {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
            Value::DateTime(dt) => {
                let datetime = DateTime::from_timestamp(*dt, 0).unwrap();
                write!(f, "{}", datetime.format(VALUE_DATE_TIME_FORMAT))
            }
            Value::Date(d) => {
                let date_time = DateTime::from_timestamp(*d, 0).unwrap();
                write!(f, "{}", date_time.format(VALUE_DATE_FORMAT))
            }
            Value::Time(t) => write!(f, "{}", t),
            Value::Null => write!(f, "Null"),
        }
    }
}

impl Value {
    pub fn equals(&self, other: &Self) -> bool {
        if self.data_type() != other.data_type() {
            return false;
        }

        match self.data_type() {
            DataType::Any => true,
            DataType::Text => self.as_text() == other.as_text(),
            DataType::Integer => self.as_int() == other.as_int(),
            DataType::Float => self.as_float() == other.as_float(),
            DataType::Boolean => self.as_bool() == other.as_bool(),
            DataType::DateTime => self.as_date_time() == other.as_date_time(),
            DataType::Date => self.as_date() == other.as_date(),
            DataType::Time => self.as_time() == other.as_time(),
            DataType::Undefined => true,
            DataType::Null => true,
            _ => false,
        }
    }

    pub fn compare(&self, other: &Self) -> Ordering {
        let self_type = self.data_type();
        let other_type = other.data_type();

        if self_type.is_int() && other_type.is_int() {
            return other.as_int().cmp(&self.as_int());
        }

        if self_type.is_float() && other_type.is_float() {
            return other.as_float().total_cmp(&self.as_float());
        }

        if self_type.is_text() && other_type.is_text() {
            return other.as_text().cmp(&self.as_text());
        }

        if self_type.is_datetime() && other_type.is_datetime() {
            return other.as_date_time().cmp(&self.as_date_time());
        }

        if self_type.is_date() && other_type.is_date() {
            return other.as_date().cmp(&self.as_date());
        }

        if self_type.is_time() && other_type.is_time() {
            return other.as_time().cmp(&self.as_time());
        }

        Ordering::Equal
    }

    pub fn plus(&self, other: &Value) -> Result<Value, String> {
        let self_type = self.data_type();
        let other_type = other.data_type();

        if self_type.is_int() && other_type.is_int() {
            let lhs = self.as_int();
            let rhs = other.as_int();

            if let Some(sub) = lhs.checked_add(rhs) {
                return Ok(Value::Integer(sub));
            }

            return Err(format!(
                "Attempt to compute `{} + {}`, which would overflow",
                lhs, rhs
            ));
        }

        if self_type.is_float() && other_type.is_float() {
            return Ok(Value::Float(self.as_float() + other.as_float()));
        }

        if self_type.is_int() && other_type.is_float() {
            return Ok(Value::Float((self.as_int() as f64) + other.as_float()));
        }

        if self_type.is_float() && other_type.is_int() {
            return Ok(Value::Float(self.as_float() + (other.as_int() as f64)));
        }

        Ok(Value::Integer(0))
    }

    pub fn minus(&self, other: &Value) -> Result<Value, String> {
        let self_type = self.data_type();
        let other_type = other.data_type();

        if self_type.is_int() && other_type.is_int() {
            let lhs = self.as_int();
            let rhs = other.as_int();

            if let Some(sub) = lhs.checked_sub(rhs) {
                return Ok(Value::Integer(sub));
            }

            return Err(format!(
                "Attempt to compute `{} - {}`, which would overflow",
                lhs, rhs
            ));
        }

        if self_type.is_float() && other_type.is_float() {
            return Ok(Value::Float(self.as_float() - other.as_float()));
        }

        if self_type.is_int() && other_type.is_float() {
            return Ok(Value::Float((self.as_int() as f64) - other.as_float()));
        }

        if self_type.is_float() && other_type.is_int() {
            return Ok(Value::Float(self.as_float() - (other.as_int() as f64)));
        }

        Ok(Value::Integer(0))
    }

    pub fn mul(&self, other: &Value) -> Result<Value, String> {
        let self_type = self.data_type();
        let other_type = other.data_type();

        if self_type.is_int() && other_type.is_int() {
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

        if self_type.is_float() && other_type.is_float() {
            return Ok(Value::Float(self.as_float() * other.as_float()));
        }

        if self_type.is_int() && other_type.is_float() {
            return Ok(Value::Float(other.as_float().mul(self.as_int() as f64)));
        }

        if self_type.is_float() && other_type.is_int() {
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
                return Err(format!("Attempt to divide `{}` by zero", self));
            }
        }

        if self_type.is_int() && other_type.is_int() {
            return Ok(Value::Integer(self.as_int() / other.as_int()));
        }

        if self_type.is_float() && other_type.is_float() {
            return Ok(Value::Float(self.as_float() / other.as_float()));
        }

        if self_type.is_int() && other_type.is_float() {
            return Ok(Value::Float(self.as_int() as f64 / other.as_float()));
        }

        if self_type.is_float() && other_type.is_int() {
            return Ok(Value::Float(self.as_float() / other.as_int() as f64));
        }

        Ok(Value::Null)
    }

    pub fn modulus(&self, other: &Value) -> Result<Value, String> {
        let self_type = self.data_type();
        let other_type = other.data_type();

        if other_type.is_int() {
            let other = other.as_int();
            if other == 0 {
                return Err(format!(
                    "Attempt to calculate the remainder of `{}` with a divisor of zero",
                    self
                ));
            }
        }

        if self_type.is_int() && other_type.is_int() {
            return Ok(Value::Integer(self.as_int() % other.as_int()));
        }

        if self_type.is_float() && other_type.is_float() {
            return Ok(Value::Float(self.as_float() % other.as_float()));
        }

        if self_type.is_int() && other_type.is_float() {
            return Ok(Value::Float(self.as_int() as f64 % other.as_float()));
        }

        if self_type.is_float() && other_type.is_int() {
            return Ok(Value::Float(self.as_float() % other.as_int() as f64));
        }

        Ok(Value::Integer(0))
    }

    pub fn pow(&self, other: &Value) -> Result<Value, String> {
        let self_type = self.data_type();
        let other_type = other.data_type();

        if self_type.is_int() && other_type.is_int() {
            let other = other.as_int();
            if other < 0 {
                return Err(format!("Attempt to perform `{} ^ {}`", self, other));
            }
            return Ok(Value::Integer(i64::pow(self.as_int(), other as u32)));
        }

        if self_type.is_float() && other_type.is_float() {
            return Ok(Value::Float(f64::powf(self.as_float(), other.as_float())));
        }

        if self_type.is_float() && other_type.is_int() {
            let other = other.as_int();
            if other < 0 {
                return Err(format!("Attempt to perform `{} ^ {}`", self, other));
            }
            return Ok(Value::Float(f64::powi(self.as_float(), other as i32)));
        }

        if self_type.is_int() && other_type.is_float() {
            let expr: u32 = other.as_float() as u32;
            return Ok(Value::Integer(i64::pow(self.as_int(), expr)));
        }

        Ok(Value::Null)
    }

    pub fn data_type(&self) -> DataType {
        match self {
            Value::Integer(_) => DataType::Integer,
            Value::Float(_) => DataType::Float,
            Value::Text(_) => DataType::Text,
            Value::Boolean(_) => DataType::Boolean,
            Value::Array(data_type, _) => DataType::Array(Box::new(data_type.clone())),
            Value::DateTime(_) => DataType::DateTime,
            Value::Date(_) => DataType::Date,
            Value::Time(_) => DataType::Time,
            Value::Null => DataType::Null,
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

    pub fn as_array(&self) -> Vec<Value> {
        if let Value::Array(_, elements) = self {
            return elements.to_vec();
        }
        vec![]
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
