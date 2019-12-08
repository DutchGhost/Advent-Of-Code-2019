static PUZZLE: &'static str = include_str!(r"..\..\..\Inputs\day08.txt");

struct Layer {
    zeros: usize,
    calc: usize,
}

fn freqs(iter: impl Iterator<Item = u8>) -> [usize; 10] {
    let mut freqs = [0usize; 10];

    for elem in iter {
        freqs[elem as usize] += 1;
    }

    freqs
}

fn main() {
    let mut iter = PUZZLE.trim().chars().map(|c| c as u8 - 48);

    let mut fewest_0_digits = std::usize::MAX;
    let mut calc = 0;
    loop {
        let layer = freqs(iter.by_ref().take(25 * 6));

        if layer == [0; 10] {
            break;
        }

        if layer[0] < fewest_0_digits {
            fewest_0_digits = layer[0];
            calc = layer[1] * layer[2];
        }
    }

    println!("Part 1: {}", calc);
}
