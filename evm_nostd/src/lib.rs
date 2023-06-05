#![no_std]

use evm::interpreter::Interpreter;

#[no_mangle]
fn main() {
    let code = [0x60 as u8, 0x00];
    let mut evm = Interpreter::new(&code);
    evm.run();
    assert_eq!(evm.stack.top(), 0);
}
