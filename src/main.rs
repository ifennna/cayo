mod chunk;

use chunk::Chunk;
use chunk::OpCode;
    
fn main() {
    let mut chunk = Chunk::new();
    chunk.write(OpCode::OpReturn);
}
