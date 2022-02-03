use crate::chunk::{Chunk, opcodes};
use crate::value::Value;

pub const STACK_MAX:usize = 256;

pub struct VM {
    pub chunk: Chunk,
    pub ip: usize,
    pub stack: Vec<Value>,
    trace: bool,
}

#[derive(Debug)]
pub enum InterpretError {
    CompileError,
    RuntimeError,
}

impl VM {
    pub fn new(trace: bool) -> VM {
        let chunk = Chunk::new();
        VM{
            chunk: chunk,
            ip: 0,
            stack: Vec::with_capacity(STACK_MAX),
            trace: trace,
        }
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    #[inline(always)]
    fn read_byte(&mut self) -> u8 {
        let b = self.chunk.code[self.ip];
        self.ip += 1;
        b
    }

    #[inline(always)]
    fn read_constant(&mut self) -> f64 {
        let i = self.read_byte() as usize;
        self.chunk.constants[i]
    }

    #[inline(always)]
    fn binary_op(&mut self, op: fn(Value, Value) -> Value) {
        let b = self.pop();
        let a = self.pop();
        self.push(op(a, b));
    }

    fn run(&mut self) -> Result<(), InterpretError> {
        loop {
            if self.trace {
                print!("          ");
                for v in self.stack.iter() {
                    print!("[ {} ]", v);
                }
                println!("");
                self.chunk.disassemble_instruction(self.ip);
            }

            let inst = self.read_byte();
            match inst {
                opcodes::CONSTANT => {
                    let constant = self.read_constant();
                    self.push(constant);
                    println!("{}", constant);
                }
                opcodes::ADD => self.binary_op(|a, b| { a + b }),
                opcodes::SUBTRACT => self.binary_op(|a, b| { a - b }),
                opcodes::MULTIPLY => self.binary_op(|a, b| { a * b }),
                opcodes::DIVIDE => self.binary_op(|a, b| { a / b }),
                opcodes::NEGATE => {
                    let v = self.pop();
                    self.push(-v);
                }
                opcodes::RETURN => {
                    println!("{}", self.pop());
                    return Ok(());
                }
                _ => {
                    return Err(InterpretError::RuntimeError);
                }
            }
        }
    }

    pub fn interpret(&mut self, chunk: Chunk) -> Result<(), InterpretError> {
        self.chunk = chunk;
        self.ip = 0;
        self.run()
    }
}
