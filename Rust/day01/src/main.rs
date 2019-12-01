static PUZZLE: &'static str = include_str!("input.txt");

mod fuel;
use fuel::Fuel;

fn parse<'a>(input: &'a str) -> impl Iterator<Item = i64> + 'a {
    input.lines().map(|line| line.parse::<i64>().unwrap())
}

fn part1(s: &str) -> i64 {
    parse(s)
        .map(|mass| mass / 3)
        .map(|mass| mass - 2)
        .sum::<i64>()
}

fn part2(s: &str) -> i64 {
    parse(s).flat_map(|mass| mass.fuel()).sum()
}

fn main() {
    dbg!(part1(PUZZLE));
    dbg!(part2(PUZZLE));
}
