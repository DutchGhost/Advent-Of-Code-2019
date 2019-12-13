static PUZZLE: &'static str = include_str!(r"..\..\..\Inputs\day13.txt");

use std::{
    cell::Cell,
    io::{stdin, BufReader},
};

use intcode::{
    future::{stream, FutureExt},
    machine::Machine,
};

mod drawer;
use drawer::{Drawer, FancyDrawer};

mod joystick;
use joystick::JoyStick;

fn parse_input(s: &str) -> Vec<isize> {
    s.split(",").map(|w| w.parse().unwrap()).collect::<Vec<_>>()
}

fn part1(program: Vec<isize>) -> usize {
    let mut drawer = Drawer::new();
    let mut machine = Machine::new(program, stream::empty(), &mut drawer);

    machine.execute().expect("Machine failed to run!");

    drawer.blocks()
}

fn part2(mut program: Vec<isize>) -> isize {
    program[0] = 2;

    let should_display = Cell::new(false);
    let mut fancy_drawer = FancyDrawer::new(&should_display);

    let stdin = stdin();
    let stdin = stdin.lock();
    let bufreader = BufReader::new(stdin);
    let mut joystick = JoyStick::new(bufreader, &should_display);

    let mut machine = Machine::new(program, &mut joystick, &mut fancy_drawer);

    machine.execute().expect("Machine failed to run!");

    fancy_drawer.score()
}

fn main() {
    let mut program = parse_input(PUZZLE);
    program.extend_from_slice(&[0; 512]);

    let p1 = part1(program.clone());
    let p2 = part2(program);

    println!("Part 1: {}\nPart2: {}", p1, p2);
}
