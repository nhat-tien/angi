use std::collections::HashMap;

use crate::ast::Expr;

pub const MAGIC_NUMBER: u32 = 0x414E4749; // "ANGI"
pub const VERSION: u32 = 0x00000001; // "0.0.1"

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
    pub code_offset: u32,
    pub ins_count: u32,
    pub register_in_used: [bool; 16],
    pub ins_code: Vec<u8>
}

impl BytecodeGen {
    pub fn new() -> Self {
        BytecodeGen {
            constants: HashMap::new(),
            thunks: vec![],
            code_offset: 0,
            ins_count: 0,
            register_in_used: [false; 16],
            ins_code: vec![],
        }
    }

    pub fn visit_expr(&mut self, expr: &crate::ast::Expr) {
        match expr {
            Expr::Table { .. } => {
            }
            Expr::LiteralString(str) => {
                self.make_const(Constant::String(str.clone()));
            }
            Expr::Number(num) => {
                self.make_const(Constant::Number(*num));
            }
            _ => panic!("Not implement yet"),
        }
    }

    fn visit_table(&mut self, expr: crate::ast::Expr) {
        if let Expr::Table { fields } = expr {

            let reg_table = self.get_register().expect("Error in get register: table");
            self.emit_ins(instructions::encode_mtb(reg_table.into()));

            for (key, value) in fields.iter() {
                let reg_key = self.get_register().expect("Error in get register: the key");
                let reg_value = self.get_register().expect("Error in get register: the value");

                let idx_const = self.make_const(Constant::String(key.clone()));

                self.emit_ins(instructions::encode_ldc(
                    reg_key as u32,
                    idx_const.try_into().expect("Error when convert idx_const to u32"),
                ));

                match &value {
                    Expr::Number(num) => {
                        let idx_const = self.make_const(Constant::Number(*num));
                        self.emit_ins(instructions::encode_ldc(
                            reg_value as u32,
                            idx_const.try_into().expect("Error when convert idx_const to u32"),
                        ));
                    },
                    Expr::LiteralString(str) => {
                        let idx_const = self.make_const(Constant::String(str.to_string()));
                        self.emit_ins(instructions::encode_ldc(
                            reg_value as u32,
                            idx_const.try_into().expect("Error when convert idx_const to u32"),
                        ));
                    },
                    Expr::Table { .. } => {
                        let idx_thunk = self.make_thunk(value.clone());
                        self.emit_ins(instructions::encode_mtk(
                            reg_value as u32,
                            idx_thunk.try_into().expect("Error when convert idx_const to u32"),
                        ));
                    },
                    expr => panic!("Error: visit_table, not implement yet {:?}", expr)
                }

                self.emit_ins(instructions::encode_sat(
                    reg_table as u32,
                    reg_key as u32,
                    reg_value as u32
                ));

                self.free_register(reg_key as usize);
                self.free_register(reg_value as usize);
            }

            self.free_register(reg_table as usize);
            self.emit_ins(instructions::encode_ret(
                reg_table as u32
            ));
        }
    }

    pub fn visit_thunk() {
        todo!()
    }

    pub fn get_binary(&mut self, expr: crate::ast::Expr) -> Vec<u8> {
        self.visit_table(expr);
        self.add_thunk_code();
        self.calculate_the_code_offset();

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

    pub fn add_header_to_binary(&self,bytes: &mut Vec<u8>) {
        bytes.extend_from_slice(&MAGIC_NUMBER.to_be_bytes());
        bytes.extend_from_slice(&VERSION.to_be_bytes());

        bytes.extend_from_slice(&24_u32.to_be_bytes());                        // const offset in byte
        bytes.extend_from_slice(&(self.constants.len() as u32).to_be_bytes()); // const size
        
        bytes.extend_from_slice(&(self.code_offset.to_be_bytes()));            // thunk_table offset in byte
        bytes.extend_from_slice(&(self.thunks.len() as u32).to_be_bytes());     // thunk_table size
        
        bytes.extend_from_slice(&(
            self.code_offset + (self.thunks.len() as u32 * 4)
        ).to_be_bytes());                                                      // code offset in byte
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

    pub fn add_thunk_code(&mut self) {
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

    pub fn calculate_the_code_offset(&mut self) {
        for constant in self.constants.keys() {
            match constant {
                Constant::Number(_) => {
                    self.code_offset += 9; // 1 byte type + 8 byte num
                }
                Constant::String(str) => {
                    self.code_offset += 5 + str.len() as u32
                    // 1 byte type + 4 byte len + len of str
                }
            }
        }
    }

    pub fn make_thunk(&mut self, expr: Expr) -> usize {
        let idx = self.thunks.len();
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
        println!("{:?}", self.constants);
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
}

