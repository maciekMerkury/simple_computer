#[cfg(test)]
mod tests;

mod memory;
use memory::*;

// 16 bits let's go
mod word;
use word::*;

#[derive(Debug)]
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
    Jiz(usize, u16)
}

#[derive(Debug, Clone)]
pub struct CPU {
    // more than 64KiB is bloat anyway
    memory: Memory,
    // program counter
    pc: Word,
    registers: [Word; 7],
}

impl CPU {
    pub fn new() -> Self {
        return Self {
            memory: Default::default(),
            pc: Default::default(),
            registers: Default::default(),
        }
    }

    pub fn execute_instruction(&mut self, inst: Instruction) {
        // this can be here, because in case of jump, it is set explicitly
        self.pc.increment();
        use Instruction::*;
        match inst {
            Add(r1, r2, r3) => {
                self.registers[r3].set_i16(self.registers[r1].to_i16() + self.registers[r2].to_i16());
            },

            Inv(r1) => {
                self.registers[r1].set_u16(!self.registers[r1].to_u16());
            },

            Lod(num, position, r1) => {
                let val = self.registers[r1].to_u16();
                self.registers[r1].set_u16(if position {
                    (255_u16 & val) + ((num as u16) << 8)
                } else {
                    (65280_u16 & val) + (num as u16)
                });
            },

            Jiz(r1, value) => {
                if r1 == 0 {
                    self.pc.set_u16(value);
                }
            },
        };
    }
}

