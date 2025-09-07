pub const MAGIC_NUMBER: u32 = 0x414E4749; // "ANGI"
pub const VERSION: u16 = 0x0001; // "0.0.1"

#[derive(Default)]
pub struct BytecodeGen {
    str_constant: Vec<String>
}

impl BytecodeGen {

    pub fn new() -> Self {
        BytecodeGen {
            str_constant: vec![]
        } 
    }

    fn visit_expr(&mut self, expr: &crate::ast::Expr) {
        todo!()
    }

    pub fn get_binary(&self, expr: &crate::ast::Expr) -> Vec<u8> {
        vec![1,2,3,4,5]
    }
}

