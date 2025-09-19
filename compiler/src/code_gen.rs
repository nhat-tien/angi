use crate::ast::Expr;

pub const MAGIC_NUMBER: u32 = 0x414E4749; // "ANGI"
pub const VERSION: u32 = 0x00000001; // "0.0.1"

pub enum Constant {
    Number(i32),
    String(String),
}

pub struct Thunk {
    pub expr: Expr,
}

#[derive(Default)]
pub struct BytecodeGen {
    pub constants: Vec<Constant>,
    pub thunk: Vec<Thunk>,
    pub code_offset: u32,
    pub register_in_used: [bool; 16],
}

impl BytecodeGen {
    pub fn new() -> Self {
        BytecodeGen {
            constants: vec![],
            thunk: vec![],
            code_offset: 0,
            register_in_used: [false; 16],
        }
    }

    pub fn visit_expr(&mut self, expr: &crate::ast::Expr) {
        match expr {
            Expr::Table { fields } => {
                let reg_tab = self.get_register();
                for field in fields {
                    self.make_const(Constant::String(field.key.clone()));
                    self.visit_expr(&field.value);
                }
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

    pub fn get_binary(&mut self, expr: &crate::ast::Expr) -> Vec<u8> {
        self.visit_expr(expr);
        self.calculate_the_code_offset();

        let mut bytes = vec![];

        self.add_header_to_binary(&mut bytes);
        self.add_const_to_binary(&mut bytes);

        bytes
    }

    pub fn add_const_to_binary(&self, bytes: &mut Vec<u8>) {
        for constant in &self.constants {
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
        bytes.extend_from_slice(&1095649097_u32.to_be_bytes()); //ANGI
        bytes.extend_from_slice(&1_u32.to_be_bytes()); //1
        bytes.extend_from_slice(&20_u32.to_be_bytes()); // const offset in byte
        bytes.extend_from_slice(&(self.constants.len() as u32).to_be_bytes()); // const size
        bytes.extend_from_slice(&(self.code_offset.to_be_bytes())); // code offset in byte
        bytes.extend_from_slice(&1_u32.to_be_bytes()); // code size
    }

    pub fn calculate_the_code_offset(&mut self) {
        for constant in &self.constants {
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
        let idx = self.thunk.len();
        self.thunk.push(Thunk { expr });
        idx
    }

    pub fn make_const(&mut self, constant: Constant) -> usize {
        let idx = self.constants.len();
        self.constants.push(constant);
        idx
    }

    pub fn get_register(&self) -> Option<u8> {
        for i in 0..16 {
            if self.register_in_used[i] {
                return Some(i as u8);
            }
        }
        None
    }
}
