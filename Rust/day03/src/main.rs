static PUZZLE: &'static str = include_str!(r"..\..\..\Inputs\day03.txt");

use std::collections::hash_map::{Entry, HashMap};

#[derive(Debug)]
enum Instruction {
    Right(usize),
    Down(usize),
    Left(usize),
    Up(usize),
}

fn parse<'a>(
    s: &'a str,
) -> (
    impl Iterator<Item = Instruction> + 'a,
    impl Iterator<Item = Instruction> + 'a,
) {
    let mut lines = s.lines();

    let first = lines.next().unwrap();

    let mapfn = |ins: &str| {
        let num = ins[1..].parse::<usize>().unwrap();
        match ins.chars().next().unwrap() {
            'U' => Instruction::Up(num),
            'D' => Instruction::Down(num),
            'L' => Instruction::Left(num),
            'R' => Instruction::Right(num),
            _ => panic!("INVALID INSTRUCTION"),
        }
    };

    let first = first.split(",").map(mapfn);

    let second = lines.next().unwrap();
    let second = second.split(",").map(mapfn);

    (first, second)
}

#[derive(Copy, Clone, Debug)]
enum Segment {
    Horizontal { x: isize, dx: isize, y: isize },
    Vertical { x: isize, y: isize, dy: isize },
}

impl Segment {
    fn contains_x(&self, check_x: isize) -> bool {
        match self {
            Self::Horizontal { x, dx, .. } => {
                let (x, dx) = sort(x, dx);

                *x <= check_x && check_x <= *dx
            }
            Self::Vertical { x, .. } => *x == check_x,
        }
    }

    fn contains_y(&self, check_y: isize) -> bool {
        match self {
            Self::Horizontal { y, .. } => *y == check_y,
            Self::Vertical { y, dy, .. } => {
                let (y, dy) = sort(y, dy);

                *y <= check_y && check_y <= *dy
            }
        }
    }

    fn intersect<'a>(
        &'a self,
        iter: impl Iterator<Item = &'a Self> + 'a,
    ) -> impl Iterator<Item = isize> + 'a {
        iter.filter(move |item| match item {
            Self::Horizontal { y, .. } => match self {
                Self::Vertical { x, .. } => self.contains_y(*y) && item.contains_x(*x),
                _ => false,
            },
            Self::Vertical { x, .. } => match self {
                Self::Horizontal { y, .. } => self.contains_x(*x) && item.contains_y(*y),
                _ => false,
            },
        })
        .map(move |item| match item {
            Self::Horizontal { y, .. } => match self {
                Self::Vertical { x, .. } => y.abs() + x.abs(),
                _ => unreachable!(),
            },
            Self::Vertical { x, .. } => match self {
                Self::Horizontal { y, .. } => y.abs() + x.abs(),
                _ => unreachable!(),
            },
        })
    }
}

fn sort<T: Ord>(t1: T, t2: T) -> (T, T) {
    if t1 < t2 {
        (t1, t2)
    } else {
        (t2, t1)
    }
}

fn segments<'a>(
    x: &'a mut isize,
    y: &'a mut isize,
    iter: impl Iterator<Item = Instruction> + 'a,
) -> impl Iterator<Item = Segment> + 'a {
    iter.map(move |ins| {
        let (old_x, old_y) = (*x, *y);
        match ins {
            Instruction::Down(n) => {
                *y -= n as isize;
                Segment::Vertical {
                    x: old_x,
                    y: old_y,
                    dy: *y,
                }
            }
            Instruction::Up(n) => {
                *y += n as isize;
                Segment::Vertical {
                    x: old_x,
                    y: old_y,
                    dy: *y,
                }
            }
            Instruction::Right(n) => {
                *x += n as isize;
                Segment::Horizontal {
                    y: *y,
                    x: old_x,
                    dx: *x,
                }
            }
            Instruction::Left(n) => {
                *x -= n as isize;
                Segment::Horizontal {
                    y: *y,
                    x: old_x,
                    dx: *x,
                }
            }
        }
    })
}

fn part1(s: &str) -> isize {
    let (first, second) = parse(s);

    let (mut x, mut y) = (0, 0);
    let first_map = segments(&mut x, &mut y, first).collect::<Vec<_>>();

    let (mut x, mut y) = (0, 0);
    segments(&mut x, &mut y, second)
        .filter_map(move |elem| elem.intersect(first_map.iter()).min())
        .min()
        .unwrap()
}

#[derive(Ord, PartialOrd, Hash, Clone, Copy, Eq, PartialEq)]
enum Line {
    First,
    Second,
}

fn setup_first_wire(
    map: &mut HashMap<(isize, isize), (usize, Line)>,
    iter: impl Iterator<Item = Instruction>,
) {
    let (mut cx, mut cy) = (0, 0);

    let mut first_total_steps = 0usize;

    for elem in iter {
        match elem {
            Instruction::Down(n) => {
                for _ in 0..n {
                    cy -= 1;
                    first_total_steps += 1;
                    map.insert((cx, cy), (first_total_steps, Line::First));
                }
            }
            Instruction::Up(n) => {
                for _ in 0..n {
                    cy += 1;
                    first_total_steps += 1;
                    map.insert((cx, cy), (first_total_steps, Line::First));
                }
            }
            Instruction::Left(n) => {
                for _ in 0..n {
                    cx -= 1;
                    first_total_steps += 1;
                    map.insert((cx, cy), (first_total_steps, Line::First));
                }
            }
            Instruction::Right(n) => {
                for _ in 0..n {
                    cx += 1;
                    first_total_steps += 1;
                    map.insert((cx, cy), (first_total_steps, Line::First));
                }
            }
        }
    }
}

fn inserter_fn(
    map: &mut HashMap<(isize, isize), (usize, Line)>,
    collided: &mut Vec<usize>,
    cx: isize,
    cy: isize,
    steps: usize,
) {
    match map.entry((cx, cy)) {
        Entry::Vacant(e) => {
            e.insert((steps, Line::Second));
        }
        Entry::Occupied(o) => {
            let (n, line) = o.get();

            if *line == Line::First {
                collided.push(n + steps);
            }
        }
    }
}

fn part2(s: &str) -> usize {
    let (first, second) = parse(s);

    let mut map = HashMap::new();

    setup_first_wire(&mut map, first);

    let mut collided = Vec::new();
    let mut second_total_steps = 0;

    let (mut cx, mut cy) = (0, 0);

    for elem in second {
        match elem {
            Instruction::Down(n) => {
                for _ in 0..n {
                    cy -= 1;
                    second_total_steps += 1;
                    inserter_fn(&mut map, &mut collided, cx, cy, second_total_steps);
                }
            }
            Instruction::Up(n) => {
                for _ in 0..n {
                    cy += 1;
                    second_total_steps += 1;
                    inserter_fn(&mut map, &mut collided, cx, cy, second_total_steps);
                }
            }
            Instruction::Left(n) => {
                for _ in 0..n {
                    cx -= 1;
                    second_total_steps += 1;
                    inserter_fn(&mut map, &mut collided, cx, cy, second_total_steps);
                }
            }
            Instruction::Right(n) => {
                for _ in 0..n {
                    cx += 1;
                    second_total_steps += 1;
                    inserter_fn(&mut map, &mut collided, cx, cy, second_total_steps);
                }
            }
        }
    }

    collided.into_iter().min().unwrap()
}

fn main() {
    let p1 = part1(PUZZLE);
    let p2 = part2(PUZZLE);
    println!("Part 1: {}\nPart 2: {}", p1, p2);
}
