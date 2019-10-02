pub type Offset = usize;

#[derive(Copy, Clone)]
pub enum OpCode {
    OpConstant(Offset),
    OpReturn,
}

#[derive(Debug, Copy, Clone)]
pub enum Constant {
    Number(f64)
}

pub struct Chunk {
    pub code: Vec<OpCode>,
    constants: Vec<Constant>
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::<OpCode>::new(),
            constants: Vec::<Constant>::new()
        }
    }

    pub fn write(&mut self, code: OpCode) {
        self.code.push(code)
    }

    pub fn add_constant(&mut self, value: Constant) -> Offset {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn get_constant(&self, position: Offset) -> Constant {
        self.constants[position]
    }
}