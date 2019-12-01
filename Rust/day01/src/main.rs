static PUZZLE: &'static str = include_str!(r"..\..\..\Inputs\day01.txt");

mod fuel;
use fuel::Fuel;

use std::iter;

fn parse<'a>(input: &'a str) -> impl Iterator<Item = i64> + 'a {
    input.lines().map(|line| line.parse::<i64>().unwrap())
}

fn part1(s: &str) -> i64 {
    parse(s).map(|mut mass| mass.fuel().unwrap()).sum::<i64>()
}

fn part2(s: &str) -> i64 {
    parse(s)
        .flat_map(|mut mass| iter::from_fn(move || mass.fuel()))
        .sum()
}

fn main() {
    let p1 = part1(PUZZLE);
    let p2 = part2(PUZZLE);

    println!("Part 1: {}\nPart 2: {}", p1, p2);
}
