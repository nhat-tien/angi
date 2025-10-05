use std::collections::HashMap;

use instructions::OpCode;

use crate::ast::Expr;

pub const MAGIC_NUMBER: u32 = 0x414E4749; // "ANGI"
pub const VERSION: u32 = 0x00000001; // "0.0.1"
const METADATA_BITS: u32 = 32;

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Constant {
    Number(i32),
    String(String),
}

#[derive(Clone)]
pub struct Thunk {
    pub expr: Expr,
    pub offset: u32
}

#[derive(Default)]
pub struct BytecodeGen {
    pub constants: HashMap<Constant, usize>,
    pub thunks: Vec<Thunk>,
    pub ins_count: u32,
    pub register_in_used: [bool; 16],
    pub ins_code: Vec<u8>
}

impl BytecodeGen {
    pub fn new() -> Self {
        BytecodeGen {
            constants: HashMap::new(),
            thunks: vec![],
            ins_count: 0,
            register_in_used: [false; 16],
            ins_code: vec![],
        }
    }

    pub fn visit_expr(&mut self, expr: crate::ast::Expr) {
        match expr {
            Expr::Table { .. } => {
                self.visit_table(expr);
            }
            Expr::LiteralString(str) => {
                let reg = self.get_register().expect("Error in get register");
                let const_idx = self.make_const(Constant::String(str.clone()));

                self.emit_ins(OpCode::LDC.encode(vec![
                    reg as u32,
                    const_idx.try_into().expect("Error when convert idx_const to u32"),
                ]));

                self.emit_ins(OpCode::RET.encode(vec![
                    reg as u32,
                ]));
            }
            Expr::Number(num) => {
                let reg = self.get_register().expect("Error in get register");
                let const_idx = self.make_const(Constant::Number(num));

                self.emit_ins(OpCode::LDC.encode(vec![
                    reg as u32,
                    const_idx.try_into().expect("Error when convert idx_const to u32"),
                ]));

                self.emit_ins(OpCode::RET.encode(vec![
                    reg as u32,
                ]));
            }
            _ => panic!("Not implement yet"),
        }
    }

    fn visit_table(&mut self, expr: crate::ast::Expr) {
        if let Expr::Table { fields } = expr {

            let reg_table = self.get_register().expect("Error in get register: table");
            self.emit_ins(OpCode::MTB.encode(vec![reg_table as u32]));

            for (key, value) in fields.iter() {
                let reg_key = self.get_register().expect("Error in get register: the key");
                let reg_value = self.get_register().expect("Error in get register: the value");

                let idx_const = self.make_const(Constant::String(key.clone()));

                self.emit_ins(OpCode::LDC.encode(vec![
                    reg_key as u32,
                    idx_const.try_into().expect("Error when convert idx_const to u32"),
                ]));

                match &value {
                    Expr::Number(num) => {
                        let idx_const = self.make_const(Constant::Number(*num));
                        self.emit_ins(OpCode::LDC.encode(vec![
                            reg_value as u32,
                            idx_const.try_into().expect("Error when convert idx_const to u32"),
                        ]));
                    },
                    Expr::LiteralString(str) => {
                        let idx_const = self.make_const(Constant::String(str.to_string()));
                        self.emit_ins(OpCode::LDC.encode(vec![
                            reg_value as u32,
                            idx_const.try_into().expect("Error when convert idx_const to u32"),
                        ]));
                    },
                    Expr::Table { .. } => {
                        let idx_thunk = self.make_thunk(value.clone());
                        self.emit_ins(OpCode::MTK.encode(vec![
                            reg_value as u32,
                            idx_thunk.try_into().expect("Error when convert idx_const to u32"),
                        ]));
                    },
                    expr => panic!("Error: visit_table, not implement yet {:?}", expr)
                }

                self.emit_ins(OpCode::SAT.encode(vec![
                    reg_table as u32,
                    reg_key as u32,
                    reg_value as u32
                ]));

                self.free_register(reg_key as usize);
                self.free_register(reg_value as usize);
            }

            self.free_register(reg_table as usize);
            self.emit_ins(OpCode::RET.encode(vec![
                reg_table as u32
            ]));
        }
    }

    pub fn get_binary(&mut self, expr: crate::ast::Expr) -> Vec<u8> {
        self.visit_expr(expr);
        self.visit_remain_thunk();

        let mut bytes = vec![];
        self.add_header_to_binary(&mut bytes);
        self.add_const_to_binary(&mut bytes);
        self.add_thunk_table_to_binary(&mut bytes);
        self.add_ins_code_to_binary(&mut bytes);

        bytes
    }

