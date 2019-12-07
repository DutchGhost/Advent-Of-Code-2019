static PUZZLE: &'static str = include_str!(r"..\..\..\Inputs\day07.txt");

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

enum ExecuteResult<'a> {
    Continue,
    Output(isize),
    Exit,
    Input(&'a mut isize),
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

    fn execute<'a>(&self, ip: &mut isize, program: &'a mut [isize]) -> ExecuteResult<'a> {
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
                result = ExecuteResult::Input(&mut program[write_place]);
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

fn run_program(mut program: Vec<isize>, setting: isize, input_signal: isize) -> isize {
    let mut ip = 0;
    let mut exit = 0;

    let inputs = [setting, input_signal];
    let mut inputsidx = 0;

    loop {
        let opcode = Opcode::from(program[ip as usize] % 100);
        match opcode.execute(&mut ip, &mut program) {
            ExecuteResult::Continue => {}
            ExecuteResult::Output(result) => exit = result,
            ExecuteResult::Input(i) => {
                *i = inputs[inputsidx];
                inputsidx += 1;
            }
            ExecuteResult::Exit => break,
        }
    }
    exit
}

fn run_settings(program: Vec<isize>, settings: &[isize]) -> isize {
    let mut signal = 0;

    for setting in settings {
        signal = run_program(program.clone(), *setting, signal);
    }

    signal
}

macro_rules! multi_for {
    ([$name:ident $(,$names:ident)*] in [$e:expr $(,$es:expr)*] $b:block) => {
        for $name in $e {
            multi_for!([$($names),*] in [$($es),*] $b);
        }
    };

    ([] in [] $b:block) => {$b}
}

macro_rules! filter {
    ($name:ident $(,$names:ident)*) => {
        false $(|| $name == $names)* || filter!($($names),*)
    };
    () => {false}
}

fn part1(s: &str) -> isize {
    let program = parse_input(s);

    let mut highest = 0;
    multi_for! {
        [a, b, c, d, e] in [0..=4, 0..=4, 0..=4, 0..=4, 0..=4] {
            if filter!(a, b, c, d, e) { continue }
            let output = run_settings(program.clone(), &[a, b, c, d, e]);
            if output > highest {

                highest = output;
            }
        }
    }

    highest
}

fn main() {
    let p1 = part1(PUZZLE);
    println!("Part 1: {}", p1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_me() {
        let out = run_settings(
            vec![
                3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
            ],
            &[4, 3, 2, 1, 0],
        );

        assert_eq!(out, 43210);
    }
}
