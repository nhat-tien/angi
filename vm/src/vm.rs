use crate::constant::ConstantValue;
use crate::error::RuntimeError;
use crate::metadata::MetaData;
use crate::register::Register;
use crate::utils::{read_i64, read_n_bytes_from_end_of_file, read_str_with_len, read_u32, read_u32_from_end_of_file, read_u8};
use crate::value::Value;
use instructions::{extract_opcode, OpCode, MAGIC_NUMBER};
use std::collections::HashMap;
use std::fs::{self, File};

pub struct VM {
    metadata: MetaData,
    registers: Register,
    const_pool: HashMap<usize, ConstantValue>,
    thunk_table: HashMap<usize, u32>,
    bytes: Vec<u8>,
}

impl Default for VM {
    fn default() -> Self {
        VM {
            registers: Register::new(),
            const_pool: HashMap::new(),
            thunk_table: HashMap::new(),
            bytes: vec![],
            metadata: MetaData::default(),
        }
    }
}

impl VM {

    pub fn new_from_file(file_addr: &str) -> Result<Self, RuntimeError> {
        let mut vm = VM::default();
        let bytes = fs::read(file_addr).map_err(|_| RuntimeError { message: "Can't read file".into() })?;
        vm.load(bytes)?;
        Ok(vm)
    }

    pub fn new() -> Result<Self, RuntimeError> {
        let mut vm = VM::default();

        let exe_path = match std::env::current_exe() {
            Ok(path) => path,
            Err(_) => return Err(RuntimeError { message: "error in get file itself".into() })
        };

        let file = match File::open(&exe_path) {
            Ok(f) => f,
            Err(_) => return Err(RuntimeError { message: "error in open file itself".into() })
        };

        let bytecode_size = match read_u32_from_end_of_file(&file) {
            Ok(n) => n,
            Err(_) => return Err(RuntimeError { message: "error in get bytecode size".into() })
        };

        let bytes = match read_n_bytes_from_end_of_file(&file, bytecode_size as u64) {
            Ok(n) => n,
            Err(_) => return Err(RuntimeError { message: "error in get bytecode size".into() })
        };

        vm.load(bytes)?;

        Ok(vm)
    }

    pub fn load(&mut self, bytes: Vec<u8>) -> Result<(), RuntimeError> {
        self.bytes = bytes;
        self.load_metadata()?;
        self.load_const()?;
        self.load_thunk_table()?;
        Ok(())
    }

    pub fn eval(&mut self) -> Result<Value, RuntimeError> {
        let code_offset = self.metadata.code_offset;
        let mut cursor = code_offset as usize;
        let value = self.handle_instruction(&mut cursor)?;
        Ok(value)
    }

    pub fn eval_table(&mut self, str_addr: &str) -> Result<Value, RuntimeError> {
        let code_offset = self.metadata.code_offset;
        let mut cursor = code_offset as usize;
        let mut value: Value = Value::None;

        for key in str_addr.split('.') {
            value = self.handle_instruction(&mut cursor)?;
            if let Value::Table(table) = &value {
                match table.get(vec![key]) {
                    Some(Value::Thunk(thunk_idx)) => {
                        if let Some(thunk_offset) = self.thunk_table.get(&(thunk_idx as usize)) {
                            cursor = (*thunk_offset + code_offset) as usize;
                        }
                        value = Value::Thunk(thunk_idx);
                    }
                    Some(n) => value = n,
                    None => {
                        return Err(RuntimeError {
                            message: format!("property not found: {}", key),
                        });
                    }
                }
            };
        }

        Ok(value)
    }

