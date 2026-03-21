use crate::compiler::ast::Expr;

#[derive(Clone)]
pub struct Thunk {
    pub expr: Expr,
    pub offset: u32
}

