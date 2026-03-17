use super::ast::{Expr, Operator};
use super::error::ParseError;
use super::lexer::Lexer;
use super::token::Token;
use crate::diagnostic::{Diagnostic, DiagnosticEngine, Severity, Span};
use std::collections::HashMap;
use std::iter::Peekable;

pub fn parse_with_engine(
    lex: &mut Lexer,
    engine: &mut DiagnosticEngine
) -> Option<Expr> {
    let mut lexer = lex.peekable();
    skip_new_line(&mut lexer);
    expr_with_bp(&mut lexer, engine, 0)
}

pub fn parse(lex: &mut Lexer) -> Result<Expr, ParseError> {
    let mut engine = DiagnosticEngine::new();
    match parse_with_engine(lex, &mut engine) {
        Some(ast) => Ok(ast),
        None => {
            if let Some(diag) = engine.diagnostics.first() {
                Err(ParseError {
                    error: diag.message.clone(),
                    location: (diag.span.line as u32, diag.span.column as u32),
                })
            } else {
                Err(ParseError {
                    error: "Unknown parse error".to_string(),
                    location: (0, 0),
                })
            }
        }
    }
}

fn skip_new_line(lexer: &mut Peekable<&mut Lexer>) {
    while let Some(Ok((_, Token::NewLine, _))) = lexer.peek() {
        lexer.next();
    }
}

fn sync(lexer: &mut Peekable<&mut Lexer>) {
    while let Some(Ok((_, tok, _))) = lexer.peek() {
        match tok {
            Token::Semicolon | Token::RightBrace | Token::RightBracket | Token::NewLine => {
                break;
            }
            _ => {
                lexer.next();
            }
        }
    }
}

fn sync_until<F>(lexer: &mut Peekable<&mut Lexer>, predicate: F)
where
    F: Fn(&Token) -> bool,
{
    while let Some(Ok((_, tok, _))) = lexer.peek() {
        if predicate(tok) {
            return;
        }
        lexer.next();
    }
}

fn report_error(engine: &mut DiagnosticEngine, line: u32, column: u32, message: String) {
    engine.report(Diagnostic {
        severity: Severity::Error,
        message,
        span: Span {
            line: line.max(1) as usize,
            column: column as usize,
        },
        span_len: 1,
        help: None,
        notes: vec![],
    });
}

