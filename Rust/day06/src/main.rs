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

    match map.orbit_index(&orbittee) {
        Some(orbits) => {
            for orbitter in orbits.iter() {
                count += 1;
                let counted = recursive(orbitter, map);
                count += counted;
            }
            count
        }
        None => 0,
    }
}

fn main() {
    let mut orbits: OrbitGraph<&str, HashSet<&str>> = OrbitGraph::new();

    for line in PUZZLE.lines() {
        let (orbittee, orbitter) = parse(line);
        orbits.orbit(orbittee).add(orbitter);
        orbits.orbit(orbitter);
    }

    let mut total = 0;
    for (k, _) in orbits.iter() {
        total += recursive(k, &orbits);
    }

    dbg!(total);
}
