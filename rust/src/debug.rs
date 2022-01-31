use crate::chunk::{Chunk, opcodes};

impl Chunk {
    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);

        let mut offset: usize = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("{:04}    | ", offset);
        } else {
            print!("{:04} {:4} ", offset, self.lines[offset]);
        }

        let inst = self.code[offset];
        match inst {
            opcodes::CONSTANT => {
                constant_instruction("OP_CONSTANT", self, offset)
            }
            opcodes::ADD => {
                simple_instruction("OP_ADD", offset)
            },
            opcodes::SUBTRACT => {
                simple_instruction("OP_SUBTRACT", offset)
            },
            opcodes::MULTIPLY => {
                simple_instruction("OP_MULTIPLY", offset)
            },
            opcodes::DIVIDE => {
                simple_instruction("OP_DIVIDE", offset)
            },
            opcodes::NEGATE => {
                simple_instruction("OP_NEGATE", offset)
            },
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
