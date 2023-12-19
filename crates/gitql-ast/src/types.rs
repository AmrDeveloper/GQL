use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt;

lazy_static! {
    pub static ref TABLES_FIELDS_TYPES: HashMap<&'static str, DataType> = {
        let mut map = HashMap::new();
        map.insert("commit_id", DataType::Text);
        map.insert("title", DataType::Text);
        map.insert("message", DataType::Text);
        map.insert("name", DataType::Text);
        map.insert("full_name", DataType::Text);
        map.insert("insertions", DataType::Integer);
        map.insert("deletions", DataType::Integer);
        map.insert("files_changed", DataType::Integer);
        map.insert("email", DataType::Text);
        map.insert("type", DataType::Text);
        map.insert("datetime", DataType::DateTime);
        map.insert("is_head", DataType::Boolean);
        map.insert("is_remote", DataType::Boolean);
        map.insert("commit_count", DataType::Integer);
        map.insert("repo", DataType::Text);
        map
    };
}

#[derive(Debug, Clone)]
pub enum DataType {
    Any,
    Text,
    Integer,
    Float,
    Boolean,
    Date,
    Time,
    DateTime,
    Undefined,
    Null,
    Variant(Vec<DataType>),
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

        if self.is_bool() && other.is_bool() {
            return true;
        }

        if self.is_int() && other.is_int() {
            return true;
        }

        if self.is_float() && other.is_float() {
            return true;
        }

        if self.is_number() && other.is_number() {
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
        }
    }
}

impl DataType {
    pub fn is_any(&self) -> bool {
        if let DataType::Any = self {
            return true;
        }
        false
    }

    pub fn is_bool(&self) -> bool {
        if let DataType::Boolean = self {
            return true;
        }
        false
    }

    pub fn is_int(&self) -> bool {
        if let DataType::Integer = self {
            return true;
        }
        false
    }

    pub fn is_float(&self) -> bool {
        if let DataType::Float = self {
            return true;
        }
        false
    }

    pub fn is_number(&self) -> bool {
        self.is_int() || self.is_float()
    }

    pub fn is_text(&self) -> bool {
        if let DataType::Text = self {
            return true;
        }
        false
    }

    pub fn is_time(&self) -> bool {
        if let DataType::Time = self {
            return true;
        }
        false
    }

    pub fn is_date(&self) -> bool {
        if let DataType::Date = self {
            return true;
        }
        false
    }

    pub fn is_datetime(&self) -> bool {
        if let DataType::DateTime = self {
            return true;
        }
        false
    }

    pub fn is_null(&self) -> bool {
        if let DataType::Null = self {
            return true;
        }
        false
    }

    pub fn is_undefined(&self) -> bool {
        if let DataType::Undefined = self {
            return true;
        }
        false
    }

    pub fn is_variant(&self) -> bool {
        matches!(self, DataType::Variant(_))
    }
}
