use crate::{error::VmError, tree::Tree};
use super::{generate_error_message_when_mismatch_casting, FromValue, Value};

#[derive(Clone, Debug)]
pub struct Table {
    child: Box<Tree<Value>>
}

impl Table {

    pub fn get<T>(&self, key: &str) -> Option<T> where T : FromValue {
        let result = self.child.get(vec![key]);
        result.map(|v| T::from_value(v).ok())?
    }
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

