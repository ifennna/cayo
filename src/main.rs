mod bytecode;
mod debug;
mod vm;

use crate::bytecode::BinaryOp;
use bytecode::{Chunk, Constant, OpCode};
use std::env;
use vm::VM;

fn main() {
    let mut vm = VM::new();
    let mut chunk = Chunk::new();

    let mut constant = chunk.add_constant(Constant::Number(1.2));
    chunk.write(OpCode::OpConstant(constant), 123);
    constant = chunk.add_constant(Constant::Number(3.6));
    chunk.write(OpCode::OpConstant(constant), 123);

    chunk.write(OpCode::BinaryOperation(BinaryOp::Add), 124);

    constant = chunk.add_constant(Constant::Number(5.8));
    chunk.write(OpCode::OpConstant(constant), 125);

    chunk.write(OpCode::BinaryOperation(BinaryOp::Divide), 126);
    chunk.write(OpCode::OpReturn, 127);

    vm.interpret(chunk);
}
