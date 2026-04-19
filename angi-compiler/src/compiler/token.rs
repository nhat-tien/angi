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
    Dot,             // .
    DoubleDot,       // ..

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
    MultilineString(String),
    Number(i32),

    //interpolation
    StringStart,
    InterpStart,
    InterpEnd,
    StringEnd,

    // Preserve keyword
    Let,
    In,
    True,
    False,
    EnumDeclare,
    TableDeclare,

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
            "enum" => Some(Token::EnumDeclare),
            "table" => Some(Token::TableDeclare),
            _ => None
        }
    }

    pub fn is_prefix_token(&self) -> bool {
        matches!(self, Token::Plus | Token::Dash)
    }

    pub fn to_str_symbol(&self) -> &str {
        match self {
            Token::Semicolon => ";",
            _ => todo!()
        }
    }
}
