use crate::compiler::ast::Expr;

#[derive(Clone, Debug)]
pub struct Function {
    pub offset: u32,
    pub params: Vec<String>,
    pub body: Box<Expr>
}

