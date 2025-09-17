use crate::register::Register;

pub struct VM {
    registers: Register,
}

impl VM {
    pub fn new() -> Self {
        VM { registers: Register::new() }
    }

    pub fn load() {
        todo!()
    }

    pub fn eval() {
        todo!()
    }
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}