    pub fn add_const_to_binary(&mut self, bytes: &mut Vec<u8>) {
        let mut sort_const: Vec<(&Constant, &usize)> = self.constants.iter().collect();

        sort_const.sort_by(|a, b| a.1.cmp(b.1));

        for ( constant, _ )in sort_const {
            match constant {
                Constant::Number(num) => {
                    bytes.extend_from_slice(&0_u8.to_be_bytes());
                    bytes.extend_from_slice(&(*num as i64).to_be_bytes());
                }
                Constant::String(str) => {
                    bytes.extend_from_slice(&1_u8.to_be_bytes());
                    bytes.extend_from_slice(&(str.len() as u32).to_be_bytes());
                    bytes.extend_from_slice(&str.clone().into_bytes());
                }
            }
        }
    }

    pub fn add_header_to_binary(&self, bytes: &mut Vec<u8>) {
        bytes.extend_from_slice(&MAGIC_NUMBER.to_be_bytes());
        bytes.extend_from_slice(&VERSION.to_be_bytes());

        let const_len_in_bytes = self.get_constant_len_in_bytes();
        let thunk_offset = const_len_in_bytes + METADATA_BITS;
        let code_offset = thunk_offset + (self.thunks.len() as u32 * 4);

        bytes.extend_from_slice(&METADATA_BITS.to_be_bytes());                 // const offset in byte
        bytes.extend_from_slice(&(self.constants.len() as u32).to_be_bytes()); // const size
        
        bytes.extend_from_slice(&(thunk_offset.to_be_bytes()));                // thunk_table offset in byte
        bytes.extend_from_slice(&(self.thunks.len() as u32).to_be_bytes());    // thunk_table size
        
        bytes.extend_from_slice(&code_offset.to_be_bytes());                                                      // code offset in byte
        bytes.extend_from_slice(&self.ins_count.to_be_bytes());                // code size
    }

    pub fn add_ins_code_to_binary(&self,bytes: &mut Vec<u8>) {
        bytes.extend_from_slice(&self.ins_code);
    }

    pub fn add_thunk_table_to_binary(&mut self, bytes: &mut Vec<u8>) {
        for thunk in &self.thunks {
             bytes.extend_from_slice(&thunk.offset.to_be_bytes());                // code size
        }
    }

    pub fn visit_remain_thunk(&mut self) {
        let mut idx = 0;
        while idx < self.thunks.len() {
            match self.thunks[idx].expr {
                Expr::Table { .. } => {
                    self.set_offset_thunk(idx, self.ins_count + 1);
                    self.visit_table(self.thunks[idx].expr.clone());
                },
                _ => panic!("Not Impliment Yet")
            }
           idx += 1; 
        }
    }

    pub fn make_thunk(&mut self, expr: Expr) -> usize {
        let idx = self.thunks.len() + 1;
        self.thunks.push(Thunk {
            expr,
            offset: 0
        });
        idx
    }
    pub fn set_offset_thunk(&mut self, idx: usize, offset: u32) {
        let thunk = self.thunks.get_mut(idx).expect("Error: not found thunk at index");
        thunk.offset = offset;
    }

    pub fn make_const(&mut self, constant: Constant) -> usize {
        let idx = self.constants.len() + 1;
        let result = self.constants.entry(constant).or_insert(idx);
        *result
    }

    pub fn get_register(&mut self) -> Option<u8> {
        for i in 0..16 {
            if !self.register_in_used[i] {
                self.register_in_used[i] = true;
                return Some(i as u8);
            }
        }
        None
    }

    pub fn free_register(&mut self, idx: usize) {
        self.register_in_used[idx] = false;
    }

    pub fn emit_ins(&mut self, bytes: [u8;4]) -> usize {
        self.ins_code.extend_from_slice(&bytes);
        self.ins_count += 1;
        self.ins_count as usize
    }

    pub fn get_constant_len_in_bytes(&self) -> u32 {
        let mut const_len = 0;
        for constant in self.constants.keys() {
            match constant {
                Constant::Number(_) => {
                    const_len += 9; // 1 byte type + 8 byte num
                }
                Constant::String(str) => {
                    const_len += 5 + str.len() as u32
                    // 1 byte type + 4 byte len + len of str
                }
            }
        }
        const_len
    }
}



