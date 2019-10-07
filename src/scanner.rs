use itertools::MultiPeek;
use std::str::Chars;

#[derive(Debug)]
pub enum Token {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,

    Bang,
    BangEqual,
    Equal,
    DoubleEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Identifier(String),
    StringLiteral(String),
    NumberLiteral(f64),

    And,
    Class,
    Else,
    False,
    For,
    Func,
    If,
    Let,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    While,

    Error(String),

    Comment,
    Whitespace,
    EOF,
}

#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub line: usize,
    column: usize,
}

impl Position {
    pub fn reset() -> Position {
        Position { line: 1, column: 1 }
    }

    fn increment_column(&mut self) {
        self.column += 1;
    }

    fn next_line(&mut self) {
        self.line += 1;
        self.column = 1;
    }
}

#[derive(Debug)]
pub struct Lexeme {
    pub token: Token,
    pub position: Position,
}

fn is_whitespace(c: char) -> bool {
    match c {
        ' ' | '\r' | '\t' | '\n' => true,
        _ => false,
    }
}

#[derive(Debug)]
pub enum ScanError {
    UnknownCharacter(Position, String),
}

pub struct Scanner<'a> {
    source: MultiPeek<Chars<'a>>,
    current_string: String,
    current_position: Position,
}

impl<'a> Scanner<'a> {
    pub fn new(text: &'a std::string::String) -> Scanner<'a> {
        Scanner {
            source: itertools::multipeek(text.chars()),
            current_string: String::new(),
            current_position: Position::reset(),
        }
    }

    pub fn scan_token(&mut self) -> Result<Lexeme, ScanError> {
        self.current_string.clear();
        match self.advance() {
            Some('(') => self.make_token(Token::LeftParen),
            Some(')') => self.make_token(Token::RightParen),
            Some('{') => self.make_token(Token::LeftBrace),
            Some('}') => self.make_token(Token::RightBrace),
            Some(';') => self.make_token(Token::SemiColon),
            Some(',') => self.make_token(Token::Comma),
            Some('.') => self.make_token(Token::Dot),
            Some('-') => self.make_token(Token::Minus),
            Some('+') => self.make_token(Token::Plus),
            Some('/') => {
                if self.peek_match('/') {
                    let token = self.make_token(Token::Comment);
                    self.advance_until_newline();
                    token
                } else {
                    self.make_token(Token::Slash)
                }
            }
            Some('*') => self.make_token(Token::Star),
            Some(c) if is_whitespace(c) => self.make_token(Token::Whitespace),
            None => self.make_token(Token::EOF),
            _ => Err(ScanError::UnknownCharacter(
                self.current_position,
                String::from(&self.current_string),
            )),
        }
    }

    fn advance(&mut self) -> Option<char> {
        let character = self.source.next();
        if let Some(ch) = character {
            self.current_string.push(ch);
            if ch == '\n' {
                self.current_position.next_line();
            } else {
                self.current_position.increment_column();
            }
        }
        character
    }

    fn peek_match(&mut self, ch: char) -> bool {
        if self.source.peek() == Some(&ch) {
            return true;
        }
        false
    }

    fn advance_until_newline(&mut self) {
        loop {
            if let Some('\n') = self.advance() {
                break;
            }
        }
    }

    fn make_token(&self, token_type: Token) -> Result<Lexeme, ScanError> {
        Ok(Lexeme {
            token: token_type,
            position: self.current_position,
        })
    }
}
