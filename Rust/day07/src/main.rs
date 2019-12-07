static PUZZLE: &'static str = include_str!(r"..\..\..\Inputs\day07.txt");

fn parse_input(s: &str) -> Vec<isize> {
    s.split(",").map(|w| w.parse().unwrap()).collect::<Vec<_>>()
}

mod intcode;
use intcode::{Machine, Intcode, Poll};

fn run_program(program: Vec<isize>, setting: isize, input_signal: isize) -> isize {

    let mut exit = 0;
    let mut machine = Machine::new(program);
    
    let inputs = [setting, input_signal];
    let mut inputs_idx = 0;
    loop {
        match machine.step() {
            Poll::Running => {}
            Poll::Output(result) => exit = result,
            Poll::Input(i) => {
                *i = inputs[inputs_idx];
                inputs_idx += 1;
            }
            Poll::Exit => break,
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
