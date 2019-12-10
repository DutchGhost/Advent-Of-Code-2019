static PUZZLE: &'static str = include_str!(r"..\..\..\Inputs\day10.txt");

use std::collections::HashSet;

#[derive(Eq, Ord, PartialOrd, PartialEq)]
enum Astroid {
    Astroid,
    Empty,
}

impl std::fmt::Display for Astroid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Astroid => '#'.fmt(f),
            Self::Empty => '.'.fmt(f),
        }
    }
}

impl Astroid {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Astroid,
            c => panic!(format!("Invalid astroid!: {}", c)),
        }
    }
}

fn parse(s: &str) -> Vec<Vec<Astroid>> {
    s.lines()
        .map(|line| line.chars().map(Astroid::from).collect())
        .collect()
}

const TEST: &'static str = ".#..#
.....
#####
....#
...##";

fn fat_print(x: usize, y: usize, map: &[Vec<Astroid>]) {
    for (yy, line) in map.iter().enumerate() {
        for (xx, astroid) in line.iter().enumerate() {
            if xx == x && yy == y {
                print!("*");
            } else {
                print!("{}", astroid);
            }
        }
        println!()
    }
}

fn upper_left(cx: usize, cy: usize, map: &[Vec<Astroid>]) -> usize {
    let mut set = HashSet::new();
    let mut count = 0;
    for y in (0..cy).rev() {
        for x in (0..cx).rev() {
            if (cx != x && cy != y) {
                if (map[y][x] == Astroid::Astroid) {
                    let offset_x = x + 1;
                    let offset_y = y + 1;

                    let offset_cx = cx + 1;
                    let offset_cy = cy + 1;

                    let delta_x = offset_cx - offset_x;
                    let delta_y = offset_cy - offset_y;

                    let slope = ((delta_y as f64 / delta_x as f64) * 1000000.0) as usize;
                    if set.insert(slope) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn upper_right(cx: usize, cy: usize, map: &[Vec<Astroid>]) -> usize {
    let mut set = HashSet::new();
    let mut count = 0;
    for y in (0..cy).rev() {
        for x in (cx..map[0].len()) {
            if (cx != x && cy != y) {
                if (map[y][x] == Astroid::Astroid) {
                    let offset_x = x + 1;
                    let offset_y = y + 1;

                    let offset_cx = cx + 1;
                    let offset_cy = cy + 1;

                    let delta_x = offset_x - offset_cx;
                    let delta_y = offset_cy - offset_y;

                    let slope = ((delta_y as f64 / delta_x as f64) * 1000000.0) as usize;
                    if set.insert(slope) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn down_right(cx: usize, cy: usize, map: &[Vec<Astroid>]) -> usize {
    let mut set = HashSet::new();
    let mut count = 0;
    for y in (cy..map.len()) {
        for x in (cx..map[0].len()) {
            if (cx != x && cy != y) {
                if (map[y][x] == Astroid::Astroid) {
                    let offset_x = x + 1;
                    let offset_y = y + 1;

                    let offset_cx = cx + 1;
                    let offset_cy = cy + 1;

                    let delta_x = offset_x - offset_cx;
                    let delta_y = offset_y - offset_cy;

                    let slope = ((delta_y as f64 / delta_x as f64) * 1000000.0) as usize;
                    if set.insert(slope) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn cross_search(cx: usize, cy: usize, map: &[Vec<Astroid>]) -> usize {
    let mut count = 0;
    for x in (0..cx).rev() {
        if map[cy][x] == Astroid::Astroid {
            count += 1;
            break;
        }
    }

    for x in (cx + 1..map[0].len()) {
        if map[cy][x] == Astroid::Astroid {
            count += 1;
            break;
        }
    }

    for y in (0..cy).rev() {
        if map[y][cx] == Astroid::Astroid {
            count += 1;
            break;
        }
    }

    for y in (cy + 1..map.len()) {
        if map[y][cx] == Astroid::Astroid {
            count += 1;
            break;
        }
    }

    count
}

fn down_left(cx: usize, cy: usize, map: &[Vec<Astroid>]) -> usize {
    let mut set = HashSet::new();
    let mut count = 0;
    for y in (cy..map.len()) {
        for x in (0..cx).rev() {
            if (cx != x && cy != y) {
                if (map[y][x] == Astroid::Astroid) {
                    let offset_x = x + 1;
                    let offset_y = y + 1;

                    let offset_cx = cx + 1;
                    let offset_cy = cy + 1;

                    let delta_x = offset_cx - offset_x;
                    let delta_y = offset_y - offset_cy;

                    let slope = ((delta_y as f64 / delta_x as f64) * 1000000.0) as usize;
                    if set.insert(slope) {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}
fn main() {
    let mut astroid = parse(PUZZLE.trim());

    let p1 = (0..astroid.len())
        .flat_map(|y| (0..astroid[0].len()).map(move |x| (x, y)))
        .filter(|&(x, y)| astroid[y][x] == Astroid::Astroid)
        .map(|(x, y)| {
            upper_right(x, y, &astroid)
                + upper_left(x, y, &astroid)
                + down_right(x, y, &astroid)
                + down_left(x, y, &astroid)
                + cross_search(x, y, &astroid)
        })
        .max()
        .unwrap();

    println!("Part 1: {}\n", p1);
}
