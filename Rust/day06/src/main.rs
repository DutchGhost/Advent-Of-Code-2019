static PUZZLE: &'static str = include_str!(r"..\..\..\Inputs\day06.txt");

use std::collections::{HashSet, HashMap};

fn parse(s: &str) -> (&str, &str) {
    let mut splitter = s.split(")");

    (splitter.next().unwrap(), splitter.next().unwrap())
}

fn recursive<'a>(orbittee: &'a str, map: &HashMap<&'a str, HashSet<&'a str>>) -> usize {

    map.get(&orbittee)
        .map(|orbits| {
            orbits
                .iter()
                .fold(0, |count, orbitter| count + 1 + recursive(orbitter, map))
        })
        .unwrap_or(0)
}

fn part1<'a>(orbits: &HashMap<&'a str, HashSet<&'a str>>) -> usize {
    orbits
        .iter()
        .map(|(orbittee, _)| recursive(orbittee, &orbits))
        .sum()
}

fn recfind<'a>(
    stack: &mut Vec<&'a str>,
    orbittee: &'a str,
    map: &HashMap<&'a str, HashSet<&'a str>>,
) {
    map.iter()
        .find(|(_, v)| v.contains(orbittee))
        .map(|(k, _)| {
            stack.push(k);

            recfind(stack, k, map);
        });
}

fn part2<'a>(orbits: &HashMap<&'a str, HashSet<&'a str>>) -> usize {
    let mut san = Vec::new();

    recfind(&mut san, "SAN", &orbits);

    let mut you = Vec::new();

    recfind(&mut you, "YOU", &orbits);

    you.iter()
        .enumerate()
        .filter_map(|(meidx, me)| {
            san.iter()
                .position(|santa| santa == me)
                .map(|santaidx| santaidx + meidx)
        })
        .min()
        .unwrap()
}

fn main() {
    let mut orbits: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in PUZZLE.lines() {
        let (orbittee, orbitter) = parse(line);
        orbits.entry(orbittee).or_insert_with(HashSet::default).insert(orbitter);
        orbits.entry(orbitter).or_insert_with(HashSet::default);
    }

    let p1 = part1(&orbits);
    let p2 = part2(&orbits);

    println!("Part 1: {}\nPart 2: {}", p1, p2);
}
