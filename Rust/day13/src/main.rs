static PUZZLE: &'static str = include_str!(r"..\..\..\Inputs\day13.txt");

use intcode::{
    future::{
        sink::Sink,
        Poll,
        stream::{once},
        FutureExt,
    },
    machine::Machine,
};

#[derive(Eq, Ord, PartialEq, PartialOrd, Clone, Hash, Copy)]
enum Expecting {
    X,
    Y,
    Id,
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
            n => panic!("Not a valid tile!"),
        }
    }
}

use std::collections::HashMap;

struct Drawer {
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
            },

            Expecting::Y => {
                self.y = value;
                self.expects = Expecting::Id;
                Ok(())
            },

            Expecting::Id => {
                let tile = Tile::from(value);

                *self.map.entry((self.x, self.y)).or_insert(tile) = tile;
                self.expects = Expecting::X;
                Ok(())
            }
        }
    }
}

fn parse_input(s: &str) -> Vec<isize> {
    s.split(",").map(|w| w.parse().unwrap()).collect::<Vec<_>>()
}

fn main() {
    let mut program = parse_input(PUZZLE);
    program.extend_from_slice(&[0; 512]);
    let mut drawer = Drawer::new();
    let mut machine = Machine::new(program, once(1), &mut drawer);

    dbg!(machine.execute());

    dbg!(drawer.map.iter().filter(|(_, v)| **v == Tile::Block).count());

}
