#[allow(unused_imports)]
use crate::ast_parser::ast::{Expr, Operator};
#[allow(unused_imports)]
use crate::lexing::{lexer::{LexResult, Lexer}, token::Token};
#[allow(unused_imports)]
use super::parser::parse;

#[test]
fn ast_test_1() {
    let mut lex = Lexer::new_from_str("1 + 1\n");
    let expr = parse(&mut lex);
    assert_eq!(expr, Expr::Binary {
        op: Operator::Add,
        lhs: Box::new(Expr::Number(1)),
        rhs: Box::new(Expr::Number(1)),
    });
}


#[test]
fn ast_test_2() {
    let mut lex = Lexer::new_from_str("1 + 2\n");
    let expr = parse(&mut lex);
    assert_eq!(expr, Expr::Binary {
        op: Operator::Add,
        lhs: Box::new(Expr::Number(1)),
        rhs: Box::new(Expr::Number(2)),
    });
}

#[test]
fn ast_test_3() {
    let mut lex = Lexer::new_from_str("20- 4\n");
    let expr = parse(&mut lex);
    assert_eq!(expr, Expr::Binary {
        op: Operator::Subtract,
        lhs: Box::new(Expr::Number(20)),
        rhs: Box::new(Expr::Number(4)),
    });
}


#[test]
fn ast_test_4() {
    let mut lex = Lexer::new_from_str("100 *3\n");
    let expr = parse(&mut lex);
    assert_eq!(expr, Expr::Binary {
        op: Operator::Multi,
        lhs: Box::new(Expr::Number(100)),
        rhs: Box::new(Expr::Number(3)),
    });
}

#[test]
fn ast_test_5() {
    let mut lex = Lexer::new_from_str("1 + 2 * 5\n");
    let expr = parse(&mut lex);
    assert_eq!(expr, Expr::Binary {
        op: Operator::Add,
        lhs: Box::new(Expr::Number(1)),
        rhs: Box::new(Expr::Binary {
            op: Operator::Multi,
            lhs: Box::new(Expr::Number(2)),
            rhs: Box::new(Expr::Number(5)),
        }),
    });
}


#[test]
fn ast_test_6() {
    let mut lex = Lexer::new_from_str("1 * 2 + 5\n");
    let expr = parse(&mut lex);
    assert_eq!(expr, Expr::Binary {
        op: Operator::Add,
        lhs: Box::new(Expr::Binary {
            op: Operator::Multi,
            lhs: Box::new(Expr::Number(1)),
            rhs: Box::new(Expr::Number(2)),
        }),
        rhs: Box::new(Expr::Number(5)),
    });
}

#[test]
fn ast_test_7() {
    let mut lex = Lexer::new_from_str("1 + 2 + 5\n");
    let expr = parse(&mut lex);
    assert_eq!(expr, Expr::Binary {
        op: Operator::Add,
        lhs: Box::new(Expr::Binary {
            op: Operator::Add,
            lhs: Box::new(Expr::Number(1)),
            rhs: Box::new(Expr::Number(2)),
        }),
        rhs: Box::new(Expr::Number(5)),
    });
}


#[test]
fn ast_test_8() {
    let mut lex = Lexer::new_from_str("1 + 2 / 12\n");
    let expr = parse(&mut lex);
    assert_eq!(expr, Expr::Binary {
        op: Operator::Add,
        lhs: Box::new(Expr::Number(1)),
        rhs: Box::new(Expr::Binary {
            op: Operator::Divide,
            lhs: Box::new(Expr::Number(2)),
            rhs: Box::new(Expr::Number(12)),
        }),
    });
}
