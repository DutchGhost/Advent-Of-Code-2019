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
    fn from(mode: isize) -> Self {
        [Self::Position, Self::Immediate][mode as usize]
    }
}

#[derive(Eq, PartialEq)]
enum Opcode {
    Add,
    Mul,
    Save,
    Output,
    Halt,
}

impl Opcode {
    fn operands(&self) -> usize {
        match self {
            Self::Add => 3,
            Self::Mul => 3,
            Self::Save => 1,
            Self::Output => 1,
            Self::Halt => 1,
        }
    }
}
impl From<isize> for Opcode {
    fn from(n: isize) -> Self {
        match n {
            1 => Self::Add,
            2 => Self::Mul,
            3 => Self::Save,
            4 => Self::Output,
            99 => Self::Halt,
            n => panic!(format!("Invalid opcode: {}", n)),
        }
    }
}

fn parse(n: isize) -> ([ParameterMode; 3], Opcode) {
    (
        [
            ParameterMode::from(n / 10_000 % 10),
            ParameterMode::from(n / 1000 % 10),
            ParameterMode::from(n / 100 % 10),
        ],
        Opcode::from(n % 100),
    )
}

fn main() {
    let mut program = parse_input(PUZZLE);
    let mut ip = 0;
    let mut exit = 0;

    loop {
        let (modes, opcode) = parse(program[ip]);
        match opcode {
            Opcode::Add => {
                let lhs = match modes[2] {
                    ParameterMode::Immediate => program[ip + 1],
                    ParameterMode::Position => program[program[ip + 1] as usize],
                };

                let rhs = match modes[1] {
                    ParameterMode::Immediate => program[ip + 2],
                    ParameterMode::Position => program[program[ip + 2] as usize],
                };

                let result_place = program[ip + 3];

                program[result_place as usize] = lhs + rhs;
            }

            Opcode::Mul => {
                let lhs = match modes[2] {
                    ParameterMode::Immediate => program[ip + 1],
                    ParameterMode::Position => program[program[ip + 1] as usize],
                };

                let rhs = match modes[1] {
                    ParameterMode::Immediate => program[ip + 2],
                    ParameterMode::Position => program[program[ip + 2] as usize],
                };

                let result_place = program[ip + 3];

                program[result_place as usize] = lhs * rhs;
            }
            Opcode::Save => {
                let place = program[ip + 1];
                program[place as usize] = 1;
            }
            Opcode::Output => {
                exit = match modes[2] {
                    ParameterMode::Immediate => program[ip + 1],
                    ParameterMode::Position => program[program[ip + 1] as usize],
                };
            }
            Opcode::Halt => break,
        }

        ip += opcode.operands() + 1;
    }

    println!("{}", exit);
}
