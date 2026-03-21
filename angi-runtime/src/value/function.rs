use crate::{error::VmError, vm::VM};

use super::{generate_error_message_when_mismatch_casting, FromValue, ToArgValue, Value};

pub struct Arg {
    pub name: String,
    pub value: Value
}

#[derive(Clone, Debug)]
pub struct Function {
    pub idx: u32
}

impl Function {
    pub fn call<R, A>(&self, vm: &mut VM, args: A) -> Result<R, VmError> 
    where 
        A : ToArgValue,
        R : FromValue 
    {
        let value = vm.eval_function(self.idx, args)?;
        R::from_value(value)
    }
}


impl FromValue for Function where {

    fn from_value(v: Value) -> Result<Self, VmError> {
        match v {
            Value::Function(u32) => Ok(Function { idx : u32 }),
            v => Err(VmError::ValueTypeMismatch {
                message: generate_error_message_when_mismatch_casting(v, "Function".into()),
            }),
        }
    }

}
