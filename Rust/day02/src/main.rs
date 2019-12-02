static PUZZLE: &'static str = include_str!(r"..\..\..\Inputs\day02.txt");

fn parse(s: &str) -> Vec<usize> {
    s.split(",")
        .map(|w| w.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

#[derive(Eq, PartialEq)]
enum Opcode {
    Add,
    Mul,
    Halt,
}

impl From<usize> for Opcode {
    fn from(n: usize) -> Self {
        match n {
            1 => Self::Add,
            2 => Self::Mul,
            99 => Self::Halt,
            _ => panic!(format!("Invalid opcode: {}", n)),
        }
    }
}

fn run(program: &mut [usize], noun: usize, verb: usize) -> usize {
    let mut ip = 0;
    program[1] = noun;
    program[2] = verb;

    loop {
        let opcode = Opcode::from(program[ip]);
        let (lhs_place, rhs_place, result_place) = match opcode {
            Opcode::Halt => break,
            Opcode::Add | Opcode::Mul => (program[ip + 1], program[ip + 2], program[ip + 3]),
        };

        let result = match opcode {
            Opcode::Add => program[lhs_place] + program[rhs_place],
            Opcode::Mul => program[lhs_place] * program[rhs_place],
            Opcode::Halt => unreachable!(),
        };

        program[result_place] = result;
        ip += 4;
    }

    program[0]
}

fn part1(s: &str) -> usize {
    run(&mut parse(s), 12, 2)
}

fn part2(s: &str) -> usize {
    const MAGIC_NUMBER: usize = 19690720;
    let program = parse(s);

    let mut clone = Vec::with_capacity(program.len());

    (0..=99)
        .flat_map(|noun| (0..=99).map(move |verb| (noun, verb)))
        .find(|(noun, verb)| {
            clone.clear();
            clone.extend_from_slice(&program);

            run(&mut clone, *noun, *verb) == MAGIC_NUMBER
        })
        .map(|(noun, verb)| 100 * noun + verb)
        .unwrap()
}

fn main() {
    let p1 = part1(PUZZLE);
    let p2 = part2(PUZZLE);

    println!("Part 1: {}\nPart 2: {}", p1, p2);
}
