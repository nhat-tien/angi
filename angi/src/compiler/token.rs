#[derive(Debug, PartialEq)]
pub enum Token {
    LeftBrace,       // {
    RightBrace,      // }
    LeftBracket,     // [
    RightBracket,    // ]
    LeftParen,       // (
    RightParen,      // )
    Equal,           // =
    Semicolon,       // ;
    Comma,           // ,
    RightArrow,      // ->
    EqualRightArrow, // => 
    Bar,             // |

    // Operator
    Plus,            // +
    Dash,            // -
    Star,            // *
    Slash,           // / 
    Percent,         // %
    Pipe,            // |>
    Bind,            // >>=
    
    Name(String),
    String(String),
    Number(i32),

    Let,
    In,
    True,
    False,
    
    NewLine,
    EndOfFile,
    None
}

impl Token {
    pub fn str_to_keyword(name: &str) -> Option<Token> {
        match name {
            "let"   => Some(Token::Let),
            "in"    => Some(Token::In),
            "true"  => Some(Token::True),
            "false" => Some(Token::False),
            _ => None
        }
    }

    pub fn is_prefix_token(&self) -> bool {
        matches!(self, Token::Plus | Token::Dash)
    }
}
