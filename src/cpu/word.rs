#[derive(Debug, Clone, PartialEq)]
pub struct Word(u16);

impl Word {
    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn to_bytes(&self) -> [u8; 2] {
        return [(self.0 >> 8) as u8 & 255, (self.0 & 255) as u8];
    }

    pub fn from_bytes(bytes: &[u8; 2]) -> Self {
        return Self(((bytes[0] as u16) << 8) | (bytes[1] as u16));
    }

    pub fn new(num: u16) -> Self {
        return Self(num);
    }

    pub fn to_i16(&self) -> i16 {
        return self.0 as i16;
    }

    pub fn set_i16(&mut self, num: i16) {
        self.0 = num as u16;
    }

    pub fn to_u16(&self) -> u16 {
        return self.0;
    }

    pub fn set_u16(&mut self, num: u16) {
        self.0 = num;
    }

    pub fn set_bytes(&mut self, bytes: [u8; 2]) {
        *self = Self::from_bytes(&bytes);
    }
}

impl std::default::Default for Word {
    fn default() -> Self {
        return Self(0);
    }
}
