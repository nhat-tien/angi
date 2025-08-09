use crate::lexing::error::LexicalError;
use crate::lexing::token::Token;

pub type PostionInLine = (i32, i32); // (startdPosition, endPosition)
pub type LineOfCode = i32;
pub type Spanned = (LineOfCode, Token, PostionInLine);
pub type LexResult = Result<Spanned, LexicalError>;

#[derive(Debug)]
pub struct Lexer<T>
where
    T: Iterator<Item = char>,
{
    chars: T,
    pending: Vec<Spanned>,
    chr0: Option<char>,
    chr1: Option<char>,
    current_pos: i32,
    current_loc: i32,
}

impl<T> Lexer<T>
where
    T: Iterator<Item = char>,
{
    pub fn new(chars: T) -> Self {
        let mut lx = Lexer {
            chars,
            pending: Vec::new(),
            chr0: None,
            chr1: None,
            current_loc: 1,
            current_pos: -1,
        };
        let _ = lx.next_char();
        let _ = lx.next_char();
        lx
    }

    fn consume_next(&mut self) -> LexResult {
        while self.pending.is_empty() {
            self.consume()?;
        }

        Ok(self.pending.remove(0))
    }

    fn consume(&mut self) -> Result<(), LexicalError> {
        if let Some(c) = self.chr0 {
            if self.is_name_start(c) {
                let name = self.lex_name()?;
                self.emit(name);
            } else {
                self.consume_character(c)?;
            }
        } else {
            let line = self.get_line();
            let tok_pos = self.get_pos();
            self.emit((line, Token::Eof, (tok_pos, tok_pos)));
        }

        Ok(())
    }

    fn next_char(&mut self) -> Option<char> {
        if let Some('\r') = self.chr0 {
            if let Some('\n') = self.chr1 {
                self.chr0 = Some('\n');
            } else {
                self.chr0 = Some('\n');
            }
        }

        // if self.chr0.is_some() || self.chr1.is_some() {
        //     self.current_pos += 1;
        // }

        self.current_pos += 1;
        let c = self.chr0;
        let next_char = self.chars.next();
        self.chr0 = self.chr1;
        self.chr1 = next_char;
        c
    }

    fn get_pos(&self) -> i32 {
        self.current_pos
    }

    fn get_line(&self) -> i32 {
        self.current_loc
    }

    fn emit(&mut self, spanned: Spanned) {
        self.pending.push(spanned);
    }

    fn is_name_start(&self, c: char) -> bool {
        matches!(c, '_' | 'a'..='z')
    }

    fn is_name_continuation(&self) -> bool {
        self.chr0
            .map(|c| matches!(c, '_' | '0'..='9' | 'a'..='z' | 'A'..='Z'))
            .unwrap_or(false)
    }

    fn lex_name(&mut self) -> LexResult {
        let mut name = String::new();

        let line = self.get_line();

        let start_pos = self.get_pos();

        while self.is_name_continuation() {
            name.push(self.next_char().expect("lex_name"));
        }

        let end_pos = self.get_pos();

        match name {
            _ => Ok((line, Token::Name(name), (start_pos, end_pos))),
        }
    }

    fn consume_character(&mut self, c: char) -> Result<(), LexicalError> {
        let line = self.get_line();
        let pos = self.get_pos();

        match c {
            '{' => {
                self.emit((line, Token::LeftBrace, (pos, pos)));
            }
            '}' => {
                self.emit((line, Token::RightBrace, (pos, pos)));
            }
            '[' => {
                self.emit((line, Token::LeftBracket, (pos, pos)));
            }
            ']' => {
                self.emit((line, Token::RightBracket, (pos, pos)));
            }
            '=' => {
                self.emit((line, Token::Equal, (pos, pos)));
            }
            ';' => {
                self.emit((line, Token::Semicolon, (pos, pos)));
            }
            '\n' => {
                self.current_loc += 1;
                self.current_pos = 0;
                self.emit((line, Token::NewLine, (pos, pos)));
            }
            _ => {}
        }

        let _ = self.next_char();

        Ok(())
    }
}

impl<T> Iterator for Lexer<T>
where
    T: Iterator<Item = char>,
{
    type Item = LexResult;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.consume_next();

        match token {
            Ok((_, Token::Eof, _)) => None,
            s => Some(s),
        }
    }
}
