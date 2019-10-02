mod chunk;
mod debug;

use chunk::{OpCode, Chunk, Constant};
    
fn main() {
    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(Constant::Number(1.2));
    chunk.write(OpCode::OpConstant(constant));

    debug::disassemble_chunk(chunk, "test chunk");
}
