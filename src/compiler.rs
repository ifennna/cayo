use crate::scanner::{Lexeme, ScanError, Scanner, Token};

pub fn compile(source: String) -> Result<String, ScanError> {
    let mut scanner = Scanner::new(&source);
    let mut line = 0;

    loop {
        let lexeme: Lexeme = scanner.scan_token()?;
        match lexeme.position.line == line {
            true => print!("  | "),
            false => {
                print!("{} ", lexeme.position.line);
                line = lexeme.position.line;
            }
        }
        print!("{:?} {:?}\n", lexeme.token, lexeme.token);

        if let Token::EOF = lexeme.token {
            break;
        }
    }

    Ok(String::new())
}
