mod list;
mod table;
mod function;

pub use list::List;
pub use table::Table;
pub use function::Arg;

use crate::error::VmError;
use std::fmt;

use super::tree::Tree;

#[derive(Debug)]
pub enum Value {
    Int(i64),
    String(String),
    Table(Box<Tree<Value>>),
    List(Vec<Value>),
    Thunk(u32),
    Function(u32),
    None,
}

impl Clone for Value {
    fn clone(&self) -> Self {
        match self {
            Self::Int(arg0) => Self::Int(*arg0),
            Self::String(arg0) => Self::String(arg0.clone()),
            Self::Table(arg0) => Self::Table(arg0.clone()),
            Self::Thunk(arg0) => Self::Thunk(*arg0),
            Self::Function(arg0) => Self::Thunk(*arg0),
            Self::List(arg0) => Self::List(arg0.clone()),
            Self::None => Self::None,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Int(_) => write!(f, "Int"),
            Value::String(_) => write!(f, "String"),
            Value::Table(_) => write!(f, "Table"),
            Value::List(_) => write!(f, "List"),
            Value::Thunk(_) => write!(f, "Thunk"),
            Value::Function(_) => write!(f, "Function"),
            Value::None => write!(f, "None"),
        }
    }
}

impl Value {
    pub fn to_string(&self) -> Result<String, VmError> {
        match self {
            Value::String(str) => Ok(str.clone()),
            _ => Err(VmError::ValueTypeMismatch {
                message: "value not string".into(),
            }),
        }
    }

    pub fn val<T>(self) -> Result<T, VmError> where T: FromValue {
        T::from_value(self)
    }
}



pub trait FromValue: Sized + Clone {
    fn from_value(v: Value) -> Result<Self, VmError>;
}

impl FromValue for i64 {
    fn from_value(v: Value) -> Result<Self, VmError> {
        match v {
            Value::Int(int) => Ok(int),
            v => Err(VmError::ValueTypeMismatch {
                message: generate_error_message_when_mismatch_casting(v, "i64".into()),
            }),
        }
    }
}

impl FromValue for String {
    fn from_value(v: Value) -> Result<Self, VmError> {
        match v {
            Value::String(str) => Ok(str),
            v => Err(VmError::ValueTypeMismatch {
                message: generate_error_message_when_mismatch_casting(v, "String".into()),
            }),
        }
    }
}

fn generate_error_message_when_mismatch_casting(src: Value, dest: String) -> String {
    format!("Cannot casting from {src} to {dest}")
}


#[cfg(test)]
mod test {
    use crate::{error::VmError, value::Value};

    #[test]
    fn test_clone_and_display() {
        let v1 = Value::Int(42);
        let v2 = v1.clone();
        assert!(matches!(v2, Value::Int(_)));
        assert_eq!(format!("{}", v1), "Int");

        let v3 = Value::String("abc".into());
        assert_eq!(format!("{}", v3), "String");

        let v4 = Value::None;
        assert_eq!(format!("{}", v4), "None");
    }

    #[test]
    fn test_to_string_success() {
        let v = Value::String("hello".into());
        assert_eq!(v.to_string().unwrap(), "hello");
    }

    #[test]
    fn test_to_string_fail() {
        let v = Value::Int(10);
        let err = v.to_string().unwrap_err();
        match err {
            VmError::ValueTypeMismatch { message } => {
                assert!(message.contains("value not string"));
            }
            _ => panic!("Unexpected error variant"),
        }
    }

    #[test]
    fn test_from_value_i64_success() {
        let v = Value::Int(123);
        let n: i64 = v.val().unwrap();
        assert_eq!(n, 123);
    }

    #[test]
    fn test_from_value_i64_fail() {
        let v = Value::String("oops".into());
        let result: Result<i64, VmError> = v.val();
        assert!(matches!(result, Err(VmError::ValueTypeMismatch { .. })));
    }

    #[test]
    fn test_from_value_string_success() {
        let v = Value::String("world".into());
        let s: String = v.val().unwrap();
        assert_eq!(s, "world");
    }

    #[test]
    fn test_generate_error_message() {
        let msg = super::generate_error_message_when_mismatch_casting(
            Value::Int(10),
            "String".into(),
        );
        assert_eq!(msg, "Cannot casting from Int to String");
    }
}


