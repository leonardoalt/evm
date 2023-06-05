#![no_std]

pub mod instruction;
pub mod interpreter;
pub mod stack;

#[cfg(test)]
mod test {
    use crate::interpreter::*;

    #[test]
    fn simple_bytecode() {
        let code = [0x60 as u8, 0x00];
        let mut evm = Interpreter::new(&code);
        evm.run();
        assert_eq!(evm.stack.top(), 0);
    }
}
