static PUZZLE: &'static str = include_str!(r"..\..\..\Inputs\day05.txt");

fn parse_input(s: &str) -> Vec<isize> {
    s.split(",").map(|w| w.parse().unwrap()).collect::<Vec<_>>()
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum ParameterMode {
    Position,
    Immediate,
}

impl ParameterMode {
    const fn from(mode: isize) -> Self {
        [Self::Position, Self::Immediate][mode as usize]
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Opcode {
    Add,
    Mul,
    Save,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}

enum ExecuteResult {
    Continue,
    Output(isize),
    Exit,
}

const fn modes_of(n: isize) -> [ParameterMode; 3] {
    [
        ParameterMode::from(n / 100 % 10),
        ParameterMode::from(n / 1000 % 10),
        ParameterMode::from(n / 10_000 % 10),
    ]
}

fn read_value(program: &[isize], value: isize, mode: ParameterMode) -> isize {
    match mode {
        ParameterMode::Immediate => value,
        ParameterMode::Position => {
            assert!(value > -1);
            program[value as usize]
        }
    }
}

impl Opcode {
    fn operands(&self) -> usize {
        match self {
            Self::Add => 3,
            Self::Mul => 3,
            Self::Save => 1,
            Self::Output => 1,
            Self::JumpIfTrue => 2,
            Self::JumpIfFalse => 2,
            Self::LessThan => 3,
            Self::Equals => 3,
            Self::Halt => 1,
        }
    }

    fn execute(&self, ip: &mut isize, program: &mut [isize], sysid: isize) -> ExecuteResult {
        let mut result = ExecuteResult::Continue;

        let _ip = *ip as usize;
        let modes = modes_of(program[_ip]);

        match self {
            Opcode::Add => {
                let lhs = program[_ip + 1];
                let rhs = program[_ip + 2];
                let write_place = program[_ip + 3] as usize;

                program[write_place] =
                    read_value(program, lhs, modes[0]) + read_value(program, rhs, modes[1]);
            }

            Opcode::Mul => {
                let lhs = program[_ip + 1];
                let rhs = program[_ip + 2];
                let write_place = program[_ip + 3] as usize;

                program[write_place] =
                    read_value(program, lhs, modes[0]) * read_value(program, rhs, modes[1]);
            }
            Opcode::Save => {
                let write_place = program[_ip + 1] as usize;
                program[write_place] = dbg!(sysid);
            }
            Opcode::Output => {
                result = ExecuteResult::Output(read_value(program, program[_ip + 1], modes[0]));
            }
            Opcode::JumpIfTrue => {
                let checkme = program[_ip + 1];

                if read_value(program, checkme, modes[0]) != 0 {
                    let jmp = program[_ip + 2];
                    let jmp = read_value(program, jmp, modes[1]);
                    *ip = jmp;
                    return ExecuteResult::Continue;
                }
            }
            Opcode::JumpIfFalse => {
                let checkme = program[_ip + 1];

                if read_value(program, checkme, modes[0]) == 0 {
                    let jmp = program[_ip + 2];
                    let jmp = read_value(program, jmp, modes[1]);
                    *ip = jmp;
                    return ExecuteResult::Continue;
                }
            }
            Opcode::LessThan => {
                let lhs = program[_ip + 1];
                let rhs = program[_ip + 2];
                let write_place = program[_ip + 3] as usize;

                if read_value(program, lhs, modes[0]) < read_value(program, rhs, modes[1]) {
                    program[write_place] = 1;
                } else {
                    program[write_place] = 0;
                }
            }
            Opcode::Equals => {
                let lhs = program[_ip + 1];
                let rhs = program[_ip + 2];
                let write_place = program[_ip + 3] as usize;

                if read_value(program, lhs, modes[0]) == read_value(program, rhs, modes[1]) {
                    program[write_place] = 1;
                } else {
                    program[write_place] = 0;
                }
            }
            Opcode::Halt => result = ExecuteResult::Exit,
        }

        *ip += self.operands() as isize + 1;

        result
    }
}

impl From<isize> for Opcode {
    fn from(n: isize) -> Self {
        match n {
            1 => Self::Add,
            2 => Self::Mul,
            3 => Self::Save,
            4 => Self::Output,
            5 => Self::JumpIfTrue,
            6 => Self::JumpIfFalse,
            7 => Self::LessThan,
            8 => Self::Equals,
            99 => Self::Halt,
            n => panic!(format!("Invalid opcode: {}", n)),
        }
    }
}

fn part1(s: &str) -> isize {
    let mut program = parse_input(s);
    let mut ip = 0;

    let mut exit = 0;

    loop {
        let opcode = Opcode::from(program[ip as usize] % 100);

        match opcode.execute(&mut ip, &mut program, 1) {
            ExecuteResult::Continue => {}
            ExecuteResult::Output(result) => exit = result,
            ExecuteResult::Exit => break,
        }
    }

    exit
}

fn part2(s: &str) -> isize {
    let mut program = parse_input(s);
    let mut ip = 0;
    let mut exit = 0;

    loop {
        let opcode = Opcode::from(program[ip as usize] % 100);
        match opcode.execute(&mut ip, &mut program, 5) {
            ExecuteResult::Continue => {}
            ExecuteResult::Output(result) => exit = result,
            ExecuteResult::Exit => break,
        }
    }

    exit
}

fn main() {
    let p1 = part1(PUZZLE);
    let p2 = part2(PUZZLE);

    println!("Part 1: {}\nPart 2: {}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_modes() {
        assert_eq!(
            modes_of(1002),
            [
                ParameterMode::Position,
                ParameterMode::Immediate,
                ParameterMode::Position
            ]
        );
    }
}
