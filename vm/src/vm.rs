use std::collections::HashMap;
use instructions::{extract_opcode, OpCode};
use crate::constant::ConstantValue;
use crate::error::RuntimeError;
use crate::metadata::MetaData;
use crate::register::Register;
use crate::utils::{read_i64, read_str_with_len, read_u32, read_u8};
use crate::value::Value;


pub struct VM {
    metadata: MetaData,
    registers: Register,
    const_pool: HashMap<usize, ConstantValue>,
    bytes: Vec<u8>,
    cursor: usize,
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
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
        self.load_const()?;
        Ok(())
    }

    pub fn eval(&mut self) -> Result<Value, RuntimeError> {
        let code_offset = self.metadata.code_offset;

        self.cursor = code_offset as usize;

        let value = self.handle_instruction()?;

        Ok(value)
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

    pub fn load_const(&mut self) -> Result<(), RuntimeError> {
        let const_size = self.metadata.const_size;
        let const_offset = self.metadata.const_offset;

        self.cursor = const_offset as usize;

        for i in 1..(const_size + 1) {

            let const_type = read_u8(&self.bytes, &mut self.cursor).ok_or_else(|| RuntimeError {
                message: "Error in get const_size".into(),
            })?;

            match const_type {
                0 => {
                    let number = read_i64(&self.bytes, &mut self.cursor).ok_or_else(|| RuntimeError {
                        message: "Error in get number value const".into(),
                    })?;

                    self.const_pool.insert(i as usize, ConstantValue::Int(number));
                },
                1 => {
                    let str_len = read_u32(&self.bytes, &mut self.cursor).ok_or_else(|| RuntimeError {
                        message: "Error in get number value const".into(),
                    })?;

                    let string = read_str_with_len(&self.bytes, &mut self.cursor, str_len as usize).ok_or_else(|| RuntimeError {
                        message: "Error in get string value const".into(),
                    })?;

                    self.const_pool.insert(i as usize, ConstantValue::String(string));
                },
                _ => return Err(RuntimeError { message: format!("Unexpect const type {}, {}", const_type, i) })
            }
        }

        Ok(())
    }

    pub fn get_const(&self, idx: usize) -> Option<&ConstantValue> {
        self.const_pool.get(&idx)
    }

    pub fn handle_instruction(&mut self) -> Result<Value, RuntimeError> {
        loop {
            let ins = read_u32(&self.bytes, &mut self.cursor).ok_or_else(|| RuntimeError {
                message: "Error in get ins value".into(),
            })?;

            let opcode = extract_opcode(ins).ok_or_else(|| RuntimeError {
                message: "Error in get opcode".into(),
            })?;

            match opcode {
                OpCode::MTB => {
                    let params = OpCode::MTB.decode(ins);
                    self.registers.set_new_table(params[0] as usize);
                },
                OpCode::LDC => {
                    let params = OpCode::LDC.decode(ins);
                    let constant = self.const_pool.get(&(params[1] as usize)).ok_or_else(|| RuntimeError {
                        message: "Error in get constant value".into()
                    })?;
                    match constant {
                        ConstantValue::Int(int) => {
                            self.registers.set(params[0] as usize, Value::Int(*int));
                        },
                        ConstantValue::String(str) => {
                            self.registers.set(params[0] as usize, Value::String(str.to_string()));
                        }
                    }
                },
                OpCode::SAT => {
                    let params = OpCode::SAT.decode(ins);
                    let table_reg = params[0];
                    let key = self.registers.get(params[1] as usize).ok_or_else(|| RuntimeError {
                        message: "Error in get key in SAT".into()
                    })?;
                    let value = self.registers.get(params[2] as usize).ok_or_else(|| RuntimeError {
                        message: "Error in get value in SAT".into()
                    })?;
                    self.registers.set_attr_table(table_reg as usize, key.to_string()?, value);
                }
                OpCode::MTK => {
                    let params = OpCode::MTK.decode(ins);
                    let reg = params[0];
                    let thunk_idx = params[1];
                    self.registers.set(reg as usize, Value::Thunk(thunk_idx));
                },
                OpCode::RET => {
                    let params = OpCode::RET.decode(ins);
                    return self.registers.get(params[0] as usize).ok_or_else(|| RuntimeError {
                        message: "Error in get registers value".into()
                    });
                }
                _ => return Err(RuntimeError { message: "Unexpect opcode".into() })
            }
        }
    }
}

