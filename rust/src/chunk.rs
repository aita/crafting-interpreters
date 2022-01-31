use crate::value;

pub mod opcodes {
    macro_rules! opcodes {
        (@step $_idx:expr,) => {};

        (@step $idx:expr, $head:ident, $($tail:ident,)*) => {
            pub const $head: u8 = $idx;

            opcodes!(@step $idx + 1u8, $($tail,)*);
        };

        ($($n:ident),*) => {
            opcodes!(@step 0u8, $($n,)*);
        }
    }

    opcodes!(
        CONSTANT,
        ADD,
        SUBTRACT,
        MULTIPLY,
        DIVIDE,
        NEGATE,
        RETURN
    );

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_opcodes() {
            assert_eq!(CONSTANT, 0x00);
            assert_eq!(RETURN, 0x01);
        }
    }
}

pub struct Chunk {
    pub code: Vec<u8>,
    pub lines: Vec<u32>,
    pub constants: Vec<value::value>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::new(),
            lines: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn write(&mut self, byte: u8, line: u32) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, v: value::value) -> usize {
        self.constants.push(v);
        self.constants.len() - 1
    }
}
