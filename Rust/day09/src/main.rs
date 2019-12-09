static PUZZLE: &'static str = include_str!(r"..\..\..\Inputs\day09.txt");

mod intcode;
use intcode::*;

fn parse_input(s: &str) -> Vec<isize> {
    s.split(",").map(|w| w.parse().unwrap()).collect::<Vec<_>>()
}

fn main() {
    let mut v = parse_input(PUZZLE);

    v.extend_from_slice(&[0; 4096]);

    let mut machine = Machine::new(v);

    dbg!(machine.run(std::iter::once(1)));

    println!("{:?}", machine.memory());
}
