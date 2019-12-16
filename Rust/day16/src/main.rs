static PUZZLE: &'static str = include_str!(r"..\..\..\Inputs\day16.txt");

use std::iter;

fn parse(s: &str) -> Vec<i128> {
    s.chars().map(|s| (s as u8 - 48) as i128).collect()
}

fn make_pattern(cycle: usize) -> impl Iterator<Item = i128> {
    iter::repeat(0)
        .take(cycle)
        .chain(iter::repeat(1).take(cycle))
        .chain(iter::repeat(0).take(cycle))
        .chain(iter::repeat(-1).take(cycle))
        .cycle()
        .skip(1)
}

fn part1(v: &[i128]) -> String {
    let mut input: Vec<_> = v.to_owned();

    for _ in 1..=100 {
        let new = input
            .iter()
            .enumerate()
            .map(|(idx, _)| {
                (input
                    .iter()
                    .zip(make_pattern(idx + 1))
                    .map(|(elem, multiplier)| elem * multiplier)
                    .sum::<i128>()
                    % 10)
                    .abs()
            })
            .collect::<Vec<i128>>();

        input = new;
    }

    input.into_iter().take(8).map(|n| n.to_string()).collect()
}

fn part2(s: &str) -> Vec<isize> {
    let input_chars: Vec<_> = s.chars().collect();

    let offset = input_chars
        .iter()
        .take(7)
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    let mut digits: Vec<i64> = input_chars
        .iter()
        .map(|c| c.to_string().parse::<i64>().unwrap())
        .cycle()
        .take(10000 * input_chars.len())
        .skip(offset)
        .collect();

    for _ in 0..100 {
        for i in (0..digits.len() - 1).rev() {
            digits[i] = (digits[i] + digits[i + 1]) % 10;
        }
    }

    println!("part2:{:?}", &digits[0..8]);

    vec![]
}
fn main() {
    let mut v = parse(PUZZLE);
    let p1 = part1(&v);
    let p2 = part2(PUZZLE);

    println!("Part 1: {}\nPart 2: {:?}", p1, p2);
}
