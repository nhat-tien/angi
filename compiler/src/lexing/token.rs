
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
    Dash, // -
    Star, // *
    Slash, // / 
    Percent, // %
    
    
    Name(String),
    String(String),
    Number(i32),

    // Reserved 
    Return,
    Import,
    True,
    False,
    
    NewLine,
    EndOfFile,
    None
}

impl Token {
    pub fn str_to_keyword(name: &str) -> Option<Token> {
        match name {
            "return" => Some(Token::Return),
            "import" => Some(Token::Import),
            "true" => Some(Token::True),
            "false" => Some(Token::False),
            _ => None
        }
    }

    pub fn is_prefix_token(&self) -> bool {
        matches!(self, Token::Plus | Token::Dash)
    }
}
