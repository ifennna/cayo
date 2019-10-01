use crate::chunk::OpCode;

pub fn disassemble_chunk(chunk: Vec<OpCode>, name: &str) {
    println!("=== {} ===", name);
    
    for (index, instruction) in chunk.iter().enumerate() {
        disassemble_instruction(instruction, index);
    }
}

fn disassemble_instruction(instruction: &OpCode, index: usize) {
    print!("{} ", index);

    match instruction {
        OpCode::OpReturn => simple_instruction("OpReturn"),
    }
}

fn simple_instruction(name: &str) {
    print!("{}\n", name);
}