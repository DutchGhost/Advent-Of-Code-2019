use intcode::future::{sink::Sink, Poll};

use std::cell::Cell;

#[derive(Eq, Ord, PartialEq, PartialOrd, Clone, Hash, Copy)]
enum Expecting {
    X,
    Y,
    Id,
    ScoreBoard,
}

#[derive(Eq, Ord, PartialEq, PartialOrd, Clone, Hash, Copy)]
enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

impl From<isize> for Tile {
    fn from(n: isize) -> Self {
        match n {
            0 => Self::Empty,
            1 => Self::Wall,
            2 => Self::Block,
            3 => Self::HorizontalPaddle,
            4 => Self::Ball,
            n => panic!(format!("Not a valid tile: {} !", n)),
        }
    }
}

use std::collections::HashMap;

pub struct Drawer {
    map: HashMap<(isize, isize), Tile>,

    x: isize,
    y: isize,
    expects: Expecting,
}

impl Drawer {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            x: 0,
            y: 0,
            expects: Expecting::X,
        }
    }

    pub fn blocks(&self) -> usize {
        self.map.iter().filter(|(_, v)| **v == Tile::Block).count()
    }
}

impl Sink<isize> for Drawer {
    type Error = ();

    fn poll_ready(&mut self) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn send(&mut self, value: isize) -> Result<(), Self::Error> {
        match self.expects {
            Expecting::X => {
                self.x = value;
                self.expects = Expecting::Y;
                Ok(())
            }

            Expecting::Y => {
                self.y = value;
                self.expects = Expecting::Id;
                Ok(())
            }

            Expecting::Id => {
                let tile = Tile::from(value);

                *self.map.entry((self.x, self.y)).or_insert(tile) = tile;
                self.expects = Expecting::X;
                Ok(())
            }
            _ => unreachable!(),
        }
    }
}

const WIDTH: usize = 40;
const HEIGHT: usize = 40;

pub struct Grid {
    grid: [[Tile; WIDTH]; HEIGHT],
}

impl Grid {
    fn new() -> Self {
        Self {
            grid: [[Tile::Empty; WIDTH]; HEIGHT],
        }
    }
}

impl Grid {
    fn display(&self) {
        for row in self.grid.iter() {
            for cell in row.iter() {
                match cell {
                    Tile::Block => print!("b"),
                    Tile::Empty => print!(" "),
                    Tile::Wall => print!("w"),
                    Tile::HorizontalPaddle => print!("_"),
                    Tile::Ball => print!("o"),
                };
            }
            println!()
        }
    }
}

pub struct FancyDrawer<'a> {
    grid: Grid,
    x: isize,
    y: isize,
    expects: Expecting,
    should_display: &'a Cell<bool>,
    score: isize,
}

impl<'a> FancyDrawer<'a> {
    pub fn new(should_display: &'a Cell<bool>) -> Self {
        Self {
            grid: Grid::new(),
            x: 0,
            y: 0,
            expects: Expecting::X,
            should_display,
            score: 0,
        }
    }

    pub fn score(&self) -> isize {
        self.score
    }
}

impl<'a> Sink<isize> for FancyDrawer<'a> {
    type Error = ();

    fn poll_ready(&mut self) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn send(&mut self, value: isize) -> Result<(), Self::Error> {
        match self.expects {
            Expecting::X => {
                self.x = value;
                self.expects = Expecting::Y;
                Ok(())
            }
            Expecting::Y => {
                self.y = value;

                match (self.x, self.y) {
                    (-1, 0) => self.expects = Expecting::ScoreBoard,
                    _ => self.expects = Expecting::Id,
                }

                Ok(())
            }
            Expecting::Id => {
                let tile = Tile::from(value);
                self.grid.grid[self.y as usize][self.x as usize] = tile;
                match tile {
                    Tile::HorizontalPaddle | Tile::Ball => {
                        if self.should_display.get() {
                            self.grid.display()
                        }
                    }
                    _ => {}
                }
                self.expects = Expecting::X;
                Ok(())
            }
            Expecting::ScoreBoard => {
                self.score = value;
                self.expects = Expecting::X;
                Ok(())
            }
        }
    }
}
