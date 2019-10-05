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

#[derive(Debug)]
pub enum ScanError {
    UnknownCharacter(Position, String),
}

pub struct Scanner<'a> {
    source: Chars<'a>,
    current_string: String,
    current_position: Position,
}

impl<'a> Scanner<'a> {
    pub fn new(text: &'a std::string::String) -> Scanner<'a> {
        Scanner {
            source: text.chars(),
            current_string: String::new(),
            current_position: Position::reset(),
        }
    }

    pub fn scan_token(&mut self) -> Result<Lexeme, ScanError> {
        match self.advance() {
            Some("(") => self.make_token(Token::LeftParen),
            Some(")") => self.make_token(Token::RightParen),
            Some("{") => self.make_token(Token::LeftBrace),
            Some("}") => self.make_token(Token::RightBrace),
            Some(";") => self.make_token(Token::SemiColon),
            Some(",") => self.make_token(Token::Comma),
            Some(".") => self.make_token(Token::Dot),
            Some("-") => self.make_token(Token::Minus),
            Some("+") => self.make_token(Token::Plus),
            Some("/") => self.make_token(Token::Slash),
            Some("*") => self.make_token(Token::Star),
            None => self.make_token(Token::EOF),
            _ => Err(ScanError::UnknownCharacter(
                self.current_position,
                String::from(&self.current_string),
            )),
        }
    }

    fn advance(&mut self) -> Option<&str> {
        let mut character;
        self.current_string = String::new();
        loop {
            character = self.source.next();
            match character {
                Some('\t') | Some(' ') | Some('\r') => {
                    self.current_position.increment_column();
                    return Some(&self.current_string);
                }
                Some('\n') => {
                    self.current_position.next_line();
                    if self.current_string.len() > 0 {
                        return Some(&self.current_string);
                    }
                }
                Some(ch) => self.current_string.push(ch),
                None => return None,
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
