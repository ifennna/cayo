pub enum OpCode {
    OpReturn,
}

pub struct Chunk {
    code: Vec<OpCode>
}

impl Chunk {
    pub fn new() -> Chunk {
        return Chunk {
            code: Vec::new()
        }
    }

    pub fn write(&mut self, byte: OpCode) {
        self.code.push(byte);
    }
}