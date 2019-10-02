use crate::chunk::{Chunk, OpCode, Offset};

enum Iterate {
    Skip,
    Continue
}

pub fn disassemble_chunk(chunk: Chunk, name: &str) {
    println!("=== {} ===", name);
    
    for (index, instruction) in chunk.code.iter().enumerate() {
        if let Iterate::Skip = disassemble_instruction(&chunk, *instruction, index) {
            continue
        }
    }
}

fn disassemble_instruction(chunk: &Chunk, instruction: OpCode, index: usize) -> Iterate {
    print!("{} ", index);

    match instruction {
        OpCode::OpReturn => simple_instruction("OpReturn"),
        OpCode::OpConstant(offset) => constant_instruction("OpConstant", chunk, offset),
    }
}

fn simple_instruction(name: &str) -> Iterate {
    print!("{}\n", name);
    return Iterate::Continue;
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: Offset) -> Iterate {
    let constant = chunk.get_constant(offset);

    print!("{} {:?}\n", name, constant);
    return Iterate::Skip;
}