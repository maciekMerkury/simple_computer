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
    use super::CPU;
    use super::instruction::Instruction::Add;

    let mut cpu = CPU::new();
    cpu.registers[0].set_i16(1);
    cpu.registers[1].set_i16(1);
    cpu.execute_instruction(Add(0, 1, 2));
    assert!(cpu.registers[2].to_i16() == 2);
    assert!(cpu.pc.to_u16() == 1);
}

#[test]
fn cpu_inv() {
    use super::CPU;
    use super::instruction::Instruction::Inv;

    let mut cpu = CPU::new();
    cpu.registers[0].set_u16(1);
    cpu.execute_instruction(Inv(0));
    assert!(cpu.registers[0].to_u16() == !1);
    assert!(cpu.pc.to_u16() == 1);
}

#[test]
fn cpu_lod() {
    use super::CPU;
    use super::instruction::Instruction::Lod;

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
    use super::CPU;
    use super::instruction::Instruction::Jiz;

    let mut cpu = CPU::new();
    cpu.execute_instruction(Jiz(0, u8::MAX));
    assert!(cpu.pc.to_u16() == u8::MAX.into());
}

// Instruction from and to u16
#[test]
fn conv_add() {
    use super::Instruction::Add;
    let add = Add(7, 1, 2);
    let b = add.to_word();
    println!("{:?} -> {:#018b}", add, b.to_u16());
    assert!(b == super::Word::new(0b00000_010_001_111_00u16));
}

#[test]
fn conv_inv() {
    use super::Instruction::Inv;
    let inv = Inv(0);
    let b = inv.to_word();
    println!("{:?} -> {:#018b}", inv, b.to_u16());
    assert!(b == super::Word::new(0b00000000000_000_01u16));
}

#[test]
fn conv_lod() {
    use super::Instruction::Lod;
    let lod = Lod(255, false, 6);
    let b = lod.to_word();
    println!("{:?} -> {:#018b}", lod, b.to_u16());
    assert!(b == super::Word::new(0b00_110_0_11111111_10u16));
    
}

#[test]
fn conv_jiz() {
    use super::Instruction::Jiz;
    let jiz = Jiz(2, 255);
    let b = jiz.to_word();
    println!("{:?} -> {:#018b}", jiz, b.to_u16());
    assert!(b == super::Word::new(0b000_11111111_010_11u16));
}

