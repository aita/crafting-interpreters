mod chunk;
mod debug;
mod value;
mod vm;

fn main() {
    let mut vm = vm::VM::new(true);
    let mut chunk = chunk::Chunk::new();

    let constant = chunk.add_constant(1.2);
    chunk.write(chunk::opcodes::CONSTANT, 123);
    chunk.write(constant as u8, 123);
    let constant = chunk.add_constant(3.4);
    chunk.write(chunk::opcodes::CONSTANT, 123);
    chunk.write(constant as u8, 123);

    chunk.write(chunk::opcodes::ADD, 123);
    let constant = chunk.add_constant(5.6);
    chunk.write(chunk::opcodes::CONSTANT, 123);
    chunk.write(constant as u8, 123);

    chunk.write(chunk::opcodes::DIVIDE, 123);
    chunk.write(chunk::opcodes::NEGATE, 123);

    chunk.write(chunk::opcodes::RETURN, 123);

    chunk.disassemble("test code");
    vm.interpret(chunk).unwrap();
}
