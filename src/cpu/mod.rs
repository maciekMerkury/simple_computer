#[cfg(test)]
mod tests;

pub mod memory;
use memory::*;

// 16 bits let's go
pub mod word;
use word::*;

pub mod instruction;
use instruction::*;

#[derive(Debug, Clone)]
pub struct CPU {
    // more than 64KiB is bloat anyway
    pub memory: Memory,
    // program counter
    pub pc: Word,
    pub registers: [Word; 8],
}

impl CPU {
    pub fn new() -> Self {
        return Self {
            memory: Default::default(),
            pc: Default::default(),
            registers: Default::default(),
        };
    }

    pub fn execute_instruction(&mut self, inst: Instruction) {
        // this can be here, because in case of jump, it is set explicitly
        self.pc.increment();
        use Instruction::*;
        match inst {
            Add(r1, r2, r3) => {
                self.registers[r3]
                    .set_i16(self.registers[r1].to_i16() + self.registers[r2].to_i16());
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
                    self.pc.set_u16(value.into());
                }
            },
        };
    }
}
