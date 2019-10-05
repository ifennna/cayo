mod bytecode;
mod compiler;
mod debug;
mod read;
mod scanner;
mod vm;

use crate::read::CliError;
use std::env;
use std::env::Args;

fn main() {
    let args = env::args();

    match start(args) {
        Ok(_) => println!("done"),
        Err(err) => println!("{:?}", err),
    }
}

fn start(mut args: Args) -> Result<i8, CliError> {
    match args.len() {
        1 => read::repl(),
        2 => read::file(args.nth(1).unwrap()),

        _ => {
            println!("Usage: cayo [path]");
            Ok(0)
        }
    }
}
