use super::intcode::{Intcode, Poll};
use super::Machine;

use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
pub enum Color {
    Black,
    White,
}

impl Color {
    fn from_int(n: isize) -> Self {
        match n {
            0 => Self::Black,
            1 => Self::White,
            n => panic!(format!("Invalid color: {}", n)),
        }
    }

    fn to_int(self) -> isize {
        match self {
            Self::Black => 0,
            Self::White => 1,
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_int(n: isize) -> Self {
        match n {
            0 => Self::Left,
            1 => Self::Right,
            n => panic!(format!("Invalid direction: {}", n)),
        }
    }

    fn turn_left(&mut self) {
        *self = match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }

    fn turn_right(&mut self) {
        *self = match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

enum Output {
    Paint,
    Turn,
}

#[derive(Debug)]
pub struct Cell {
    pub color: Color,
    pub visits: usize,
}

impl Cell {
    pub fn new(color: Color) -> Self {
        Self { color, visits: 0 }
    }
}

pub struct Robot {
    brain: Machine,
    facing: Direction,
    expecting: Output,
    x: isize,
    y: isize,
    pub visited: HashMap<(isize, isize), Cell>,
}

impl Robot {
    pub fn new(program: Vec<isize>, visited: HashMap<(isize, isize), Cell>) -> Self {
        Self {
            brain: Machine::new(program),
            facing: Direction::Up,
            expecting: Output::Paint,
            x: 0,
            y: 0,
            visited,
        }
    }

    pub fn step(&mut self) -> bool {
        let current_color = self
            .visited
            .get(&(self.x, self.y))
            .map(|cell| cell.color)
            .unwrap_or(Color::Black);

        let s = self.brain.step();
        //dbg!(&s);
        match s {
            Poll::Running => {}
            Poll::Output(o) => {
                match self.expecting {
                    Output::Paint => {
                        let color = Color::from_int(o);
                        let mut entry = self.visited.entry((self.x, self.y)).or_insert(Cell {
                            color: Color::Black,
                            visits: 0,
                        });

                        entry.color = color;
                        entry.visits += 1;
                        self.expecting = Output::Turn;
                    }
                    Output::Turn => {
                        let direction = Direction::from_int(o);

                        match direction {
                            Direction::Left => self.facing.turn_left(),
                            Direction::Right => self.facing.turn_right(),
                            _ => panic!("No up or down possible as a turn"),
                        }

                        match self.facing {
                            Direction::Up => self.y += 1,
                            Direction::Down => self.y -= 1,
                            Direction::Left => self.x -= 1,
                            Direction::Right => self.x += 1,
                        }
                        //self.visited.entry((self.x, self.y)).or_insert(Cell { color: Color::Black, visits: 1});
                        self.expecting = Output::Paint
                    }
                }
            }
            Poll::Input(i) => *i = current_color.to_int(),
            Poll::Exit => return true,
        }

        return false;
    }
}
