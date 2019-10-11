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

fn is_digit(c: char) -> bool {
    return c >= '0' && c <= '9';
}

fn is_alpha(c: char) -> bool {
    return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
}

fn check_keyword(input_string: &String, index: usize, token_string: String, token: Token) -> Token {
    if input_string[index..] == token_string {
        return token;
    }

    Token::Identifier(String::from(input_string))
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
            Some('*') => self.make_token(Token::Star),
            Some('!') => {
                if self.peek_match('=') {
                    self.make_token(Token::BangEqual)
                } else {
                    self.make_token(Token::Bang)
                }
            }
            Some('=') => {
                if self.peek_match('=') {
                    self.make_token(Token::DoubleEqual)
                } else {
                    self.make_token(Token::Equal)
                }
            }
            Some('>') => {
                if self.peek_match('=') {
                    self.make_token(Token::GreaterEqual)
                } else {
                    self.make_token(Token::Greater)
                }
            }
            Some('<') => {
                if self.peek_match('=') {
                    self.make_token(Token::LessEqual)
                } else {
                    self.make_token(Token::Less)
                }
            }
            Some('/') => {
                if self.peek_match('/') {
                    let token = self.make_token(Token::Comment);
                    self.advance_until_newline();
                    token
                } else {
                    self.make_token(Token::Slash)
                }
            }
            Some('"') => self.make_string(),
            Some(c) if is_whitespace(c) => self.make_token(Token::Whitespace),
            Some(c) if is_digit(c) => self.make_digit(),
            Some(c) if is_alpha(c) => self.make_identifier(),
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
            self.source.next();
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

    fn make_string(&mut self) -> Result<Lexeme, ScanError> {
        // remove the starting '"'
        self.current_string.pop();
        loop {
            self.advance();
            if let Some('"') = self.source.peek() {
                break;
            }
        }
        // skip the trailing '"'
        self.source.next();
        self.make_token(Token::StringLiteral(String::from(&self.current_string)))
    }

    fn make_digit(&mut self) -> Result<Lexeme, ScanError> {
        let mut decimal_count = 1;
        loop {
            match self.source.peek() {
                // handle decimals if present
                Some('.') if decimal_count != 0 => match self.source.peek() {
                    // ensure digit after decimal is a valid number, if not we treat the
                    // decimal as a dot instead
                    Some(&ch) if is_digit(ch) => {
                        decimal_count -= 1;
                        self.advance();
                    }
                    _ => {}
                },
                Some(&c) if is_digit(c) => {
                    self.advance();
                }
                _ => break,
            }
        }

        self.make_token(Token::NumberLiteral(self.current_string.parse().unwrap()))
    }

    fn make_identifier(&mut self) -> Result<Lexeme, ScanError> {
        loop {
            match self.source.peek() {
                Some(&ch) if is_alpha(ch) || is_digit(ch) => {
                    self.advance();
                }
                _ => break,
            }
        }

        let token_type = self.check_identifier_type();

        self.make_token(token_type)
    }

    fn check_identifier_type(&mut self) -> Token {
        let mut current_chars = itertools::multipeek(self.current_string.chars());
        match current_chars.peek().unwrap() {
            'a' => check_keyword(&self.current_string, 1, "nd".into(), Token::And),
            'c' => check_keyword(&self.current_string, 1, "lass".into(), Token::Class),
            'e' => check_keyword(&self.current_string, 1, "lse".into(), Token::Else),
            'f' if self.current_string.len() > 1 => match current_chars.peek().unwrap() {
                'a' => check_keyword(&self.current_string, 2, "lse".into(), Token::False),
                'o' => check_keyword(&self.current_string, 2, "r".into(), Token::For),
                'u' => check_keyword(&self.current_string, 2, "nc".into(), Token::Func),
                _ => Token::Identifier(String::from(&self.current_string)),
            },
            'i' => check_keyword(&self.current_string, 1, "f".into(), Token::If),
            'l' => check_keyword(&self.current_string, 1, "f".into(), Token::Let),
            'n' => check_keyword(&self.current_string, 1, "il".into(), Token::Nil),
            'o' => check_keyword(&self.current_string, 1, "hile".into(), Token::Or),
            'p' => check_keyword(&self.current_string, 1, "hile".into(), Token::Print),
            'r' => check_keyword(&self.current_string, 1, "hile".into(), Token::Return),
            's' => check_keyword(&self.current_string, 1, "hile".into(), Token::Super),
            't' if self.current_string.len() > 1 => match current_chars.peek().unwrap() {
                'h' => check_keyword(&self.current_string, 2, "is".into(), Token::This),
                'r' => check_keyword(&self.current_string, 2, "ue".into(), Token::True),
                _ => Token::Identifier(String::from(&self.current_string)),
            },
            'w' => check_keyword(&self.current_string, 1, "hile".into(), Token::While),
            _ => Token::Identifier(String::from(&self.current_string)),
        }
    }

    fn make_token(&self, token_type: Token) -> Result<Lexeme, ScanError> {
        Ok(Lexeme {
            token: token_type,
            position: self.current_position,
        })
    }
}
