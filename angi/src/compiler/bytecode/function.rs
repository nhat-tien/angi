use crate::compiler::ast::Expr;

#[derive(Clone, Debug)]
pub struct Function {
    pub offset: u32,
    pub params: Vec<String>,
    pub body: Box<Expr>
}


impl Function {

    pub fn from_epxr(expr: Expr) -> Option<Self> {
        if let Expr::FunctionDeclare { params, body } = expr {
            Some(Function {
                offset: 0,
                params,
                body
            })
        } else {
            None
        }
    }
    
}
