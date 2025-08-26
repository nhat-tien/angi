#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Unary {
        op: Operator,
        rhs: Box<Expr>,
    },
    Binary {
        op: Operator,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Number(i32),
    LiteralString(String),
    Table {},
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Divide,
    Multi,
}
