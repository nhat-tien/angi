use super::ast::{Expr, Operator};
use super::error::ParseError;
use super::lexer::Lexer;
use super::token::Token;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn parse(lex: &mut Lexer) -> Result<Expr, ParseError> {
    let mut lexer = lex.peekable();
    skip_new_line(&mut lexer);
    expr_with_bp(&mut lexer, 0)
}

pub fn expr_table(lexer: &mut Peekable<&mut Lexer>) -> Result<Expr, ParseError> {
    let mut attr_set: HashMap<String, Expr> = HashMap::new();
    skip_new_line(lexer);
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
            Some(Ok((line_of_code, token, (start_pos, _)))) => {
                return Err(ParseError {
                    error: format!("Expect Token::Name, but find {:?}", token),
                    location: (line_of_code, start_pos),
                });
            }
            _ => panic!("Unexpect Parser Err"),
        };

        if !matches!(lexer.next(), Some(Ok((_, Token::Equal, (_, _))))) {
            return Err(ParseError {
                error: String::from("Expect equal"),
                location: (0, 0),
            });
        }

        let rhs = expr_with_bp(lexer, 0)?;

        if !matches!(lexer.next(), Some(Ok((_, Token::Semicolon, (_, _))))) {
            return Err(ParseError {
                error: String::from("Expect semicolon"),
                location: (0, 0),
            });
        }

        attr_set.insert(name, rhs);
    }

    Ok(Expr::Table { fields: attr_set })
}

pub fn expr_list(lexer: &mut Peekable<&mut Lexer>) -> Result<Expr, ParseError> {
    let mut items = vec![];
    skip_new_line(lexer);
    loop {
        if let Some(Ok((_, Token::RightBracket, (_, _)))) = lexer.peek() {
            lexer.next();
            break;
        }
        if let Some(Ok((_, Token::NewLine, (_, _)))) = lexer.peek() {
            lexer.next();
            continue;
        }

        let rhs = expr_with_bp(lexer, 0)?;

        items.push(rhs);

        while let Some(Ok((_, Token::NewLine, (_, _)))) = lexer.peek() {
            lexer.next();
            continue;
        }

        if let Some(Ok((_, Token::RightBracket, (_, _)))) = lexer.peek() {
            continue;
        }

        if !matches!(lexer.next(), Some(Ok((_, Token::Comma, (_, _))))) {
            return Err(ParseError {
                error: String::from("Expect Comma"),
                location: (0, 0),
            });
        }
    }

    Ok(Expr::List { items })
}

