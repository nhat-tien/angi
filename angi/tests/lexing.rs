use angi::compiler::token::Token;
use angi::compiler::lexer::{LexResult, Lexer};

#[test]
fn lexing_test_expr_simple() {
    let lex = Lexer::new_from_str("1 + 1");
    let tokens: Vec<LexResult> = lex.collect();
    assert_eq!(tokens, vec![
        Ok((1, Token::Number(1), (1,1))),
        Ok((1, Token::Plus, (3,3))),
        Ok((1, Token::Number(1), (5,5))),
        Ok((1, Token::EndOfFile, (6,6)))
    ])
}

#[test]
fn lexing_test_table() {
    let lex = Lexer::new_from_str(r#"
    {
       name = "Tien";
    }
"#);
    let tokens: Vec<LexResult> = lex.collect();
    assert_eq!(tokens, vec![
        Ok((1, Token::NewLine, (1,1))),
        Ok((2, Token::LeftBrace, (5,5))),
        Ok((2, Token::NewLine, (6,6))),
        Ok((3, Token::Name("name".into()), (8,11))),
        Ok((3, Token::Equal, (13,13))),
        Ok((3, Token::String("Tien".into()), (15,20))),
        Ok((3, Token::Semicolon, (21,21))),
        Ok((3, Token::NewLine, (22,22))),
        Ok((4, Token::RightBrace, (5,5))),
        Ok((4, Token::NewLine, (6,6))),
        Ok((5, Token::EndOfFile, (1,1)))
    ])
}
