use std::fmt;

#[derive(Debug, PartialEq)]
enum OpCode {
    Add,
    Multiply,
    Exit,
    Unknown,
}

impl From<i64> for OpCode {
    fn from(i: i64) -> OpCode {
        match i {
            1 => OpCode::Add,
            2 => OpCode::Multiply,
            99 => OpCode::Exit,
            _ => OpCode::Unknown,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Error {
    ProgramTooShort,
    NotEnoughParams(OpCode),
    SegFault,
    UnknownOpCode(i64),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ProgramTooShort => write!(f, "Program is too short"),
            Error::NotEnoughParams(op) => write!(f, "Not enough parameters for {:?} operation", op),
            Error::SegFault => write!(f, "Segmentation fault"),
            Error::UnknownOpCode(op_code) => write!(f, "Unknown op code: {}", op_code),
        }
    }
}

type Program = Vec<i64>;

pub fn part1(input: Program) {
    match set_input(input, 12, 2).and_then(run_intcode) {
        Ok(mem) => println!("Output is: {}", mem[0]),
        Err(e) => println!("{}", e),
    }
}

const NOUN_INDEX: usize = 1;
const VERB_INDEX: usize = 2;

fn set_input(mut prog: Program, noun: i64, verb: i64) -> Result<Program, Error> {
    if prog.len() < 3 {
        return Err(Error::ProgramTooShort);
    }

    prog[NOUN_INDEX] = noun;
    prog[VERB_INDEX] = verb;

    Ok(prog)
}

fn run_intcode(mut prog: Program) -> Result<Program, Error> {
    let len = prog.len();
    if len < 1 {
        return Err(Error::ProgramTooShort);
    }

    let mut i = 0;

    while i < len {
        match OpCode::from(prog[i]) {
            OpCode::Add => {
                if (i + 3) >= len {
                    return Err(Error::NotEnoughParams(OpCode::Add));
                }
                let src1 = prog[i + 1] as usize;
                let src2 = prog[i + 2] as usize;
                let dest = prog[i + 3] as usize;
                if src1 >= len || src2 >= len || dest >= len {
                    return Err(Error::SegFault);
                }
                prog[dest] = prog[src1] + prog[src2];
            }
            OpCode::Multiply => {
                if (i + 3) >= len {
                    return Err(Error::NotEnoughParams(OpCode::Multiply));
                }
                let src1 = prog[i + 1] as usize;
                let src2 = prog[i + 2] as usize;
                let dest = prog[i + 3] as usize;
                if src1 >= len || src2 >= len || dest >= len {
                    return Err(Error::SegFault);
                }
                prog[dest] = prog[src1] * prog[src2];
            }
            OpCode::Exit => return Ok(prog),
            OpCode::Unknown => return Err(Error::UnknownOpCode(prog[i])),
        }

        i = i + 4;
    }

    Ok(prog)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_input() {
        assert_eq!(Err(Error::ProgramTooShort), set_input(vec![1, 2], 12, 2));
        assert_eq!(Ok(vec![1, 12, 2]), set_input(vec![1, 2, 3], 12, 2));
        assert_eq!(Ok(vec![1, 12, 2, 4]), set_input(vec![1, 2, 3, 4], 12, 2));
    }

    #[test]
    fn test_run_intcode() {
        assert_eq!(Err(Error::ProgramTooShort), run_intcode(vec![]));
        assert_eq!(
            Err(Error::UnknownOpCode(100)),
            run_intcode(vec![1, 0, 0, 0, 100])
        );
        assert_eq!(Err(Error::SegFault), run_intcode(vec![1, 0, 0, 5, 99]));
        assert_eq!(
            Err(Error::NotEnoughParams(OpCode::Add)),
            run_intcode(vec![1, 1, 1])
        );
        assert_eq!(
            Err(Error::NotEnoughParams(OpCode::Multiply)),
            run_intcode(vec![2, 1, 1])
        );
        assert_eq!(Ok(vec![2, 0, 0, 0]), run_intcode(vec![1, 0, 0, 0]));
        assert_eq!(
            Ok(vec![1, 1, 5, 4, 99, 98]),
            run_intcode(vec![1, 1, 5, 4, 0, 98])
        );
        assert_eq!(
            Ok(vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]),
            run_intcode(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50])
        );
        assert_eq!(Ok(vec![2, 0, 0, 0, 99]), run_intcode(vec![1, 0, 0, 0, 99]));
        assert_eq!(Ok(vec![2, 3, 0, 6, 99]), run_intcode(vec![2, 3, 0, 3, 99]));
        assert_eq!(
            Ok(vec![2, 4, 4, 5, 99, 9801]),
            run_intcode(vec![2, 4, 4, 5, 99, 0])
        );
        assert_eq!(
            Ok(vec![30, 1, 1, 4, 2, 5, 6, 0, 99]),
            run_intcode(vec![1, 1, 1, 4, 99, 5, 6, 0, 99])
        );
    }
}
