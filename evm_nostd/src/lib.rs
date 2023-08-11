#![no_std]

use evm::interpreter::*;

#[no_mangle]
fn main() {
    test_memory();
    test_storage();
}

fn test_memory() {
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

fn test_storage() {
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
