use crate::instruction::*;
use crate::memory::*;
use crate::stack::*;
use crate::storage::*;

#[derive(Clone, Debug)]
pub struct Interpreter<'a> {
    pub mem: Memory,
    pub storage: Storage,
    pub stack: Stack,
    pub code: &'a [u8],
    pub pc: usize,
}

#[derive(Clone, Debug, Default)]
pub struct Tx<'a> {
    pub value: u64,
    pub from: u64,
    pub to: u64,
    pub data: &'a [u8],
}

impl<'a> Interpreter<'a> {
    pub fn new(code: &'a [u8]) -> Self {
        Self {
            mem: Memory::default(),
            storage: Storage::default(),
            stack: Stack::default(),
            code,
            pc: 0,
        }
    }

    pub fn run(&mut self, _tx: &Tx) {
        let mut pc = self.code.iter().cloned();
        while let Some(op) = parse(&mut pc) {
            match op {
                Instruction::Stop => break,
                Instruction::Mload => self.mload(),
                Instruction::Mstore => self.mstore(),
                Instruction::Sload => self.sload(),
                Instruction::Sstore => self.sstore(),
                Instruction::Pop => _ = self.stack.pop(),
                Instruction::Push0 => self.stack.push0(),
                Instruction::Push(byte) => self.stack.push(byte as u64),
                Instruction::Dup(idx) => self.stack.dup(idx as usize),
                Instruction::Swap(idx) => self.stack.swap(idx as usize),
            }
        }
    }

    fn mload(&mut self) {
        let idx = self.stack.pop();
        self.stack.push(self.mem.load(idx as usize));
    }

    fn mstore(&mut self) {
        let idx = self.stack.pop();
        let val = self.stack.pop();
        self.mem.store(idx as usize, val);
    }

    fn sload(&mut self) {
        let key = self.stack.pop();
        self.stack.push(self.storage.load(key));
    }

    fn sstore(&mut self) {
        let key = self.stack.pop();
        let val = self.stack.pop();
        self.storage.store(key, val);
    }
}
