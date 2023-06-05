#[derive(Clone, Debug)]
pub struct Stack {
    length: usize,
    values: [u64; 1024],
}

impl Default for Stack {
    fn default() -> Self {
        Self {
            length: 0,
            values: [0; 1024],
        }
    }
}

impl Stack {
    pub fn push0(&mut self) {
        self.push(0);
    }

    pub fn push(&mut self, v: u64) {
        assert!(self.length < 1024);
        self.values[self.length] = v;
        self.length += 1;
    }

    pub fn pop(&mut self) {
        assert!(self.length > 0);
        self.length -= 1;
    }

    pub fn swap(&mut self, n: usize) {
        assert!(self.length > n);
        let top = self.length - 1;
        self.values.swap(top - n, top);
    }

    pub fn dup(&mut self, n: usize) {
        assert!(self.length >= n);
        self.values[self.length] = self.values[self.length - n];
        self.length += 1;
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn top(&self) -> u64 {
        assert!(self.length > 0);
        self.values[self.length - 1]
    }
}
