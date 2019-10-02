mod bytecode;
mod debug;

use bytecode::{OpCode, Chunk, Constant};
    
fn main() {
    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(Constant::Number(1.2));
    chunk.write(OpCode::OpConstant(constant), 123);
    chunk.write(OpCode::OpReturn, 123);

    debug::disassemble_chunk(chunk, "test chunk");
}
