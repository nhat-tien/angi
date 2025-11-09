use crate::{error::VmError, vm::VM};

use super::{generate_error_message_when_mismatch_casting, FromValue, Value};

#[derive(Clone)]
pub struct List<T: FromValue> {
    raw_child: Option<Vec<T>>,
    value_child: Vec<Value>
}

impl<T> List<T> where T : FromValue {

    pub fn new(value: Vec<Value>) -> Self {
        List { raw_child: None, value_child: value }
    }

    pub fn len(&self) -> usize {
        match &self.raw_child {
            Some(list) => list.len(),
            None => 0
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, idx: usize) -> Option<&T>{
        if let Some(list) = &self.raw_child {
            Some(list.get(idx)?)
        } else {
            None
        }
    }

    pub fn iter(&self) -> Option<impl Iterator<Item = T>> {
        self.raw_child.as_ref().map(|list| list.clone().into_iter())
    }

    pub fn force(&mut self, vm: &mut VM) {
       self.raw_child = Some(
            self.value_child
                .iter()
                .map(|v| vm.force::<T>(v.clone()).unwrap())
                .collect::<Vec<_>>()
        );
    }
}

impl<'a, T> IntoIterator for &'a List<T> where T : FromValue {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        match &self.raw_child {
            Some(list) => list.iter(),
            None => [].iter()
        }
    }
}

impl<'a, T> IntoIterator for &'a mut List<T> where T : FromValue {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        match &mut self.raw_child {
            Some(list) => list.iter_mut(),
            None => [].iter_mut()
        }
    }
}

impl<T> IntoIterator for List<T> where T : FromValue {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        match self.raw_child {
            Some(list) => list.into_iter(),
            None => vec![].into_iter()
        }
    }
}


impl<T> FromValue for List<T> where T : FromValue{
    fn from_value(v: Value) -> Result<Self, VmError> {
        match v {
            Value::List(list) => Ok(List::new(list)),
            v => Err(VmError::ValueTypeMismatch {
                message: generate_error_message_when_mismatch_casting(v, "List".into()),
            }),
        }
    }

}
