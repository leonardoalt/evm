#![no_std]

pub mod instruction;
pub mod interpreter;
pub mod memory;
pub mod stack;

#[cfg(test)]
mod test {
    use crate::interpreter::*;

    #[test]
    fn simple_bytecode() {
        let code = [0x60 as u8, 0x00];
        let mut evm = Interpreter::new(&code);

        let tx = Tx::default();
        evm.run(&tx);
        assert_eq!(evm.stack.top(), 0);
    }

    #[test]
    fn memory() {
        /*
           PUSH1 0
           PUSH1 0x66
           MSTORE
           PUSH1 0
           MLOAD
        */
        let code = [0x60 as u8, 0x00, 0x60, 0x66, 0x41, 0x60, 0x00, 0x40];

        let mut evm = Interpreter::new(&code);

        let tx = Tx::default();
        evm.run(&tx);
        assert_eq!(evm.stack.top(), 0x66);
    }
}
