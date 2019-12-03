static PUZZLE: &'static str = include_str!(r"..\..\..\Inputs\day03.txt");

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
enum Point {
    Horizontal { x: isize, dx: isize, y: isize },
    Vertical { x: isize, y: isize, dy: isize },
}

impl Point {
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

    fn intersect<'a>(&'a self, list: &'a [Self]) -> impl Iterator<Item = isize> + 'a {
        list.iter()
            .filter(move |item| match item {
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

fn make_map<'a>(
    x: &'a mut isize,
    y: &'a mut isize,
    iter: impl Iterator<Item = Instruction> + 'a,
) -> impl Iterator<Item = Point> + 'a {
    iter.map(move |ins| {
        let (old_x, old_y) = (*x, *y);
        match ins {
            Instruction::Down(n) => {
                *y -= n as isize;
                Point::Vertical {
                    x: old_x,
                    y: old_y,
                    dy: *y,
                }
            }
            Instruction::Up(n) => {
                *y += n as isize;
                Point::Vertical {
                    x: old_x,
                    y: old_y,
                    dy: *y,
                }
            }
            Instruction::Right(n) => {
                *x += n as isize;
                Point::Horizontal {
                    y: *y,
                    x: old_x,
                    dx: *x,
                }
            }
            Instruction::Left(n) => {
                *x -= n as isize;
                Point::Horizontal {
                    y: *y,
                    x: old_x,
                    dx: *x,
                }
            }
        }
    })
}

fn main() {
    let (first, second) = parse(PUZZLE);

    let (mut x, mut y) = (0, 0);
    let first_map = make_map(&mut x, &mut y, first).collect::<Vec<_>>();

    let (mut x, mut y) = (0, 0);
    let p1 = make_map(&mut x, &mut y, second)
        .filter_map(move |elem| elem.intersect(&first_map).min())
        .min();

    println!("Part 1:  {:?}", p1);
}
