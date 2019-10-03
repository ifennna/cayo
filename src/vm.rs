use crate::bytecode::*;
use crate::debug;

const TRACE_EXECUTION: bool = true;

pub struct VM {
    chunk: Chunk,
    stack: Vec<Constant>,
    pc: usize,
}

pub enum InterpretationResult {
    Ok,
    CompileError,
    RuntimeError,
}

impl VM {
    pub fn new() -> VM {
        VM {
            chunk: Chunk::new(),
            stack: Vec::<Constant>::new(),
            pc: 0,
        }
    }

    pub fn interpret(&mut self, chunk: Chunk) -> InterpretationResult {
        self.chunk = chunk;
        self.run()
    }

    pub fn pop(&mut self) -> Constant {
        self.stack.pop().unwrap()
    }

    // bytecode dispatch
    fn run(&mut self) -> InterpretationResult {
        loop {
            match &self.read_byte() {
                OpCode::OpReturn => {
                    println!("{:?}", self.stack.pop());
                    return InterpretationResult::Ok;
                }
                OpCode::OpConstant(offset) => {
                    let value: Constant = self.read_value(*offset);
                    self.stack.push(value)
                }
                OpCode::OpNegate => {
                    match self.pop() {
                        Constant::Number(n) => self.stack.push(Constant::Number(-n)),
                    }
                },
            };
        }
    }

    fn read_byte(&mut self) -> OpCode {
        // not the fastest implementation, yet to see if we can acquire a pointer directly
        // into the vector
        let code = self.chunk.code[self.pc];
        if TRACE_EXECUTION {
            for value in &self.stack {
                println!("[{:?}]", value);
            }
            debug::disassemble_instruction(&self.chunk, code, self.pc);
        }
        self.pc += 1;
        code
    }

    fn read_value(&mut self, offset: Offset) -> Constant {
        self.chunk.get_constant(offset)
    }
}
