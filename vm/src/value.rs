mod list;
mod table;

pub use list::List;
pub use table::Table;

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
    None,
}

impl Clone for Value {
    fn clone(&self) -> Self {
        match self {
            Self::Int(arg0) => Self::Int(*arg0),
            Self::String(arg0) => Self::String(arg0.clone()),
            Self::Table(arg0) => Self::Table(arg0.clone()),
            Self::Thunk(arg0) => Self::Thunk(*arg0),
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
            Value::None => write!(f, "None"),
        }
    }
}

impl Value {
    pub fn to_string(&self) -> Result<String, VmError> {
        match self {
            Value::String(str) => Ok(str.clone()),
            _ => Err(VmError::ValueTypeMismatch {
                message: "constant not string".into(),
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



