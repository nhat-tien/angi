#[derive(Debug, PartialEq)]
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
    Boolean(bool),
    Table {
        fields: Vec<AttrSet>
    },
    LetIn {
        let_part: Vec<AttrSet>,
        in_part: Vec<AttrSet>,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Div,
    Mul,
}

type Indentifier = String;

#[derive(Debug, PartialEq)]
pub struct AttrSet {
    pub key: Indentifier,
    pub value: Expr
}
