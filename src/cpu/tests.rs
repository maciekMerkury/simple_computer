#[test]
fn word_bytes() {
    use super::Word;
    let w1 = Word::new(u16::MAX);
    assert!(w1.to_bytes() == [255u8; 2]);

    let bytes: [u8; 2] = [255; 2];
    let w2 = Word::from_bytes(&bytes);
    assert!(w2 == w1);
}

#[test]
fn cpu_add() {
    use super::instruction::Instruction::Add;
    use super::CPU;

    let mut cpu = CPU::new();
    cpu.registers[0].set_i16(1);
    cpu.registers[1].set_i16(1);
    cpu.execute_instruction(Add(0, 1, 2));
    assert!(cpu.registers[2].to_i16() == 2);
    assert!(cpu.pc.to_u16() == 1);
}

#[test]
fn cpu_inv() {
    use super::instruction::Instruction::Inv;
    use super::CPU;

    let mut cpu = CPU::new();
    cpu.registers[0].set_u16(1);
    cpu.execute_instruction(Inv(0));
    assert!(cpu.registers[0].to_u16() == !1);
    assert!(cpu.pc.to_u16() == 1);
}

#[test]
fn cpu_lod() {
    use super::instruction::Instruction::Lod;
    use super::CPU;

    let mut cpu = CPU::new();
    cpu.execute_instruction(Lod(255, false, 0));
    assert!(cpu.registers[0].to_u16() == 255);
    assert!(cpu.pc.to_u16() == 1);
    cpu.execute_instruction(Lod(255, true, 0));
    assert!(cpu.registers[0].to_u16() == u16::MAX);
    assert!(cpu.pc.to_u16() == 2);
}

#[test]
fn cpu_jiz() {
    use super::instruction::Instruction::Jiz;
    use super::CPU;

    let mut cpu = CPU::new();
    cpu.execute_instruction(Jiz(0, u8::MAX));
    assert!(cpu.pc.to_u16() == u8::MAX.into());
}

// Instruction from and to u16

#[test]
fn instructions_to_word() {
    use super::Instruction::*;

    let add = Add(7, 1, 2);
    let b = add.to_word();
    println!("{:?} -> {:#018b}", add, b.to_u16());
    assert!(b == super::Word::new(0b00000_010_001_111_00u16));

    let inv = Inv(0);
    let b = inv.to_word();
    println!("{:?} -> {:#018b}", inv, b.to_u16());
    assert!(b == super::Word::new(0b00000000000_000_01u16));

    let lod = Lod(255, false, 6);
    let b = lod.to_word();
    println!("{:?} -> {:#018b}", lod, b.to_u16());
    assert!(b == super::Word::new(0b00_110_0_11111111_10u16));

    let jiz = Jiz(2, 255);
    let b = jiz.to_word();
    println!("{:?} -> {:#018b}", jiz, b.to_u16());
    assert!(b == super::Word::new(0b000_11111111_010_11u16));
}

#[test]
fn word_to_instructions() {
    use super::Instruction::*;
    use super::{from_word, Word};

    let add = Word::new(0b00000_010_001_111_00u16);
    let mut inst = from_word(&add);
    println!("add: {:?}", inst);
    assert!(inst == Add(7, 1, 2));

    let inv = Word::new(0b00000000000_000_01u16);
    inst = from_word(&inv);
    println!("inv : {:?}", inst);
    assert!(inst == Inv(0));

    let lod = Word::new(0b00_110_0_11111111_10u16);
    inst = from_word(&lod);
    println!("lod: {:?}", inst);
    assert!(inst == Lod(255, false, 6));

    let jiz = Word::new(0b000_11111111_010_11u16);
    inst = from_word(&jiz);
    println!("jiz: {:?}", inst);
    assert!(inst == Jiz(2, 255));
}
