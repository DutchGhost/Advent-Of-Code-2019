static PUZZLE: &'static str = include_str!(r"..\..\..\Inputs\day08.txt");

struct Layer {
    zeros: usize,
    ones_by_twos: usize,
}

fn freqs(iter: impl Iterator<Item = u8>) -> Layer {
    let mut freqs = [0usize; 3];

    for elem in iter {
        freqs[elem as usize] += 1;
    }

    Layer {
        zeros: freqs[0],
        ones_by_twos: freqs[1] * freqs[2],
    }
}

fn part1(image: &[u8]) -> usize {
    image
        .chunks(25 * 6)
        .map(|chunk| freqs(chunk.iter().copied()))
        .min_by_key(|layer| layer.zeros)
        .map(|layer| layer.ones_by_twos)
        .unwrap()
}

#[derive(Copy, Clone)]
enum Color {
    Black,
    White,
    Transparent,
}

impl Color {
    fn from(bit: u8) -> Self {
        match bit {
            0 => Self::Black,
            1 => Self::White,
            2 => Self::Transparent,
            n => panic!(format!("Invalid color: {}", n)),
        }
    }
}

impl std::ops::BitOrAssign for Color {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = match (*self, rhs) {
            (Self::Transparent, rhs) => rhs,
            (lhs, _) => lhs,
        }
    }
}

fn part2(image: &[u8]) -> usize {
    let mut final_img = [[Color::Transparent; 25]; 6];

    for layer in image.chunks(25 * 6) {
        for (y, row) in layer.chunks(25).enumerate() {
            for (x, bit) in row.iter().enumerate() {
                final_img[y][x] |= Color::from(*bit);
            }
        }
    }

    for row in final_img.iter() {
        for elem in row.iter() {
            match elem {
                Color::White => print!("#"),
                Color::Black => print!(" "),
                Color::Transparent => panic!(),
            }
        }
        println!();
    }

    0
}

fn main() {
    let input = PUZZLE
        .trim()
        .chars()
        .map(|c| c as u8 - 48)
        .collect::<Vec<_>>();

    let p1 = part1(&input);
    let p2 = part2(&input);

    println!("Part 1: {}\nPart 2: {}", p1, p2);
}
