use std::mem;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BitSet {
    bits: usize,
}

impl BitSet {
    pub fn with(bit: usize) -> Self {
        check(bit);
        Self { bits: 1 << bit }
    }

    pub fn is_enabled(&self, bit: usize) -> bool {
        check(bit);
        let bit_index = 1 << bit;
        (self.bits & bit_index) == bit_index
    }

    pub fn enable(&mut self, bit: usize) {
        check(bit);
        let bit_index = 1 << bit;
        self.bits |= bit_index;
    }

    pub fn intersection(&self, other: &BitSet) -> Self {
        Self { bits: self.bits & other.bits }
    }
}

fn check(value: usize) {
    if mem::size_of::<usize>() * 8 <= value {
        panic!("out of bounds");
    }
}