#![no_std]

pub mod instruction;
pub mod interpreter;
pub mod memory;
pub mod stack;
pub mod storage;

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
           PUSH1 0x66
           PUSH1 0
           MSTORE
           PUSH1 0
           MLOAD
        */
        let code = [0x60 as u8, 0x66, 0x60, 0x00, 0x52, 0x60, 0x00, 0x51];

        let mut evm = Interpreter::new(&code);

        let tx = Tx::default();
        evm.run(&tx);
        assert_eq!(evm.stack.top(), 0x66);
    }

    #[test]
    fn storage() {
        /*
           PUSH1 0x66
           PUSH1 0
           SSTORE
           PUSH1 0
           SLOAD
        */
        let code = [0x60 as u8, 0x66, 0x60, 0x00, 0x55, 0x60, 0x00, 0x54];

        let mut evm = Interpreter::new(&code);

        let tx = Tx::default();
        evm.run(&tx);
        assert_eq!(evm.stack.top(), 0x66);
    }
}
