use std::collections::HashMap;

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
    Boolean(bool),
    Table {
        fields: HashMap<Indentifier, Expr>
    },
    List {
        items: Vec<Expr>
    },
    LetIn {
        let_part: HashMap<Indentifier, Expr>,
        in_part: HashMap<Indentifier, Expr>,
    },
    FunctionDeclare {
        params: Vec<String>,
        body: Box<Expr>
    },
    FunctionCall {
        args: Vec<Expr>
    },
    Var(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Div,
    Mul,
}

type Indentifier = String;
