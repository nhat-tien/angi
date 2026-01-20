use std::str::Chars;

use super::error::LexicalError;
use super::token::Token;

pub type PostionInLine = (u32, u32); // (startdPosition, endPosition)
pub type LineOfCode = u32;
pub type Spanned = (LineOfCode, Token, PostionInLine);
pub type LexResult = Result<Spanned, LexicalError>;

#[derive(Debug)]
pub struct Lexer<'a>
{
    chars: Chars<'a>,
    pending: Vec<Spanned>,
    chr0: Option<char>,
    chr1: Option<char>,
    current_pos: u32,
    current_loc: u32,
}

impl<'a> Lexer<'a>
{
    pub fn new(chars: Chars<'a>) -> Self {
        let mut lx = Lexer {
            chars,
            pending: Vec::new(),
            chr0: None,
            chr1: None,
            current_loc: 1,
            current_pos: 0,
        };

        lx.move_next_char();
        lx.move_next_char();
        lx.current_pos = 1;

        lx
    }

    pub fn new_from_str(str: &'static str) -> Self {
        let mut lx = Lexer {
            chars: str.chars(),
            pending: Vec::new(),
            chr0: None,
            chr1: None,
            current_loc: 1,
            current_pos: 0,
        };

        lx.move_next_char();
        lx.move_next_char();
        lx.current_pos = 1;

        lx
    }

    fn consume_next(&mut self) -> LexResult {
        while self.pending.is_empty() {
            self.consume()?;
        };

        Ok(self.pending.remove(0))
    }

    fn consume(&mut self) -> Result<(), LexicalError> {
        if let Some(c) = self.chr0 {
            if self.is_name_start(c) {
                let name = self.lex_name()?;
                self.emit(name);
                self.move_next_char();
            } else if self.is_number_start(c, self.chr1) {
                let number = self.lex_number()?;
                self.emit(number);
                self.move_next_char();
            } else {
                self.consume_character(c)?;
            }
        } else {
            let line = self.get_line();
            let tok_pos = self.get_pos();
            self.emit((line, Token::EndOfFile, (tok_pos, tok_pos)));
            self.emit((line, Token::None, (tok_pos, tok_pos)));
        }

        Ok(())
    }

    fn is_number_start(&self, c: char, c1: Option<char>) -> bool {
        match c {
            '0'..='9' => true,
            '-' => matches!(c1, Some('0'..='9')),
            _ => false,
        }
    }

    fn move_next_char(&mut self) {

        if let Some('\r') = self.chr0 {
            if let Some('\n') = self.chr1 {
                self.chr0 = Some('\n');
            } else {
                self.chr0 = Some('\n');
            }
        }

        self.current_pos += 1;
        let next_char = self.chars.next();
        self.chr0 = self.chr1;
        self.chr1 = next_char;
    }

    fn get_pos(&self) -> u32 {
        self.current_pos
    }

    fn get_line(&self) -> u32 {
        self.current_loc
    }

    fn emit(&mut self, spanned: Spanned) {
        self.pending.push(spanned);
    }

    fn is_name_start(&self, c: char) -> bool {
        // matches!(c, 'a'..='z')
        c.is_alphabetic()
    }

    fn is_name_continuation(&self) -> bool {
        self.chr1
            .map(|c| matches!(c, '_' | '0'..='9' | 'a'..='z' | 'A'..='Z'))
            .unwrap_or(false)
    }

    fn is_string_continuation(&self) -> bool {
        match self.chr1 {
            Some('"') => self.is_escaped(),
            Some(_) => true,
            None => false
        }
    }

    fn is_number_continuation(&self) -> bool {
        self.chr1
        .map(|c| matches!(c, '.' | '0'..='9' ))
        .unwrap_or(false)
    }

    fn is_escaped(&self) -> bool {
        self.chr0.map(|c| matches!(c, '\\')).unwrap_or(false)
    }

