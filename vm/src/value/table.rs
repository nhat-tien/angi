use crate::{error::VmError, tree::Tree};
use super::{generate_error_message_when_mismatch_casting, FromValue, Value};


pub struct Table {
    child: Box<Tree<Value>>
}

impl FromValue for Table {
    fn from_value(v: Value) -> Result<Self, VmError> {
        match v {
            Value::Table(table) => Ok(Table { child: table }),
            v => Err(VmError::ValueTypeMismatch {
                message: generate_error_message_when_mismatch_casting(v, "String".into()),
            }),
        }
    }
}

