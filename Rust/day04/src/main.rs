const BEGIN: [u8; 6] = [1, 4, 6, 8, 1, 0];
const END: [u8; 6] = [6, 1, 2, 5, 6, 4];

mod group;
use group::Grouped;

macro_rules! multi_for {
    ([$name:ident $(,$names:ident)*] in [$e:expr $(,$es:expr)*] $b:block) => {
        for $name in $e {
            multi_for!([$($names),*] in [$($es),*] $b);
        }
    };

    ([] in [] $b:block) => {$b}
}

fn loopy<F: Fn(u8, u8, u8, u8, u8, u8) -> bool>(func: F) -> usize {
    let mut valids = 0;

    multi_for! {
        [a, b, c, d, e, f] in [0..10, a..10, b..10, c..10, d..10, e..10] {
            if [a, b, c, d, e, f] < BEGIN {
                continue;
            }
            if [a, b, c, d, e, f] > END {
                return valids;
            }
            if func(a, b, c, d, e, f) {
                valids += 1;
            }
        }
    };
    valids
}

fn part1() -> usize {
    loopy(|a, b, c, d, e, f| a == b || b == c || c == d || d == e || e == f)
}

fn part2() -> usize {
    loopy(|a, b, c, d, e, f| Grouped::groups(&[a, b, c, d, e, f]).any(|group| group.len() == 2))
}

fn main() {
    let p1 = part1();
    let p2 = part2();

    println!("Part 1: {}\nPart 2: {}", p1, p2);
}
