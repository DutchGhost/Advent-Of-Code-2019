static PUZZLE: &'static str = include_str!(r"..\..\..\Inputs\day09.txt");

use intcode::{
    machine::Machine,
    future::{
        FutureExt,
        stream::{once},
        sink::{Stdout},
    }
};

fn parse_input(s: &str) -> Vec<isize> {
    s.split(",").map(|w| w.parse().unwrap()).collect::<Vec<_>>()
}

fn part1(v: Vec<isize>) -> isize {
    let mut stdout = Stdout::new();
    let mut machine = Machine::new(v, once(1), &mut stdout);
    let _ = machine.execute();

    stdout.into_inner().unwrap()
}

fn part2(v: Vec<isize>) -> isize {
    let mut stdout = Stdout::new();
    let mut machine = Machine::new(v, once(2), &mut stdout);
    let _ = machine.execute();

    stdout.into_inner().unwrap()
}

fn main() {
    let mut v = parse_input(PUZZLE);

    v.extend_from_slice(&[0; 4096]);

    let p1 = part1(v.clone());
    let p2 = part2(v);

    println!("Part 1: {}\nPart 2: {}", p1, p2);
}
