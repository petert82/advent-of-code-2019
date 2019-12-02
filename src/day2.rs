use std::fmt;

enum OpCode {
    Add,
    Multiply,
    Exit,
    Unknown,
}

#[derive(Debug, PartialEq)]
enum Error {
    InvalidProgram,
    UnknownOpCode(i64),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidProgram => write!(f, "Program is invalid"),
            Error::UnknownOpCode(op_code) => write!(f, "Unknown op code: {}", op_code),
        }
    }
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

type Program = Vec<i64>;

pub fn part1(input: Program) {
    match fix_program(input).and_then(run_intcode) {
        Ok(ints) => println!("Result is: {:?}", ints),
        Err(e) => println!("{}", e),
    }
}

fn fix_program(mut prog: Program) -> Result<Program, Error> {
    if prog.len() < 3 {
        return Err(Error::InvalidProgram);
    }

    prog[1] = 12;
    prog[2] = 2;

    Ok(prog)
}

fn run_intcode(mut prog: Program) -> Result<Program, Error> {
    let len = prog.len();
    if len < 1 {
        return Err(Error::InvalidProgram);
    }

    let mut i = 0;

    while i < len {
        match OpCode::from(prog[i]) {
            OpCode::Add => {
                if (i + 3) > len {
                    return Err(Error::InvalidProgram);
                }
                let src1 = prog[i + 1] as usize;
                let src2 = prog[i + 2] as usize;
                let dest = prog[i + 3] as usize;
                if src1 >= len || src2 >= len || dest >= len {
                    return Err(Error::InvalidProgram);
                }
                prog[dest] = prog[src1] + prog[src2];
            }
            OpCode::Multiply => {
                if (i + 3) > len {
                    return Err(Error::InvalidProgram);
                }
                let src1 = prog[i + 1] as usize;
                let src2 = prog[i + 2] as usize;
                let dest = prog[i + 3] as usize;
                if src1 >= len || src2 >= len || dest >= len {
                    return Err(Error::InvalidProgram);
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
    fn test_fix_program() {
        assert_eq!(Err(Error::InvalidProgram), fix_program(vec![1, 2]));
        assert_eq!(Ok(vec![1, 12, 2]), fix_program(vec![1, 2, 3]));
        assert_eq!(Ok(vec![1, 12, 2, 4]), fix_program(vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_run_intcode() {
        assert_eq!(
            Ok(vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]),
            run_intcode(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50])
        );
    }
}
