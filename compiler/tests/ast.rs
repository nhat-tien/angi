use std::collections::HashMap;

use compiler::ast::{Expr, Operator};
use compiler::lexer::Lexer;
use compiler::parser::parse;

#[test]
fn ast_test_plus_two_num() {
    let mut lex = Lexer::new_from_str("100 +300\n");
    let expr = parse(&mut lex);
    assert_eq!(expr, Ok(Expr::Binary {
        op: Operator::Add,
        lhs: Box::new(Expr::Number(100)),
        rhs: Box::new(Expr::Number(300)),
    }));
}

#[test]
fn ast_test_subtract_two_num() {
    let mut lex = Lexer::new_from_str("20- 4\n");
    let expr = parse(&mut lex);
    assert_eq!(expr, Ok(Expr::Binary {
        op: Operator::Sub,
        lhs: Box::new(Expr::Number(20)),
        rhs: Box::new(Expr::Number(4)),
    }));
}


#[test]
fn ast_test_multi_two_num() {
    let mut lex = Lexer::new_from_str("100 *3\n");
    let expr = parse(&mut lex);
    assert_eq!(expr, Ok(Expr::Binary {
        op: Operator::Mul,
        lhs: Box::new(Expr::Number(100)),
        rhs: Box::new(Expr::Number(3)),
    }));
}

#[test]
fn ast_test_divide_two_num() {
    let mut lex = Lexer::new_from_str("20000/ 3\n");
    let expr = parse(&mut lex);
    assert_eq!(expr, Ok(Expr::Binary {
        op: Operator::Div,
        lhs: Box::new(Expr::Number(20000)),
        rhs: Box::new(Expr::Number(3)),
    }));
}

#[test]
fn ast_test_5() {
    let mut lex = Lexer::new_from_str("1 + 2 * 5\n");
    let expr = parse(&mut lex);
    assert_eq!(expr, Ok(Expr::Binary {
        op: Operator::Add,
        lhs: Box::new(Expr::Number(1)),
        rhs: Box::new(Expr::Binary {
            op: Operator::Mul,
            lhs: Box::new(Expr::Number(2)),
            rhs: Box::new(Expr::Number(5)),
        }),
    }));
}


#[test]
fn ast_test_6() {
    let mut lex = Lexer::new_from_str("1 * 2 + 5\n");
    let expr = parse(&mut lex);
    assert_eq!(expr, Ok(Expr::Binary {
        op: Operator::Add,
        lhs: Box::new(Expr::Binary {
            op: Operator::Mul,
            lhs: Box::new(Expr::Number(1)),
            rhs: Box::new(Expr::Number(2)),
        }),
        rhs: Box::new(Expr::Number(5)),
    }));
}

#[test]
fn ast_test_7() {
    let mut lex = Lexer::new_from_str("1 + 2 + 5\n");
    let expr = parse(&mut lex);
    assert_eq!(expr, Ok(Expr::Binary {
        op: Operator::Add,
        lhs: Box::new(Expr::Binary {
            op: Operator::Add,
            lhs: Box::new(Expr::Number(1)),
            rhs: Box::new(Expr::Number(2)),
        }),
        rhs: Box::new(Expr::Number(5)),
    }));
}

#[test]
fn ast_test_8() {
    let mut lex = Lexer::new_from_str("1 + 2 / 12\n");
    let expr = parse(&mut lex);
    assert_eq!(expr, Ok(Expr::Binary {
        op: Operator::Add,
        lhs: Box::new(Expr::Number(1)),
        rhs: Box::new(Expr::Binary {
            op: Operator::Div,
            lhs: Box::new(Expr::Number(2)),
            rhs: Box::new(Expr::Number(12)),
        }),
    }));
}

#[test]
fn ast_test_9() {
    let mut lex = Lexer::new_from_str("3 * (2 + 4)\n");
    let expr = parse(&mut lex);
    assert_eq!(expr, Ok(Expr::Binary {
        op: Operator::Mul,
        lhs: Box::new(Expr::Number(3)),
        rhs: Box::new(Expr::Binary {
            op: Operator::Add,
            lhs: Box::new(Expr::Number(2)),
            rhs: Box::new(Expr::Number(4)),
        }),
    }));
}

#[test]
fn ast_test_10() {
    let mut lex = Lexer::new_from_str("- 100\n");
    let expr = parse(&mut lex);
    assert_eq!(expr, Ok(Expr::Unary {
        op: Operator::Sub,
        rhs: Box::new(Expr::Number(100)),
    }));
}


#[test]
fn ast_test_11() {
    let mut lex = Lexer::new_from_str("- (1 + 3) / 2\n");
    let expr = parse(&mut lex);
    assert_eq!(expr, Ok(
        Expr::Binary {
            op: Operator::Div,
            lhs: Box::new(Expr::Unary {
                op: Operator::Sub,
                rhs: Box::new(Expr::Binary {
                    op: Operator::Add,
                    lhs: Box::new(Expr::Number(1)),
                    rhs: Box::new(Expr::Number(3)),
                }),
            }),
            rhs: Box::new(Expr::Number(2)),
            },
    ));
}


#[test]
fn ast_test_12() {
    let mut lex = Lexer::new_from_str(r#"



(2 + 4) / 3
"#);
    let expr = parse(&mut lex);
    assert_eq!(expr, Ok(Expr::Binary {
        op: Operator::Div,
        lhs: Box::new(Expr::Binary {
            op: Operator::Add,
            lhs: Box::new(Expr::Number(2)),
            rhs: Box::new(Expr::Number(4)),
        }),
        rhs: Box::new(Expr::Number(3)),
    }));
}


#[test]
fn ast_test_table() {
    let mut lex = Lexer::new_from_str(r#"
    {
       name = "Tien";
       age = 10 + 11;
    }
    "#);
    let expr = parse(&mut lex);

    assert_eq!(expr, Ok(Expr::Table {
        fields: HashMap::from([
            (String::from("name"), Expr::LiteralString(String::from("Tien")) ),
            (String::from("age"),
                Expr::Binary {
                    op: Operator::Add,
                    lhs: Box::new(Expr::Number(10)),
                    rhs: Box::new(Expr::Number(11)),
                }),
        ])
    }));
}
