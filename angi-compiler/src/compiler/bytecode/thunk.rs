use crate::compiler::ast::Expr;

#[derive(Clone, Debug)]
pub struct Thunk {
    pub expr: Expr,
    pub offset: u32
}

