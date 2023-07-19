pub enum Instruction {
    Stop,
    Mload,
    Mstore,
    Pop,
    Push0,
    Push(u8),
    Dup(u8),
    Swap(u8),
}

pub fn parse(code: &mut impl Iterator<Item = u8>) -> Option<Instruction> {
    Some(match code.next()? {
        0x00 => Instruction::Stop,
        0x40 => Instruction::Mload,
        0x41 => Instruction::Mstore,
        0x50 => Instruction::Pop,
        0x5f => Instruction::Push0,
        0x60 => Instruction::Push(code.next()?),
        op if (0x80..=0x8f).contains(&op) => Instruction::Dup(op - 0x80 + 1),
        op if (0x90..=0x9f).contains(&op) => Instruction::Swap(op - 0x80 + 1),
        op => panic!("Unknown opcode: 0x{:x}", op),
    })
}
