use crate::ast::Expr;

pub trait Visitor {
    fn visit_expr(&mut self, expr: &Expr);
}

