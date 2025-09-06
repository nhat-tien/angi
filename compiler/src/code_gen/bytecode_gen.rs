use super::visitor::Visitor;

pub struct BytecodeGenVisitor {
    str_constant: Vec<String>
}

impl Visitor for BytecodeGenVisitor {

    fn visit_expr(&mut self, expr: &crate::ast::Expr) {
        todo!()
    }
}