pub fn expr_with_bp(lexer: &mut Peekable<&mut Lexer>, min_pb: u8) -> Result<Expr, ParseError> {
    let mut lhs = match lexer.next() {
        Some(Ok((_, Token::Number(num), (_, _)))) => Expr::Number(num),
        Some(Ok((_, Token::String(str), (_, _)))) => Expr::LiteralString(str),
        Some(Ok((_, Token::False, (_, _)))) => Expr::Boolean(false),
        Some(Ok((_, Token::True, (_, _)))) => Expr::Boolean(true),
        Some(Ok((_, Token::LeftParen, (_, _)))) => match lexer.peek() {
            Some(Ok((_, Token::RightParen, (_, _)))) => {
                lexer.next();
                expr_function(lexer, vec![])?
            },
            Some(Ok((_, Token::Name(_), (_, _)))) => {
                let params = get_params_of_function(lexer)?;
                expr_function(lexer, params)?
            }
            _ => {
                let lhs = expr_with_bp(lexer, 0)?;

                match lexer.next() {
                    Some(Ok((_, Token::RightParen, (_, _)))) => lhs,
                    _ => {
                        return Err(ParseError {
                            error: format!(
                                "Parse Error: Cannot find Token::RightParen, with {:?}",
                                lhs
                            ),
                            location: (0, 0),
                        });
                    }
                }
            }
        },
        Some(Ok((_, Token::Plus, (_, _)))) => {
            let ((), r_bp) = prefix_binding_power(Operator::Add);
            let rhs = expr_with_bp(lexer, r_bp)?;
            Expr::Unary {
                op: Operator::Add,
                rhs: Box::new(rhs),
            }
        }
        Some(Ok((_, Token::Dash, (_, _)))) => {
            let ((), r_bp) = prefix_binding_power(Operator::Sub);
            let rhs = expr_with_bp(lexer, r_bp)?;
            Expr::Unary {
                op: Operator::Sub,
                rhs: Box::new(rhs),
            }
        }
        Some(Ok((_, Token::Name(name), (_, _)))) => {
            match lexer.peek() {
                Some(Ok((_, Token::LeftParen, (_, _)))) => {
                    lexer.next();
                    expr_calle(lexer)?
                },
                _ => {
                    Expr::Var(name)
                }
            }
        },
        Some(Ok((_, Token::LeftBrace, (_, _)))) => expr_table(lexer)?,
        Some(Ok((_, Token::LeftBracket, (_, _)))) => expr_list(lexer)?,
        t => {
            return Err(ParseError {
                error: format!("bad token, expect left {:?}", t),
                location: (0, 0),
            });
        }
    };

    // if lhs is (var1) then this check the following token it is => or not
    //

    loop {
        let op = match lexer.peek() {
            Some(Ok((_, Token::EndOfFile, (_, _)))) => break,
            Some(Ok((_, Token::NewLine, (_, _)))) => break,
            Some(Ok((_, Token::RightParen, (_, _)))) => break,
            Some(Ok((_, Token::Semicolon, (_, _)))) => break,
            Some(Ok((_, Token::Comma, (_, _)))) => break,
            Some(Ok((_, Token::RightBracket, (_, _)))) => break,
            Some(Ok((_, Token::EqualRightArrow, (_, _)))) => break,
            Some(Ok((_, Token::Plus, (_, _)))) => Operator::Add,
            Some(Ok((_, Token::Dash, (_, _)))) => Operator::Sub,
            Some(Ok((_, Token::Star, (_, _)))) => Operator::Mul,
            Some(Ok((_, Token::Slash, (_, _)))) => Operator::Div,
            t => {
                return Err(ParseError {
                    error: format!("bad token, expect operator {:?}", t),
                    location: (0, 0),
                });
            }
        };

        let (l_pb, r_pb) = infix_binding_power(op);
        if l_pb < min_pb {
            break;
        }
        lexer.next();
        let rhs = expr_with_bp(lexer, r_pb)?;
        lhs = Expr::Binary {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }
    Ok(lhs)
}

//NOTE: Not handle (var, <missing>) yet
fn get_params_of_function(lexer: &mut Peekable<&mut Lexer>) -> Result<Vec<String>, ParseError> {
    let mut params: Vec<String> = vec![];
    loop {
        match lexer.next() {
            Some(Ok((_, Token::Name(name), (_, _)))) => params.push(name),
            Some(Ok((_, Token::Comma, (_, _)))) => continue,
            Some(Ok((_, Token::RightParen, (_, _)))) => break,
            tok => {
                return Err(ParseError {
                    error: format!("Parse Error: Unexpect token when get params, with {:?}", tok),
                    location: (0, 0),
                });
            }
        }
    }
    Ok(params)
}

fn expr_function(
    lexer: &mut Peekable<&mut Lexer>,
    params: Vec<String>,
) -> Result<Expr, ParseError> {
    if !matches!(lexer.next(), Some(Ok((_, Token::EqualRightArrow, (_, _))))) {
        return Err(ParseError {
            error: String::from("Parse Error: Expect '=>' token"),
            location: (0, 0),
        });
    };

    let body = expr_with_bp(lexer, 0)?;

    Ok(Expr::FunctionDeclare {
        params,
        body: Box::new(body),
    })

}

fn expr_calle(lexer: &mut Peekable<&mut Lexer>) -> Result<Expr, ParseError> {

    if let Some(Ok((_, Token::RightParen, (_, _)))) = lexer.peek() {
        return Ok(Expr::FunctionCall{
            args: vec![],
        })
    };

    let mut args: Vec<Expr> = vec![];

    let arg = expr_with_bp(lexer, 0)?;
    args.push(arg);

    loop {
        match lexer.next() {
            Some(Ok((_, Token::RightParen, (_, _)))) => break,
            tok => {
                if !matches!(tok, Some(Ok((_, Token::Comma, (_, _))))) {
                    return Err(ParseError {
                        error: format!("Parse Error: Unexpect ',' when get args, with {:?}", tok),
                        location: (0, 0),
                    });
                };
                let arg = expr_with_bp(lexer, 0)?;
                args.push(arg);
            }
        }
    };

    Ok(Expr::FunctionCall{
        args,
    })
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

fn skip_new_line(lexer: &mut Peekable<&mut Lexer>) {
    while let Some(Ok((_, Token::NewLine, (_, _)))) = lexer.peek() {
        lexer.next();
    }
}
