use crate::vm::{Interpretation, InterpretationError, VM};
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

#[derive(Debug)]
pub enum CliError {
    Io(io::Error),
    VM(InterpretationError),
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::Io(err)
    }
}

impl From<InterpretationError> for CliError {
    fn from(err: InterpretationError) -> CliError {
        CliError::VM(err)
    }
}

pub fn file(path: String) -> Result<Interpretation, CliError> {
    let mut vm = VM::new();

    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    vm.interpret(contents).map_err(|err| CliError::from(err))
}

pub fn repl() -> Result<Interpretation, CliError> {
    let mut vm = VM::new();
    let prompt = "=>";

    loop {
        print!("{}", prompt);

        let stdin = io::stdin();
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();

        vm.interpret(line).map_err(|err| CliError::from(err));
    }
}
