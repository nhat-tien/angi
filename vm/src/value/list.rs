use crate::error::VmError;

use super::{generate_error_message_when_mismatch_casting, FromValue, Value};


pub struct List {
    child: Vec<Value>
}

impl List {

    pub fn len(&self) -> usize {
        self.child.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get(&self, idx: usize) -> Option<&Value>{
        self.child.get(idx)
    }
}

impl<'a> IntoIterator for &'a List {
    type Item = &'a Value;
    type IntoIter = std::slice::Iter<'a, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.child.iter()
    }
}

impl<'a> IntoIterator for &'a mut List {
    type Item = &'a mut Value;
    type IntoIter = std::slice::IterMut<'a, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.child.iter_mut()
    }
}

impl IntoIterator for List {
    type Item = Value;
    type IntoIter = std::vec::IntoIter<Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.child.into_iter()
    }
}


impl FromValue for List {
    fn from_value(v: Value) -> Result<Self, VmError> {
        match v {
            Value::List(list) => Ok(List { child: list }),
            v => Err(VmError::ValueTypeMismatch {
                message: generate_error_message_when_mismatch_casting(v, "String".into()),
            }),
        }
    }
}
