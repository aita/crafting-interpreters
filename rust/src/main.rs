mod chunk;
mod debug;
mod value;

fn main() {
    let mut chunk = chunk::Chunk::new();

    let constant = chunk.add_constant(1.2);
    chunk.write(chunk::opcodes::CONSTANT, 123);
    chunk.write(constant as u8, 123);

    chunk.write(chunk::opcodes::RETURN, 123);

    chunk.disassemble("test code");
}
