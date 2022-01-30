use crate::chunk::{Chunk, opcodes};

impl Chunk {
    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);

        let mut offset: usize = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);

        let inst = self.code[offset];
        match inst {
            opcodes::CONSTANT => {
                constant_instruction("OP_CONSTANT", self, offset)
            }
            opcodes::RETURN => {
                simple_instruction("OP_RETURN", offset)
            },
            _ => {
                println!("Unknown opcode {}", inst);
                offset + 1
            }
        }
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk.code[offset + 1] as usize;
    println!("{:-16} {:4} '{}'", name, constant, chunk.constants[constant]);
    offset + 2
}
