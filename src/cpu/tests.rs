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
    use super::Instruction::Add;

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
    use super::Instruction::Inv;

    let mut cpu = CPU::new();
    cpu.registers[0].set_u16(1);
    cpu.execute_instruction(Inv(0));
    assert!(cpu.registers[0].to_u16() == !1);
    assert!(cpu.pc.to_u16() == 1);
}

#[test]
fn cpu_lod() {
    use super::CPU;
    use super::Instruction::Lod;

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
    use super::Instruction::Jiz;

    let mut cpu = CPU::new();
    cpu.execute_instruction(Jiz(0, u16::MAX));
    assert!(cpu.pc.to_u16() == u16::MAX);
}
