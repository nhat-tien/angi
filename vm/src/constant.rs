use crate::{error::VmError, value::Value};

#[derive(Debug)]
pub enum ConstantValue {
    Int(i64),
    String(String),
}

impl std::fmt::Display for ConstantValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConstantValue::Int(int) => int.fmt(f),
            ConstantValue::String(str) => str.fmt(f)
        }
    }
}

impl ConstantValue {
   pub fn to_value(&self) -> Value {
        match self {
            ConstantValue::Int(int) => Value::Int(*int),
            ConstantValue::String(str) => Value::String(str.clone())
        }
    }

    pub fn to_string(&self) -> Result<String, VmError>{
        match self {
            ConstantValue::String(str) => Ok(str.clone()),
            _ => Err(VmError::UnexpectedError { message: "constant not string".into() })
        }
    }
}

