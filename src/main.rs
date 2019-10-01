mod chunk;
mod debug;

use chunk::OpCode;
    
fn main() {
    let mut chunk = Vec::<OpCode>::new();
    chunk.push(OpCode::OpReturn);

    debug::disassemble_chunk(chunk, "test chunk");
}
