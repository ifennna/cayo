mod bytecode;
mod debug;
mod vm;

use bytecode::{Chunk, Constant, OpCode};
use vm::VM;

fn main() {
    let mut vm = VM::new();
    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(Constant::Number(1.2));
    chunk.write(OpCode::OpConstant(constant), 123);
    chunk.write(OpCode::OpNegate, 123);
    chunk.write(OpCode::OpReturn, 124);

    vm.interpret(chunk);
}