    pub fn load_metadata(&mut self) -> Result<(), RuntimeError> {
        let mut cursor = 0;
        let magic_code = read_u32(&self.bytes, &mut cursor).ok_or_else(|| RuntimeError {
            message: "Error in get magic code".into(),
        })?;

        if magic_code != MAGIC_NUMBER {
            return Err(RuntimeError { message: "Magic code not suitable".into(),});
        };

        let version = read_u32(&self.bytes, &mut cursor).ok_or_else(|| RuntimeError {
            message: "Error in get version".into(),
        })?;
        let const_offset = read_u32(&self.bytes, &mut cursor).ok_or_else(|| RuntimeError {
            message: "Error in get const_offset".into(),
        })?;
        let const_size = read_u32(&self.bytes, &mut cursor).ok_or_else(|| RuntimeError {
            message: "Error in get const_size".into(),
        })?;
        let thunk_offset = read_u32(&self.bytes, &mut cursor).ok_or_else(|| RuntimeError {
            message: "Error in get thunk_offset".into(),
        })?;
        let thunk_size = read_u32(&self.bytes, &mut cursor).ok_or_else(|| RuntimeError {
            message: "Error in get thunk_size".into(),
        })?;
        let code_offset = read_u32(&self.bytes, &mut cursor).ok_or_else(|| RuntimeError {
            message: "Error in get code_offset".into(),
        })?;
        let code_size = read_u32(&self.bytes, &mut cursor).ok_or_else(|| RuntimeError {
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

        let mut cursor = const_offset as usize;

        for i in 1..(const_size + 1) {
            let const_type =
                read_u8(&self.bytes, &mut cursor).ok_or_else(|| RuntimeError {
                    message: "Error in get const_size".into(),
                })?;

            match const_type {
                0 => {
                    let number =
                        read_i64(&self.bytes, &mut cursor).ok_or_else(|| RuntimeError {
                            message: "Error in get number value const".into(),
                        })?;

                    self.const_pool
                        .insert(i as usize, ConstantValue::Int(number));
                }
                1 => {
                    let str_len =
                        read_u32(&self.bytes, &mut cursor).ok_or_else(|| RuntimeError {
                            message: "Error in get number value const".into(),
                        })?;

                    let string = read_str_with_len(&self.bytes, &mut cursor, str_len as usize)
                        .ok_or_else(|| RuntimeError {
                            message: "Error in get string value const".into(),
                        })?;

                    self.const_pool
                        .insert(i as usize, ConstantValue::String(string));
                }
                _ => {
                    return Err(RuntimeError {
                        message: format!("Unexpect const type {}, {}", const_type, i),
                    });
                }
            }
        }

        Ok(())
    }

    pub fn load_thunk_table(&mut self) -> Result<(), RuntimeError> {
        let thunk_size = self.metadata.thunk_size;
        let thunk_offset = self.metadata.thunk_offset;

        let mut cursor = thunk_offset as usize;

        for i in 1..(thunk_size + 1) {
            let thunk_code_offset =
                read_u32(&self.bytes, &mut cursor).ok_or_else(|| RuntimeError {
                    message: "Error in get thunk code offset".into(),
                })?;

            self.thunk_table.insert(i as usize, thunk_code_offset * 4);
        }

        Ok(())
    }

    pub fn get_thunk_table(&self) -> HashMap<usize, u32> {
        self.thunk_table.clone()
    }

    pub fn get_const(&self, idx: usize) -> Option<&ConstantValue> {
        self.const_pool.get(&idx)
    }

    pub fn handle_instruction(&mut self, cursor: &mut usize) -> Result<Value, RuntimeError> {
        loop {
            let ins = read_u32(&self.bytes, cursor).ok_or_else(|| RuntimeError {
                message: "Error in get ins value".into(),
            })?;

            let opcode = extract_opcode(ins).ok_or_else(|| RuntimeError {
                message: "Error in get opcode".into(),
            })?;

            match opcode {
                OpCode::MTB => {
                    let params = OpCode::MTB.decode(ins);
                    self.registers.set_new_table(params[0] as usize);
                }
                OpCode::MLI => {
                    let params = OpCode::MLI.decode(ins);
                    self.registers.set(params[0] as usize, Value::List(vec![]));
                }
                OpCode::LDC => {
                    let params = OpCode::LDC.decode(ins);
                    let constant =
                        self.const_pool
                            .get(&(params[1] as usize))
                            .ok_or_else(|| RuntimeError {
                                message: "Error in get constant value".into(),
                            })?;
                    match constant {
                        ConstantValue::Int(int) => {
                            self.registers.set(params[0] as usize, Value::Int(*int));
                        }
                        ConstantValue::String(str) => {
                            self.registers
                                .set(params[0] as usize, Value::String(str.to_string()));
                        }
                    }
                }
                OpCode::SAT => {
                    let params = OpCode::SAT.decode(ins);
                    let table_reg = params[0];
                    let key =
                        self.registers
                            .get(params[1] as usize)
                            .ok_or_else(|| RuntimeError {
                                message: "Error in get key in SAT".into(),
                            })?;
                    let value =
                        self.registers
                            .get(params[2] as usize)
                            .ok_or_else(|| RuntimeError {
                                message: "Error in get value in SAT".into(),
                            })?;
                    self.registers
                        .set_attr_table(table_reg as usize, key.to_string()?, value);
                }
                OpCode::ADL => {
                    let params = OpCode::ADL.decode(ins);
                    let list_reg = params[0];
                    let value =
                        self.registers
                            .get(params[2] as usize)
                            .ok_or_else(|| RuntimeError {
                                message: "Error in get value in ADL".into(),
                            })?;
                    self.registers
                        .add_to_list(list_reg as usize, value);
                }
                OpCode::MTK => {
                    let params = OpCode::MTK.decode(ins);
                    let reg = params[0];
                    let thunk_idx = params[1];
                    self.registers.set(reg as usize, Value::Thunk(thunk_idx));
                }
                OpCode::RET => {
                    let params = OpCode::RET.decode(ins);
                    return self
                        .registers
                        .get(params[0] as usize)
                        .ok_or_else(|| RuntimeError {
                            message: "Error in get registers value".into(),
                        });
                }
                _ => {
                    return Err(RuntimeError {
                        message: "Unexpect opcode".into(),
                    });
                }
            }
        }
    }
}
