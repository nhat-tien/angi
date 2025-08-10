
#[derive(Debug)]
pub enum Token {

    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    LeftParen,    // (
    RightParen,   // )
    Equal,        // =
    Semicolon,    // ;
    
    Name(String),
    String(String),
    Number(i32),

    // Keyword
    Return,
    
    NewLine,
    EndOfFile,
    None
}

impl Token {
    pub fn str_to_keyword(name: &str) -> Option<Token> {
        match name {
            "return" => Some(Token::Return),
            _ => None
        }
    }
}
