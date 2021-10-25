use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct Memory([u8; u16::MAX as usize]);

impl std::default::Default for Memory {
    fn default() -> Self {
        return Self([0u8; u16::MAX as usize]);
    }
}

impl Index<u16> for Memory {
    type Output = u8;
    fn index(&self, index: u16) -> &Self::Output {
        return &self.0[index as usize];
    }
}

impl IndexMut<u16> for Memory {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        return &mut self.0[index as usize];
    }
}

