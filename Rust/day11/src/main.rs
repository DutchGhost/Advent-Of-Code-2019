static PUZZLE: &'static str = include_str!(r"..\..\..\Inputs\day11.txt");

mod intcode;
use intcode::Machine;

mod robot;
use robot::{Cell, Color, Robot};

use std::collections::HashMap;

fn parse_input(s: &str) -> Vec<isize> {
    s.split(",").map(|w| w.parse().unwrap()).collect::<Vec<_>>()
}

fn part1(program: Vec<isize>) -> usize {
    let mut machine = Robot::new(program, HashMap::new());

    while !machine.step() {}

    machine
        .visited
        .into_iter()
        .map(|(k, v)| v.visits)
        .filter(|v| *v >= 1)
        .count()
}

fn part2(program: Vec<isize>) {
    use std::cmp::max;

    let mut map = HashMap::new();
    map.insert((0, 0), Cell::new(Color::White));

    let mut machine = Robot::new(program, map);

    while !machine.step() {}

    let (max_x, max_y) = machine
        .visited
        .iter()
        .map(|((x, y), _)| (x, y))
        .fold((0, 0), |(old_x, old_y), (new_x, new_y)| {
            (max(old_x, *new_x), max(old_y, *new_y))
        });

    let offset = 30isize;
    let mut v =
        vec![vec![Color::Black; (max_x + offset * 2) as usize]; (max_y + offset * 2) as usize];

    for ((ref x, ref y), cell) in machine.visited.iter() {
        v[(*y + offset) as usize][(*x + offset) as usize] = cell.color;
    }

    v.reverse();

    for row in v.iter() {
        for cell in row.iter() {
            match cell {
                Color::Black => print!("#"),
                Color::White => print!(" "),
            }
        }
        println!()
    }
}

fn main() {
    let mut input = parse_input(PUZZLE);
    input.extend_from_slice(&[0; 512]);

    let p1 = part1(input.clone());
    part2(input);

    println!("Part 1: {}", p1);
}
