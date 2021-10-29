#[cfg(test)]
mod tests;

use crate::cpu::instruction::Instruction;

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnknownFirstToken(String),
    NotEnoughArguments(String),
    IncorrectArgument(String),
    ArgumentParseError(String, String),
}

pub fn parse_line(line: &str) -> Result<Instruction, ParseError> {
    // parse by spaces:
    // TODO: moving away from ascii_whitespace could be a good idea, but this should work for now
    let things: Vec<&str> = line.split_ascii_whitespace().into_iter().collect();
    println!("parse_line things: {:?}", things);

    // this is not the best things i've written in my life
    match things[0] {
        "add" => {
            if things.len() < 4 {
                return Err(ParseError::NotEnoughArguments(line.into()));
            }
            for i in 1..4 {
                if things[i].len() != 2 {
                    return Err(ParseError::IncorrectArgument(line.into()));
                }
            }

            let mut regs: [usize; 3] = [0; 3];

            for i in 1..4 {
                match things[i][1..2].parse::<usize>() {
                    Ok(ri) => {
                        if ri > 6 {
                            return Err(ParseError::ArgumentParseError(line.into(), things[i].into()));
                        }
                        regs[i - 1] = ri;
                    },
                    Err(e) => {return Err(ParseError::ArgumentParseError(line.into(), e.to_string()))},
                }
            }

            return Ok(Instruction::Add(regs[0], regs[1], regs[2]));
        },
        "inv" => {
            if things.len() < 2 {
                return Err(ParseError::NotEnoughArguments(line.into()));
            }
            if things[1].len() != 2 {
                return Err(ParseError::IncorrectArgument(line.into()));
            }

            match things[1][1..2].parse::<usize>() {
                Ok(ri) => {
                if ri > 6 {
                    return Err(ParseError::ArgumentParseError(line.into(), things[1].into()));
                }
                    return Ok(Instruction::Inv(ri));
                },
                Err(e) => {return Err(ParseError::ArgumentParseError(line.into(), e.to_string()))},
            }
        },
        "lod" => {
            if things.len() < 4 {
                return Err(ParseError::NotEnoughArguments(line.into()));
            }
            let mut regs: (u8, bool, usize) = Default::default();

            match things[1].parse::<u8>() {
                Ok(val) => {regs.0 = val},
                Err(e) => {return Err(ParseError::ArgumentParseError(line.into(), e.to_string()))},
            }
            match things[2].parse::<bool>() {
                Ok(val) => {regs.1 = val},
                Err(e) => {return Err(ParseError::ArgumentParseError(line.into(), e.to_string()))},
            }
            match things[3][1..2].parse::<usize>() {
                Ok(val) => {regs.2 = val},
                Err(e) => {return Err(ParseError::ArgumentParseError(line.into(), e.to_string()))},
            }

            return Ok(Instruction::Lod(regs.0, regs.1, regs.2));
        },
        "jiz" => {
            if things.len() < 3 {
                return Err(ParseError::NotEnoughArguments(line.into()));
            }
            let mut regs: (usize, u8) = Default::default();

            match things[1][1..2].parse::<usize>() {
                Ok(val) => {regs.0 = val},
                Err(e) => {return Err(ParseError::ArgumentParseError(line.into(), e.to_string()))},
            }
            match things[2].parse::<u8>() {
                Ok(val) => {regs.1 = val},
                Err(e) => {return Err(ParseError::ArgumentParseError(line.into(), e.to_string()))},
            }

            return Ok(Instruction::Jiz(regs.0, regs.1));
        },
        _ => {return Err(ParseError::UnknownFirstToken(line.into()))}
    }
}

