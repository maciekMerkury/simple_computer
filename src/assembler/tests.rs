use super::parse_line;
use crate::cpu::instruction::Instruction::{self, *};

#[test]
fn parse_add() {
    let instruction = "add r0 r1 r2";
    let compiled = parse_line(instruction).unwrap();
    assert!(compiled == Add(0, 1, 2));
    assert!(parse_line("add r r2 r9").is_err());
}

#[test]
fn parse_inv() {
    let instruction = "inv r0";
    let compiled = parse_line(instruction).unwrap();
    assert!(compiled == Inv(0));
    assert!(parse_line("inv r8").is_err());
}

#[test]
fn parse_lod() {
    let instruction = "lod 255 false r0";
    let compiled = parse_line(instruction).unwrap();
    assert!(compiled == Lod(255, false, 0));
    assert!(parse_line("lod 400 fal r8").is_err());
}

#[test]
fn parse_jiz() {
    let instruction = "jiz r0 15";
    let compiled = parse_line(instruction).unwrap();
    assert!(compiled == Jiz(0, 15));
    assert!(parse_line("jiz r0 256").is_err());
}

