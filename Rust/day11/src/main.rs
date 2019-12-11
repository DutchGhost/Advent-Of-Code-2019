static PUZZLE: &'static str = include_str!(r"..\..\..\Inputs\day11.txt");

mod intcode;
use intcode::Machine;

mod robot;
use robot::Robot;

fn parse_input(s: &str) -> Vec<isize> {
    s.split(",").map(|w| w.parse().unwrap()).collect::<Vec<_>>()
}

fn main() {
    //println!("Hello, world!");

    let mut input = parse_input(PUZZLE);
    input.extend_from_slice(&[0; 1024]);
    let mut machine = Robot::new(input);

    while !machine.step() {}

    //dbg!(machine.visited);
    dbg!(machine
        .visited
        .into_iter()
        .map(|(k, v)| v.visits)
        .filter(|v| *v >= 1)
        .count());
}
