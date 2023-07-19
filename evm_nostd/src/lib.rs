#![no_std]

use evm::interpreter::*;

#[no_mangle]
fn main() {
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
