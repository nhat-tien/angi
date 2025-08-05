
use crate::lexing::token::Token;

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: i32,
    current: i32,
    line: i32
}


impl Lexer {

    pub fn new(soure: String) -> Lexer {
        Lexer {
            source: soure,
            tokens: vec![ ],
            start: 0,
            current: 0,
            line: 1
        }
    }

    pub fn scan_source_to_tokens(&mut self) -> &Vec<Token> {

        for c in self.source.chars() {
            match c {
                '{' => {
                    self.tokens.push(Token::LeftBrace);
                },
                '}' => {
                    self.tokens.push(Token::RightBrace);
                },
                '[' => {
                    self.tokens.push(Token::LeftBracket);
                },
                ']' => {
                    self.tokens.push(Token::RightBracket);
                },
                '=' => {
                    self.tokens.push(Token::Equal);
                },
                ';' => {
                    self.tokens.push(Token::Semicolon);
                }
                '\n' | ' ' => {
                    continue;
                },
                _ => {
                    continue;
                }
            }

        }
        self.tokens.push(Token::Eof);

        &self.tokens
    }
}
