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
        properties: Vec<Property>
    },
    LetIn {
        let_part: Vec<Property>,
        in_part: Vec<Property>,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Divide,
    Multi,
}

type Indentifier = String;

#[derive(Debug, PartialEq)]
pub struct Property {
    key: Indentifier,
    value: Box<Expr>
}
