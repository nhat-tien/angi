use crate::ast::Expr;

pub const MAGIC_NUMBER: u32 = 0x414E4749; // "ANGI"
pub const VERSION: u32 = 0x00000001; // "0.0.1"

#[derive(Default)]
pub struct BytecodeGen {
    pub str_constant: Vec<String>
}

impl BytecodeGen {

    pub fn new() -> Self {
        BytecodeGen {
            str_constant: vec![]
        } 
    }

    pub fn visit_expr(&mut self, expr: &crate::ast::Expr) {
        match expr {
            Expr::Table { fields } => {
                for field in fields {
                    self.str_constant.push(field.key.clone());
                    self.visit_expr(&field.value);
                }
            },
            Expr::LiteralString(str) => {
                self.str_constant.push(str.clone());
            },
            Expr::Number(num) => {
                self.str_constant.push(format!("{}", num));
            }
            _ => panic!("Not implement yet")
        }
    }

    pub fn get_binary(&self, expr: &crate::ast::Expr) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&1095649097_u32.to_be_bytes()); //ANGI
        bytes.extend_from_slice(&1_u32.to_be_bytes()); //1
        bytes.extend_from_slice(&1_u32.to_be_bytes()); // const offset
        bytes.extend_from_slice(&1_u32.to_be_bytes()); // const size
        bytes.extend_from_slice(&1_u32.to_be_bytes()); // code offset
        bytes.extend_from_slice(&1_u32.to_be_bytes()); // code size
        

        bytes.extend_from_slice(&1_u32.to_be_bytes()); // code size
        bytes
    }

}