    fn lex_name(&mut self) -> LexResult {
        let mut name = String::new();
        let line = self.get_line();
        let start_pos = self.get_pos();

        loop {
            name.push(self.chr0.expect("lex_name"));
            if !self.is_name_continuation() {
                break;
            }
            self.move_next_char();
        }

        let end_pos = self.get_pos();

        match Token::str_to_keyword(&name) {
            Some(token) => Ok((line, token, (start_pos, end_pos))),
            None => Ok((line, Token::Name(name), (start_pos, end_pos))),
        }
    }

    fn lex_string(&mut self) -> LexResult {
        let mut string = String::new();
        let line = self.get_line();
        let start_pos = self.get_pos();

        loop {
            if !self.is_string_continuation() {
                break;
            }
            self.move_next_char();
            string.push(self.chr0.expect("lex_string"));
        }

        self.move_next_char(); // Get end position of the last "
        let end_pos = self.get_pos();

        Ok((line, Token::String(string), (start_pos, end_pos)))
    }

    fn lex_number(&mut self) -> LexResult {
        let mut string = String::new();
        let line = self.get_line();
        let start_pos = self.get_pos();

        loop {
            string.push(self.chr0.expect("lex_number"));

            if !self.is_number_continuation() {
                break;
            }
            self.move_next_char();
        }

        let end_pos = self.get_pos();

        let number = string.parse::<i32>().expect("Error in parse number");

        Ok((line, Token::Number(number), (start_pos, end_pos)))
    }

    fn consume_character(&mut self, c: char) -> Result<(), LexicalError> {
        match c {
            '{' => {
                self.emit_one_character(Token::LeftBrace);
            }
            '}' => {
                self.emit_one_character(Token::RightBrace);
            }
            '[' => {
                self.emit_one_character(Token::LeftBracket);
            }
            ']' => {
                self.emit_one_character(Token::RightBracket);
            }
            '(' => {
                self.emit_one_character(Token::LeftParen);
            }
            ')' => {
                self.emit_one_character(Token::RightParen);
            }
            '=' => {
                if matches!(self.chr1, Some('>')) {
                    self.emit_one_character(Token::EqualRightArrow);
                    self.move_next_char();
                } else {
                    self.emit_one_character(Token::Equal);
                }
            }
            ';' => {
                self.emit_one_character(Token::Semicolon);
            }
            ',' => {
                self.emit_one_character(Token::Comma);
            }
            '"' => {
                    let string = self.lex_string()?;
                    self.emit(string);
            }
            '+' => {
                self.emit_one_character(Token::Plus);
            }
            '*' => {
                self.emit_one_character(Token::Star);
            }
            '/' => {
                self.emit_one_character(Token::Slash);
            }
            '%' => {
                self.emit_one_character(Token::Percent);
            }
            '-' => {
                if matches!(self.chr1, Some('>')) {
                    self.emit_one_character(Token::RightArrow);
                    self.move_next_char();
                } else {
                    self.emit_one_character(Token::Dash);
                }
            }
            '|' => {
                if matches!(self.chr1, Some('>')) {
                    self.emit_one_character(Token::Pipe);
                    self.move_next_char();
                } else {
                    self.emit_one_character(Token::Bar);
                }
            }
            '\n' => {
                self.emit_one_character(Token::NewLine);
                self.reset_line();
            }
            _ => {}
        }

        self.move_next_char();

        Ok(())
    }

    fn reset_line(&mut self) {
        self.current_loc += 1;
        self.current_pos = 0;
    }

    fn emit_one_character(&mut self, token: Token) {
        let line = self.get_line();
        let pos = self.get_pos();
        self.emit((line, token, (pos, pos)));
    }
}

impl Iterator for Lexer<'_>
{
    type Item = LexResult;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.consume_next();

        match token {
            Ok((_, Token::None, _)) => None,
            s => Some(s),
        }
    }

}

