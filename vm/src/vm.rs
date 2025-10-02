use std::collections::HashMap;

use crate::constant::Constant;
use crate::error::RuntimeError;
use crate::metadata::MetaData;
use crate::register::Register;
use crate::utils::read_u32;

pub struct VM {
    metadata: MetaData,
    registers: Register,
    const_pool: HashMap<usize, Constant>,
    bytes: Vec<u8>,
    cursor: usize,
}

impl VM {
    pub fn new() -> Self {
        VM {
            registers: Register::new(),
            const_pool: HashMap::new(),
            bytes: vec![],
            cursor: 0,
            metadata: MetaData::default(),
        }
    }

    pub fn load(&mut self, bytes: Vec<u8>) -> Result<(), RuntimeError> {
        self.bytes = bytes;
        self.load_metadata()?;
        Ok(())
    }

    pub fn eval(&self, attribute: String) {
        let attr_arr = attribute.split('.');
    }

    pub fn load_metadata(&mut self) -> Result<(), RuntimeError> {
        let magic_code = read_u32(&self.bytes, &mut self.cursor).ok_or_else(|| RuntimeError {
            message: "Error in get magic code".into(),
        })?;
        let version = read_u32(&self.bytes, &mut self.cursor).ok_or_else(|| RuntimeError {
            message: "Error in get version".into(),
        })?;
        let const_offset = read_u32(&self.bytes, &mut self.cursor).ok_or_else(|| RuntimeError {
            message: "Error in get const_offset".into(),
        })?;
        let const_size = read_u32(&self.bytes, &mut self.cursor).ok_or_else(|| RuntimeError {
            message: "Error in get const_size".into(),
        })?;
        let thunk_offset = read_u32(&self.bytes, &mut self.cursor).ok_or_else(|| RuntimeError {
            message: "Error in get thunk_offset".into(),
        })?;
        let thunk_size = read_u32(&self.bytes, &mut self.cursor).ok_or_else(|| RuntimeError {
            message: "Error in get thunk_size".into(),
        })?;
        let code_offset = read_u32(&self.bytes, &mut self.cursor).ok_or_else(|| RuntimeError {
            message: "Error in get code_offset".into(),
        })?;
        let code_size = read_u32(&self.bytes, &mut self.cursor).ok_or_else(|| RuntimeError {
            message: "Error in code_size".into(),
        })?;

        self.metadata = MetaData {
            magic_code,
            version,
            const_offset,
            const_size,
            thunk_offset,
            thunk_size,
            code_offset,
            code_size,
        };
        Ok(())
    }

    pub fn get_metadata(&self) -> MetaData {
        self.metadata
    }

    pub fn get_const() {
    }
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}
