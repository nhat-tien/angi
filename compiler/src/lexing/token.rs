
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Token {

    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    LeftParen,    // (
    RightParen,   // )
    Equal,        // =
    Semicolon,    // ;
    Comma,        // ,
    RightArrow,   // ->
    Bar,          // |
    
    // Operator
    Plus, // +
    Minus, // -
    Star, // *
    Slash, // / 
    Percent, // %
    
    
    Name(String),
    String(String),
    Number(i32),

    // Keyword
    Return,
    Import,
    
    NewLine,
    EndOfFile,
    None
}

impl Token {
    pub fn str_to_keyword(name: &str) -> Option<Token> {
        match name {
            "return" => Some(Token::Return),
            "import" => Some(Token::Import),
            _ => None
        }
    }
}
