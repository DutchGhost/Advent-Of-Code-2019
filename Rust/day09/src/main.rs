static PUZZLE: &'static str = include_str!(r"..\..\..\Inputs\day09.txt");

mod intcode;
use intcode::{Intcode, Machine};

fn parse_input(s: &str) -> Vec<isize> {
    s.split(",").map(|w| w.parse().unwrap()).collect::<Vec<_>>()
}

fn part1(v: Vec<isize>) -> isize {
    Machine::new(v).run(std::iter::once(1)).unwrap()
}

fn part2(v: Vec<isize>) -> isize {
    Machine::new(v).run(std::iter::once(2)).unwrap()
}

fn main() {
    let mut v = parse_input(PUZZLE);

    v.extend_from_slice(&[0; 4096]);

    let p1 = part1(v.clone());
    let p2 = part2(v);

    println!("Part 1: {}\nPart 2: {}", p1, p2);
}
