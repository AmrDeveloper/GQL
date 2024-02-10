use std::fmt;

/// Represent the data types for values to be used in type checker
#[derive(Clone)]
pub enum DataType {
    /// Represent general type so can be equal to any other type
    Any,
    /// Represent String Type
    Text,
    /// Represent Integer 64 bit type
    Integer,
    /// Represent Float 64 bit type
    Float,
    /// Represent Boolean (true | false) type
    Boolean,
    /// Represent Date type
    Date,
    /// Represent Time type
    Time,
    /// Represent Date with Time type
    DateTime,
    /// Represent `Undefined` value
    Undefined,
    /// Represent `NULL` value
    Null,
    /// Represent a set of valid variant of types
    Variant(Vec<DataType>),
    /// Represent an optional type so it can passed or not, must be last parameter
    Optional(Box<DataType>),
    /// Represent variable arguments so can pass 0 or more value with spastic type, must be last parameter
    Varargs(Box<DataType>),
}

impl PartialEq for DataType {
    fn eq(&self, other: &Self) -> bool {
        if self.is_any() || other.is_any() {
            return true;
        }

        if let DataType::Variant(types) = self {
            for data_type in types {
                if data_type == other {
                    return true;
                }
            }
            return false;
        }

        if let DataType::Variant(types) = other {
            for data_type in types {
                if data_type == self {
                    return true;
                }
            }
            return false;
        }

        if let DataType::Optional(optional_type) = self {
            return optional_type.as_ref() == other;
        }

        if let DataType::Optional(optional_type) = other {
            return optional_type.as_ref() == self;
        }

        if let DataType::Varargs(data_type) = self {
            return data_type.as_ref() == other;
        }

        if let DataType::Varargs(data_type) = other {
            return data_type.as_ref() == self;
        }

        if self.is_bool() && other.is_bool() {
            return true;
        }

        if self.is_int() && other.is_int() {
            return true;
        }

        if self.is_float() && other.is_float() {
            return true;
        }

        if self.is_text() && other.is_text() {
            return true;
        }

        if self.is_date() && other.is_date() {
            return true;
        }

        if self.is_time() && other.is_time() {
            return true;
        }

        if self.is_datetime() && other.is_datetime() {
            return true;
        }

        if self.is_null() && other.is_null() {
            return true;
        }

        if self.is_undefined() && other.is_undefined() {
            return true;
        }

        false
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DataType::Any => write!(f, "Any"),
            DataType::Text => write!(f, "Text"),
            DataType::Integer => write!(f, "Integer"),
            DataType::Float => write!(f, "Float"),
            DataType::Boolean => write!(f, "Boolean"),
            DataType::Date => write!(f, "Date"),
            DataType::Time => write!(f, "Time"),
            DataType::DateTime => write!(f, "DateTime"),
            DataType::Undefined => write!(f, "Undefined"),
            DataType::Null => write!(f, "Null"),
            DataType::Variant(types) => {
                write!(f, "[")?;
                for (pos, data_type) in types.iter().enumerate() {
                    write!(f, "{}", data_type)?;
                    if pos != types.len() - 1 {
                        write!(f, " | ")?;
                    }
                }
                write!(f, "]")
            }
            DataType::Optional(data_type) => {
                write!(f, "{}?", data_type)
            }
            DataType::Varargs(data_type) => {
                write!(f, "...{}", data_type)
            }
        }
    }
}

impl DataType {
    pub fn is_any(&self) -> bool {
        matches!(self, DataType::Any)
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, DataType::Boolean)
    }

    pub fn is_int(&self) -> bool {
        matches!(self, DataType::Integer)
    }

    pub fn is_float(&self) -> bool {
        matches!(self, DataType::Float)
    }

    pub fn is_number(&self) -> bool {
        self.is_int() || self.is_float()
    }

    pub fn is_text(&self) -> bool {
        matches!(self, DataType::Text)
    }

    pub fn is_time(&self) -> bool {
        matches!(self, DataType::Time)
    }

    pub fn is_date(&self) -> bool {
        matches!(self, DataType::Date)
    }

    pub fn is_datetime(&self) -> bool {
        matches!(self, DataType::DateTime)
    }

    pub fn is_null(&self) -> bool {
        matches!(self, DataType::Null)
    }

    pub fn is_undefined(&self) -> bool {
        matches!(self, DataType::Undefined)
    }

    pub fn is_variant(&self) -> bool {
        matches!(self, DataType::Variant(_))
    }

    pub fn is_optional(&self) -> bool {
        matches!(self, DataType::Optional(_))
    }

    pub fn is_varargs(&self) -> bool {
        matches!(self, DataType::Varargs(_))
    }
}
