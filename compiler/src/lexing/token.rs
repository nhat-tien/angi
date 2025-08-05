
#[derive(Debug)]
pub enum Token {

    LeftBrace, // {
    RightBrace, // }
    LeftBracket, // [
    RightBracket, // ]
    Equal, // =
    Semicolon, // ;
    
    Identifier(String),
    String(String),
    Number(i32),

    Eof
}
