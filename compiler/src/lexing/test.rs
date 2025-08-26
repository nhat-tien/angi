#[allow(unused_imports)]
use crate::lexing::token::Token;
#[allow(unused_imports)]
use super::lexer::{LexResult, Lexer};

#[test]
fn lexing_test_1() {
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
fn lexing_test_2() {
    let lex = Lexer::new_from_str(r#"
    {
       name = "Tien";
       age = 21;
    }
        "#);
    let tokens: Vec<LexResult> = lex.collect();
    assert_eq!(tokens, vec![
        Ok((1, Token::Number(1), (1,1))),
        Ok((1, Token::Plus, (3,3))),
        Ok((1, Token::Number(1), (5,5))),
        Ok((1, Token::EndOfFile, (6,6)))
    ])
}
