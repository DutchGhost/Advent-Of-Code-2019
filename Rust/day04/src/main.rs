const BEGIN: usize = 146810;
const END: usize = 612564;

mod group;
use group::Grouped;

fn calc_num(nums: [usize; 6]) -> usize {
    let mut multiplier = 1;

    let mut result = 0;
    for n in nums.iter().rev() {
        result += n * multiplier;
        multiplier *= 10;
    }
    result
}

fn loopy<F: Fn(usize, usize, usize, usize, usize, usize) -> bool>(func: F) -> usize {
    let mut ok = 0;

    for a in 0..10 {
        for b in a..10 {
            for c in b..10 {
                for d in c..10 {
                    for e in d..10 {
                        for f in e..10 {
                            let n = calc_num([a, b, c, d, e, f]);
                            if n < BEGIN {
                                continue;
                            }
                            if n > END {
                                return ok;
                            }
                            if func(a, b, c, d, e, f) {
                                ok += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    ok
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
