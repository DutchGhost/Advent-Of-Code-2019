static PUZZLE: &'static str = include_str!(r"..\..\..\Inputs\day06.txt");

mod orbit;
use orbit::*;

use std::collections::HashSet;

fn parse(s: &str) -> (&str, &str) {
    let mut splitter = s.split(")");

    (splitter.next().unwrap(), splitter.next().unwrap())
}

fn recursive<'a>(orbittee: &'a str, map: &OrbitGraph<&'a str, HashSet<&'a str>>) -> usize {
    let mut count = 0;

    map.orbit_index(&orbittee)
        .map(|orbits| {
            orbits
                .iter()
                .fold(0, |count, orbitter| count + 1 + recursive(orbitter, map))
        })
        .unwrap_or(0)
}

fn part1(s: &str) -> usize {
    let mut orbits: OrbitGraph<&str, HashSet<&str>> = OrbitGraph::new();

    for line in PUZZLE.lines() {
        let (orbittee, orbitter) = parse(line);
        orbits.orbit(orbittee).add(orbitter);
        orbits.orbit(orbitter);
    }

    orbits
        .iter()
        .map(|(orbittee, _)| recursive(orbittee, &orbits))
        .sum()
}

fn main() {
    let p1 = part1(PUZZLE);

    println!("Part 1: {}\n", p1);
}
