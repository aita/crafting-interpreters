use crate::chunk::{Chunk, opcode};

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
            opcode::RETURN => {
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
