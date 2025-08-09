
#[derive(Debug)]
pub enum Token {

    LeftBrace, // {
    RightBrace, // }
    LeftBracket, // [
    RightBracket, // ]
    Equal, // =
    Semicolon, // ;
    
    Name(String),
    String(String),
    Number(i32),
    
    NewLine,
    Eof
}
