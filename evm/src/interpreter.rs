use crate::instruction::*;
use crate::stack::*;

#[derive(Clone, Debug)]
pub struct Interpreter<'a> {
    pub stack: Stack,
    pub code: &'a [u8],
    pub pc: usize,
}

impl<'a> Interpreter<'a> {
    pub fn new(code: &'a [u8]) -> Self {
        Self {
            stack: Stack::default(),
            code,
            pc: 0,
        }
    }

    pub fn run(&mut self) {
        let mut pc = self.code.iter().cloned();
        while let Some(op) = parse(&mut pc) {
            match op {
                Instruction::Stop => break,
                Instruction::Pop => self.stack.pop(),
                Instruction::Push0 => self.stack.push0(),
                Instruction::Push(byte) => self.stack.push(byte as u64),
                Instruction::Dup(idx) => self.stack.dup(idx as usize),
                Instruction::Swap(idx) => self.stack.swap(idx as usize),
            }
        }
    }
}
