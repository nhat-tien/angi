
use std::iter::Peekable;

use crate::lexing::{lexer::Lexer, token::Token};

use super::ast::{Expr, Operator};

pub fn parse(lex: &mut Lexer) -> Expr {
    let mut lexer = lex.peekable();
    expr_bp(&mut lexer, 0)
}

pub fn expr_bp(lexer: &mut Peekable<&mut Lexer>, min_pb: u8) -> Expr {

    let mut lhs = match lexer.next() {
        Some(Ok((_, Token::Number(num), (_,_)))) => Expr::Number(num),
        Some(Ok((_, Token::String(str), (_,_)))) => Expr::LiteralString(str),
        Some(Ok((_, Token::False, (_,_)))) => Expr::Boolean(false),
        Some(Ok((_, Token::True, (_,_)))) => Expr::Boolean(true),
        Some(Ok((_, Token::LeftParen, (_,_)))) => {
            let lhs = expr_bp(lexer, 0);
             match lexer.next() {
                Some(Ok((_, Token::RightParen, (_,_)))) => lhs,
                _ => panic!("Parse Error: Cannot find Token::RightParen, with {:?}", lhs)
            }
        },
        t => {
            if let Some(Ok((_, token, (_,_)))) = &t {
                if token.is_prefix_token() {
                    let op = match token {
                        Token::Plus => Operator::Add,
                        Token::Minus => Operator::Subtract,
                        _ => panic!("Parse Error: Prefix token invalid {:?}", token)
                    };
                    let ((), r_bp) = prefix_binding_power(op);
                    let rhs = expr_bp(lexer, r_bp);
                    return Expr::Unary { op , rhs: Box::new(rhs)}
                }
            }
            panic!("bad token, expect left {:?}", t);
        }
    }; 

    loop {
        let op = match lexer.peek() {
            Some(Ok((_, Token::EndOfFile, (_,_)))) => break,
            Some(Ok((_, Token::NewLine, (_,_)))) => break,
            Some(Ok((_, Token::Plus, (_,_)))) => Operator::Add,
            Some(Ok((_, Token::Minus, (_,_)))) => Operator::Subtract,
            Some(Ok((_, Token::Star, (_,_)))) => Operator::Multi,
            Some(Ok((_, Token::Slash, (_,_)))) => Operator::Divide,
            t => panic!("bad token, expect operator {:?}", t)
        };

        let (l_pb, r_pb) = infix_binding_power(op);
        if l_pb < min_pb {
            break;
        }

        lexer.next();
        let rhs = expr_bp(lexer, r_pb);
        lhs = Expr::Binary { op, lhs: Box::new(lhs), rhs: Box::new(rhs) }
    }

    lhs
}

fn prefix_binding_power(op: Operator) -> ((), u8) {
    match op {
        Operator::Add | Operator::Subtract => ((), 9),
        _ => panic!("bad op: {:?}", op),
    }
}

fn infix_binding_power(op: Operator) -> (u8, u8){
    match op {
       Operator::Add | Operator::Subtract => (1, 2),
       Operator::Multi | Operator::Divide => (3, 4),
    }
}

