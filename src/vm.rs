use crate::bytecode::*;
use crate::{compiler, debug};

const TRACE_EXECUTION: bool = true;

pub struct VM {
    chunk: Chunk,
    stack: Vec<Constant>,
    program_counter: usize,
}

pub type Interpretation = i8;

#[derive(Debug)]
pub enum InterpretationError {
    CompileError,
    RuntimeError,
}

impl VM {
    pub fn new() -> VM {
        VM {
            chunk: Chunk::new(),
            stack: Vec::<Constant>::new(),
            program_counter: 0,
        }
    }

    pub fn interpret(&mut self, source: String) -> Result<Interpretation, InterpretationError> {
        compiler::compile(source).map_err(|err| println!("{:?}", err));
        Ok(0)
        //        self.chunk = chunk;
        //        self.run()
    }

    pub fn pop(&mut self) -> Constant {
        self.stack.pop().unwrap()
    }

    // bytecode dispatch
    fn run(&mut self) -> Result<Interpretation, InterpretationError> {
        loop {
            match &self.read_byte() {
                OpCode::OpReturn => {
                    println!("{:?}", self.pop());
                    return Ok(0);
                }
                OpCode::OpConstant(offset) => {
                    let value: Constant = self.read_value(*offset);
                    self.stack.push(value)
                }
                OpCode::OpNegate => match self.pop() {
                    Constant::Number(n) => self.stack.push(Constant::Number(-n)),
                },
                OpCode::BinaryOperation(operator) => match (self.pop(), self.pop()) {
                    // the first operand in the stack is always on the right side of the expression
                    (Constant::Number(num2), Constant::Number(num1)) => match operator {
                        BinaryOp::Add => self.stack.push(Constant::Number(num1 + num2)),
                        BinaryOp::Subtract => self.stack.push(Constant::Number(num1 - num2)),
                        BinaryOp::Multiply => self.stack.push(Constant::Number(num1 * num2)),
                        BinaryOp::Divide => self.stack.push(Constant::Number(num1 / num2)),
                    },
                },
            };
        }
    }

    fn read_byte(&mut self) -> OpCode {
        // not the fastest implementation, yet to see if we can acquire a pointer directly
        // into the vector
        let code = self.chunk.code[self.program_counter];
        if TRACE_EXECUTION {
            print!("[");
            for value in &self.stack {
                print!("{:?} ", value);
            }
            print!("]\n");
            debug::disassemble_instruction(&self.chunk, code, self.program_counter);
        }
        self.program_counter += 1;
        code
    }

    fn read_value(&mut self, offset: Offset) -> Constant {
        self.chunk.get_constant(offset)
    }
}
