#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    /// Adds R1 to R2 and stores the result in R3
    /// casts words to i16
    Add(usize, usize, usize),

    /// Inverts R1 and stores it in R1
    Inv(usize),

    /// loads an 8bit number into a specified position in the register R1
    /// for the bool: false means 0 and true means 1
    Lod(u8, bool, usize),

    // inb4 cum: no
    /// if R1 is zero, PC is set to value
    Jiz(usize, u8)
}

// 2 bits for instruction
// 3 bits per register
impl Instruction {
    /// this assumes that all values within self are legal
    // TODO: think about rewriting this in a way that does not assume that the user is not stupid
    pub fn to_word(&self) -> super::Word {
        use Instruction::*;
        use super::Word;
        return match self {
            &Add(r1, r2, r3) => {
                //unimplemented!("Add to byte");
                Word::new(0u16 | ((r1 as u16) << 2u16) | ((r2 as u16) << 5u16) | ((r3 as u16) << 8u16))
            },
            &Inv(r1) => {
                //unimplemented!("Inv to byte");
                Word::new(1u16 | ((r1 as u16) << 2u16))
            },
            &Lod(val, pos, r1) => {
                //unimplemented!("Lod to byte");
                Word::new(2u16 | ((val as u16) << 2u16) | ((pos as u16) << 10u16) | ((r1 as u16) << 11u16))
            },
            &Jiz(r1, val) => {
                //unimplemented!("Jiz to byte");
                Word::new(3u16 | ((r1 as u16) << 2u16) | ((val as u16) << 5u16))
            },
        };
    }
}
