pub mod opcode {
    pub const RETURN: u8 = 0x01;
}

pub struct Chunk {
    pub code: Vec<u8>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk { code: vec![] }
    }

    pub fn write(&mut self, byte: u8) {
        self.code.push(byte);
    }
}