fn expr_with_bp(
    lexer: &mut Peekable<&mut Lexer>,
    engine: &mut DiagnosticEngine,
    min_pb: u8,
) -> Option<Expr> {
     let mut lhs = match lexer.next() {
        Some(Ok((_, Token::Number(num), (_, _)))) => Expr::Number(num),
        Some(Ok((_, Token::String(str), (_, _)))) => Expr::LiteralString(str),
        Some(Ok((_, Token::False, (_, _)))) => Expr::Boolean(false),
        Some(Ok((_, Token::True, (_, _)))) => Expr::Boolean(true),
        Some(Ok((_, Token::LeftParen, (_, _)))) => match lexer.peek() {
            Some(Ok((_, Token::RightParen, (_, _)))) => {
                lexer.next();
                expr_function(lexer, engine, vec![])?
            },
            Some(Ok((_, Token::Name(_), (_, _)))) => {
                let params = get_params_of_function(lexer, engine)?;
                expr_function(lexer, engine, params)?
            }
            _ => {
                let lhs = expr_with_bp(lexer, engine,  0)?;

                match lexer.next() {
                    Some(Ok((_, Token::RightParen, (_, _)))) => lhs,
                    Some(Ok((line, tok, (col, _)))) => {
                        report_error(engine, line, col, format!("Expected Token::RightParen, found {:?}", tok));
                        return None;
                    },
                    _ => {
                        report_error(engine, 0, 0, "Expected Token::RightParen, found end of life".to_string());
                        return None;
                    }
                }
            }
        },
        Some(Ok((_, Token::Plus, (_, _)))) => {
            let ((), r_bp) = prefix_binding_power(Operator::Add);
            let rhs = expr_with_bp(lexer, engine, r_bp)?;
            Expr::Unary {
                op: Operator::Add,
                rhs: Box::new(rhs),
            }
        }
        Some(Ok((_, Token::Dash, (_, _)))) => {
            let ((), r_bp) = prefix_binding_power(Operator::Sub);
            let rhs = expr_with_bp(lexer, engine, r_bp)?;
            Expr::Unary {
                op: Operator::Sub,
                rhs: Box::new(rhs),
            }
        }
        Some(Ok((_, Token::Name(name), (_, _)))) => {
            match lexer.peek() {
                Some(Ok((_, Token::LeftParen, (_, _)))) => {
                    lexer.next();
                    expr_calle(lexer, engine, name)?
                },
                _ => {
                    Expr::Var(name)
                }
            }
        },
        Some(Ok((_, Token::LeftBrace, (_, _)))) => expr_table(lexer, engine)?,
        Some(Ok((_, Token::LeftBracket, (_, _)))) => expr_list(lexer, engine)?,
        Some(Ok((_, Token::Let, (_, _)))) => expr_let_in(lexer, engine)?,
        Some(Ok((line, tok, (col, _)))) => {
            report_error(engine, line, col, format!("Not implement this token in parser, found {:?}", tok));
            return None;
        },
        _ => {
            report_error(engine, 0, 0, "Bad token in parser".to_string());
            return None;
        }
    };

    // if lhs is (var1) then this check the following token it is => or not
    //

    loop {
        let op = match lexer.peek() {
            Some(Ok((_, Token::EndOfFile, (_, _)))) => break,
            Some(Ok((_, Token::NewLine, (_, _)))) => break,
            Some(Ok((_, Token::RightParen, (_, _)))) => break,
            Some(Ok((_, Token::RightBrace, (_, _)))) => break,
            Some(Ok((_, Token::Semicolon, (_, _)))) => break,
            Some(Ok((_, Token::Comma, (_, _)))) => break,
            Some(Ok((_, Token::RightBracket, (_, _)))) => break,
            Some(Ok((_, Token::EqualRightArrow, (_, _)))) => break,
            Some(Ok((_, Token::Plus, (_, _)))) => Operator::Add,
            Some(Ok((_, Token::Dash, (_, _)))) => Operator::Sub,
            Some(Ok((_, Token::Star, (_, _)))) => Operator::Mul,
            Some(Ok((_, Token::Slash, (_, _)))) => Operator::Div,
            Some(Ok((line, tok, (col, _)))) => {
                report_error(engine, *line, *col, format!("Expect Operator, found {:?}", tok));
                return None;
            },
            _ => {
                report_error(engine, 0, 0, "Bad token in parser, expect Operator".to_string());
                return None;
            }
        };

        let (l_pb, r_pb) = infix_binding_power(op);
        if l_pb < min_pb {
            break;
        }
        lexer.next();
        let rhs = expr_with_bp(lexer, engine, r_pb)?;
        lhs = Expr::Binary {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }
    Some(lhs)
}

fn expr_table(
    lexer: &mut Peekable<&mut Lexer>,
    engine: &mut DiagnosticEngine,
) -> Option<Expr> {
    let mut attr_set = HashMap::new();
    skip_new_line(lexer);
    loop {
        match lexer.peek() {
            Some(Ok((_, Token::RightBrace, _))) => {
                lexer.next();
                break;
            }
            Some(Ok((_, Token::NewLine, _))) => {
                lexer.next();
                continue;
            }
            None => break,
            _ => {
                // Any other token - we'll handle in the main loop
                // break to let the main logic handle it
                // But we need to peek and not consume, so we break out of this inner loop
                // Actually the design is: we look for right brace or newline, otherwise we proceed to parse a field
                break;
            }
        }
    }

    loop {
        match lexer.peek() {
            Some(Ok((_, Token::RightBrace, _))) => {
                lexer.next();
                break;
            }
            Some(Ok((_, Token::NewLine, _))) => {
                lexer.next();
                continue;
            }
            None => break,
            _ => {}
        }


        let name = match lexer.next() {
            Some(Ok((_, Token::Name(name), _))) => name,
            Some(Ok((line, tok, (col, _)))) => {
                report_error(engine, line, col, format!("Expected Token::Name, found {:?}", tok));
                sync(lexer);
                continue;
            }
            Some(Err(e)) => {
                report_error(engine, e.location.0, e.location.1, e.error);
                sync(lexer);
                continue;
            }
            None => break,
        };


        if !expect_token(lexer, engine, Token::Equal) {
            sync(lexer);
            continue;
        }

        let rhs = match expr_with_bp(lexer, engine, 0) {
            Some(expr) => expr,
            None => {
                sync(lexer);
                continue;
            }
        };

        if !expect_token(lexer, engine, Token::Semicolon) {
            sync_until(lexer, |tok| { matches!(tok, Token::Name(_))});
            continue;
        }

        attr_set.insert(name, rhs);
    }

    Some(Expr::Table { fields: attr_set })
}

fn expr_list(
    lexer: &mut Peekable<&mut Lexer>,
    engine: &mut DiagnosticEngine,
) -> Option<Expr> {
    let mut items = vec![];
    skip_new_line(lexer);
    loop {
        match lexer.peek() {
            Some(Ok((_, Token::RightBracket, _))) => {
                lexer.next();
                break;
            }
            Some(Ok((_, Token::NewLine, _))) => {
                lexer.next();
                continue;
            }
            None => break,
            _ => {}
        }

        let rhs = match expr_with_bp(lexer, engine, 0) {
            Some(expr) => expr,
            None => {
                sync(lexer);
                continue;
            }
        };

        items.push(rhs);

        while let Some(Ok((_, Token::NewLine, _))) = lexer.peek() {
            lexer.next();
        }

        if let Some(Ok((_, Token::RightBracket, _))) = lexer.peek() {
            continue;
        }

        if !expect_token(lexer, engine, Token::Comma) {
            sync(lexer);
        }
    }

    Some(Expr::List { items })
}

fn get_params_of_function(
    lexer: &mut Peekable<&mut Lexer>,
    engine: &mut DiagnosticEngine,
) -> Option<Vec<String>> {
    let mut params = vec![];
    loop {
        match lexer.next() {
            Some(Ok((_, Token::Name(name), _))) => params.push(name),
            Some(Ok((_, Token::Comma, _))) => continue,
            Some(Ok((_, Token::RightParen, _))) => break,
            Some(Ok((line, tok, (col, _)))) => {
                report_error(engine, line, col, format!("Expected parameter or ')', found {:?}", tok));
                sync(lexer);
                break;
            }
            Some(Err(e)) => {
                report_error(engine, e.location.0, e.location.1, e.error);
                sync(lexer);
                break;
            }
            None => {
                report_error(engine, 0, 0, "Unexpected end of file in function parameters".to_string());
                break;
            }
        }
    }
    Some(params)
}

fn expr_function(
    lexer: &mut Peekable<&mut Lexer>,
    engine: &mut DiagnosticEngine,
    params: Vec<String>,
) -> Option<Expr> {
    if !expect_token(lexer, engine, Token::EqualRightArrow) {
        sync(lexer);
        return None;
    }

    skip_new_line(lexer);
    let body = match expr_with_bp(lexer, engine, 0) {
        Some(expr) => expr,
        None => {
            sync(lexer);
            return None;
        }
    };

    Some(Expr::FunctionDeclare {
        params,
        body: Box::new(body),
    })
}

fn expr_calle(
    lexer: &mut Peekable<&mut Lexer>,
    engine: &mut DiagnosticEngine,
    calle_name: String,
) -> Option<Expr> {
    if let Some(Ok((_, Token::RightParen, _))) = lexer.peek() {
        lexer.next();
        return Some(Expr::FunctionCall {
            name: calle_name,
            args: vec![],
        });
    }

    let mut args = vec![];

    let arg = match expr_with_bp(lexer, engine, 0) {
        Some(expr) => expr,
        None => {
            report_error(engine, 0, 0, "Expected argument in function call".to_string());
            // Try to recover to closing paren
            while !matches!(lexer.peek(), Some(Ok((_, Token::RightParen, _)))) {
                lexer.next();
                lexer.peek()?;
            }
            lexer.next(); // consume ')'
            return Some(Expr::FunctionCall {
                name: calle_name,
                args,
            });
        }
    };
    args.push(arg);

    loop {
        match lexer.next() {
            Some(Ok((_, Token::RightParen, _))) => break,
            Some(Ok((_, Token::Comma, _))) => {
                let arg = match expr_with_bp(lexer, engine, 0) {
                    Some(expr) => expr,
                    None => {
                        sync(lexer);
                        continue;
                    }
                };
                args.push(arg);
            }
            Some(Ok((line, tok, (col, _)))) => {
                report_error(engine, line, col, format!("Expected ',' or ')', found {:?}", tok));
                sync(lexer);
                continue;
            }
            Some(Err(e)) => {
                report_error(engine, e.location.0, e.location.1, e.error);
                sync(lexer);
                continue;
            }
            None => {
                report_error(engine, 0, 0, "Unexpected end of file in function call".to_string());
                break;
            }
        }
    }

    Some(Expr::FunctionCall {
        name: calle_name,
        args,
    })
}

fn expr_let_in(lexer: &mut Peekable<&mut Lexer>, engine: &mut DiagnosticEngine) -> Option<Expr> {
    let mut attr_set: HashMap<String, Expr> = HashMap::new();
    skip_new_line(lexer);
    loop {
        if let Some(Ok((_, Token::In, (_, _)))) = lexer.peek() {
            lexer.next();
            break;
        }
        if let Some(Ok((_, Token::NewLine, (_, _)))) = lexer.peek() {
            lexer.next();
            continue;
        }

        let name = match lexer.next() {
            Some(Ok((_, Token::Name(name), (_, _)))) => name,
            Some(Ok((line_of_code, _, (start_pos, _)))) => {
                report_error(engine, line_of_code, start_pos, "Atrribute Name not found".to_string());
                sync_until(lexer, |tok| { matches!(tok, Token::In) } );
                continue;
            }
            _ => panic!("Unexpect Parser Err"),
        };


        match lexer.next() {
            Some(Ok((_, Token::Equal, (_, _)))) => {},
            Some(Ok((line_of_code, _, (start_pos, _)))) => {
                report_error(engine, line_of_code, start_pos, "Expect '='".to_string());
                sync_until(lexer, |tok| { matches!(tok, Token::In) } );
                continue;
            }
            _ => panic!("Unexpect Parser Err"),
        };

        let rhs = expr_with_bp(lexer, engine, 0)?;

        if !matches!(lexer.next(), Some(Ok((_, Token::Semicolon, (_, _))))) {
            return None;
        }

        attr_set.insert(name, rhs);
    }

    skip_new_line(lexer);
    let in_part = expr_with_bp(lexer, engine, 0)?;

    Some(Expr::LetIn {
        let_part: attr_set,
        in_part: Box::new(in_part)
    })
}

fn expect_token(
    lexer: &mut Peekable<&mut Lexer>,
    engine: &mut DiagnosticEngine,
    expected: Token,
) -> bool {
    match lexer.next() {
        Some(Ok((line, tok, pos))) => {
            if tok == expected {
                true
            } else {
                report_error(engine, line, pos.0, format!("Expected {:?}, found {:?}", expected, tok));
                false
            }
        }
        Some(Err(e)) => {
            report_error(engine, e.location.0, e.location.1, e.error);
            false
        }
        None => {
            report_error(engine, 0, 0, format!("Expected {:?}, found end of file", expected));
            false
        }
    }
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


