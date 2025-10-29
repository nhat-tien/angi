use crate::error::RuntimeError;

use super::tree::Tree;



#[derive(Debug)]
pub enum Value {
    Int(i64),
    String(String),
    Table(Box<Tree<Value>>),
    List(Vec<Value>),
    Thunk(u32),
    None
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

impl Value {
    pub fn to_string(&self) -> Result<String, RuntimeError>{
        match self {
            Value::String(str) => Ok(str.clone()),
            _ => Err(RuntimeError { message: "constant not string".into() })
        }
    }
}


pub trait FromValue: Sized {
    fn from_value(v: Value) -> Option<Self>;
}

impl FromValue for i64 {
    fn from_value(v: Value) -> Option<Self> {
        match v {
            Value::Int(int) => Some(int),
            _ => None
        }
    }
}

impl FromValue for String {
    fn from_value(v: Value) -> Option<Self> {
        match v {
            Value::Int(int) => Some(int),
            _ => None
        }
    }
}
