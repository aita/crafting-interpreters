mod chunk;
mod debug;

fn main() {
    let mut chunk = chunk::Chunk::new();
    chunk.write(chunk::opcode::RETURN);

    chunk.disassemble("test code");
}
