// TODO: rename this
#[test]
fn overall_cpu_test() {
    use crate::assembler::parse_line;
    use crate::cpu::instruction::Instruction::{self, *};
    use crate::*;
    let mut cpu = cpu::CPU::new();

    let instruction = "lod 255 true r0
        lod 255 false r0
        inv r0
        lod 10 false r0
        lod 10 false r1
        add r0 r1 r2";
    let insts: Vec<Instruction> = instruction
        .split('\n')
        .map(|x| parse_line(x).unwrap())
        .collect();
    println!("{:?}", insts);

    let target_insts: Vec<Instruction> = vec![
        Lod(255, true, 0),
        Lod(255, false, 0),
        Inv(0),
        Lod(10, false, 0),
        Lod(10, false, 1),
        Add(0, 1, 2),
    ];

    assert!(insts == target_insts);

    for inst in insts.into_iter() {
        cpu.execute_instruction(inst);
    }

    let regs = cpu.registers.clone();

    use crate::cpu::word::Word;
    let target_regs: [Word; 8] = [
        Word::new(10),
        Word::new(10),
        Word::new(20),
        Word::new(0),
        Word::new(0),
        Word::new(0),
        Word::new(0),
        Word::new(0),
    ];

    assert!(regs == target_regs);
}
