use crate::ast::{AttrSet, Expr, Operator};
use crate::error::ParseError;
use crate::lexer::Lexer;
use crate::token::Token;
use std::iter::Peekable;

pub fn parse(lex: &mut Lexer) -> Result<Expr, ParseError> {
    let mut lexer = lex.peekable();

    while let Some(Ok((_, Token::NewLine, (_, _)))) = lexer.peek() {
        lexer.next();
    }

    if let Some(Ok((line_of_code, token, (start_pos, _)))) = lexer.peek() {
        match token {
            Token::Number(_) | Token::Plus | Token::Dash | Token::LeftParen => {
                Ok(expr_math_bp(&mut lexer, 0))
            }
            Token::LeftBrace => expr_table(&mut lexer),
            _ => Err(ParseError {
                error: format!("Not a expression, {:?}", token),
                location: (*line_of_code, *start_pos),
            }),
        }
    } else {
        Err(ParseError {
            error: String::from("Why we encounter the None/Err in lexer"),
            location: (0, 0),
        })
    }
}

pub fn expr_table(lexer: &mut Peekable<&mut Lexer>) -> Result<Expr, ParseError> {
    let mut attr_set: Vec<AttrSet> = vec![];
    lexer.next(); // Ignore the {
    lexer.next(); // Ignore the \n
    loop {

        if let Some(Ok((_, Token::RightBrace, (_, _)))) = lexer.peek() {
            lexer.next();
            break;
        }
        if let Some(Ok((_, Token::NewLine, (_, _)))) = lexer.peek() {
            lexer.next();
            continue;
        }

        let name = match lexer.next() {
            Some(Ok((_, Token::Name(name), (_, _)))) => name,
            _ => {
                return Err(ParseError {
                    error: String::from("Expect name"),
                    location: (0, 0),
                });
            }
        };

        if !matches!(lexer.next(), Some(Ok((_, Token::Equal, (_, _))))) {
            return Err(ParseError {
                error: String::from("Expect equal"),
                location: (0, 0),
            });
        }

        let rhs = expr_math_bp(lexer, 0);

        if !matches!(lexer.next(), Some(Ok((_, Token::Semicolon, (_, _))))) {
            return Err(ParseError {
                error: String::from("Expect right brace"),
                location: (0, 0),
            });
        }

        attr_set.push(AttrSet {
            key: name,
            value: rhs
        });
    }


    Ok(Expr::Table { properties: attr_set })
}

pub fn expr_math_bp(lexer: &mut Peekable<&mut Lexer>, min_pb: u8) -> Expr {
    let mut lhs = match lexer.next() {
        Some(Ok((_, Token::Number(num), (_, _)))) => Expr::Number(num),
        Some(Ok((_, Token::String(str), (_, _)))) => Expr::LiteralString(str),
        Some(Ok((_, Token::False, (_, _)))) => Expr::Boolean(false),
        Some(Ok((_, Token::True, (_, _)))) => Expr::Boolean(true),
        Some(Ok((_, Token::LeftParen, (_, _)))) => {
            let lhs = expr_math_bp(lexer, 0);
            match lexer.next() {
                Some(Ok((_, Token::RightParen, (_, _)))) => lhs,
                _ => panic!("Parse Error: Cannot find Token::RightParen, with {:?}", lhs),
            }
        }
        Some(Ok((_, Token::Plus, (_, _)))) => {
            let ((), r_bp) = prefix_binding_power(Operator::Add);
            let rhs = expr_math_bp(lexer, r_bp);
            Expr::Unary {
                op: Operator::Add,
                rhs: Box::new(rhs),
            }
        }
        Some(Ok((_, Token::Dash, (_, _)))) => {
            let ((), r_bp) = prefix_binding_power(Operator::Sub);
            let rhs = expr_math_bp(lexer, r_bp);
            Expr::Unary {
                op: Operator::Sub,
                rhs: Box::new(rhs),
            }
        }
        t => panic!("bad token, expect left {:?}", t),
    };

    loop {
        let op = match lexer.peek() {
            Some(Ok((_, Token::EndOfFile, (_, _)))) => break,
            Some(Ok((_, Token::NewLine, (_, _)))) => break,
            Some(Ok((_, Token::RightParen, (_, _)))) => break,
            Some(Ok((_, Token::Semicolon, (_, _)))) => break,
            Some(Ok((_, Token::Plus, (_, _)))) => Operator::Add,
            Some(Ok((_, Token::Dash, (_, _)))) => Operator::Sub,
            Some(Ok((_, Token::Star, (_, _)))) => Operator::Mul,
            Some(Ok((_, Token::Slash, (_, _)))) => Operator::Div,
            t => panic!("bad token, expect operator {:?}", t),
        };

        let (l_pb, r_pb) = infix_binding_power(op);
        if l_pb < min_pb {
            break;
        }
        lexer.next();
        let rhs = expr_math_bp(lexer, r_pb);
        lhs = Expr::Binary {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }
    lhs
}
fn prefix_binding_power(op: Operator) -> ((), u8) {
    match op {
        Operator::Add | Operator::Sub => ((), 9),
        _ => panic!("bad op: {:?}", op),
    }
}

fn infix_binding_power(op: Operator) -> (u8, u8) {
    match op {
        Operator::Add | Operator::Sub => (1, 2),
        Operator::Mul | Operator::Div => (3, 4),
    }
}
