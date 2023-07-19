extern crate alloc;

use alloc::vec::Vec;

#[derive(Clone, Debug, Default)]
pub struct Memory {
    pub mem: Vec<u64>,
}

impl Memory {
    pub fn size(&self) -> usize {
        self.mem.len()
    }

    pub fn load(&self, idx: usize) -> u64 {
        self.mem[idx]
    }

    pub fn store(&mut self, idx: usize, val: u64) {
        if self.size() <= idx {
            self.mem.resize(idx + 1, 0);
        }
        self.mem[idx] = val;
    }
}
