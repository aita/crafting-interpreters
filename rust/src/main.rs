mod chunk;
mod debug;
mod value;

fn main() {
    let mut chunk = chunk::Chunk::new();

    let constant = chunk.add_constant(1.2);
    chunk.write(chunk::opcodes::CONSTANT);
    chunk.write(constant as u8);

    chunk.write(chunk::opcodes::RETURN);

    chunk.disassemble("test code");
}
