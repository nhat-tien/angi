use std::fs;
use crate::lexing::lexer::Lexer;

pub fn parse(file_path: &str) {
    let content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(err) => panic!("Cannot open file {err:?}")
    };

    let char_iter = content.chars();

    let lex = Lexer::new(char_iter);

    for result in lex {
        println!("{result:?}");
    }
}
