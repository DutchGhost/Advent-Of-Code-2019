static PUZZLE: &'static str = include_str!(r"..\..\..\Inputs\day06.txt");

use std::collections::{HashMap, HashSet};

fn parse(s: &str) -> (&str, &str) {
    let mut splitter = s.split(")");

    (splitter.next().unwrap(), splitter.next().unwrap())
}

fn orbitters_of<'a>(orbittee: &'a str, map: &HashMap<&'a str, HashSet<&'a str>>) -> usize {
    map.get(&orbittee)
        .map(|orbits| {
            orbits
                .iter()
                .fold(0, |count, orbitter| count + 1 + orbitters_of(orbitter, map))
        })
        .unwrap_or(0)
}

fn part1<'a>(orbits: &HashMap<&'a str, HashSet<&'a str>>) -> usize {
    orbits
        .iter()
        .map(|(orbittee, _)| orbitters_of(orbittee, &orbits))
        .sum()
}

fn path_till_comm<'a>(
    stack: &mut Vec<&'a str>,
    orbittee: &'a str,
    map: &HashMap<&'a str, HashSet<&'a str>>,
) {
    map.iter()
        .find(|(_, v)| v.contains(orbittee))
        .map(|(k, _)| {
            stack.push(k);

            path_till_comm(stack, k, map);
        });
}

fn part2<'a>(orbits: &HashMap<&'a str, HashSet<&'a str>>) -> usize {
    let mut san = Vec::new();

    path_till_comm(&mut san, "SAN", &orbits);

    let mut you = Vec::new();

    path_till_comm(&mut you, "YOU", &orbits);

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

fn orbittee_of<'a>(orbitter: &'a str, map: &HashMap<&'a str, HashSet<&'a str>>) -> Option<&'a str> {
    map.iter()
        .find(|(_, orbitters)| orbitters.contains(orbitter))
        .map(|(orbittee, _)| *orbittee)
}

fn contains_orbitter<'a>(
    orbittee: &'a str,
    orbitter: &'a str,
    map: &HashMap<&'a str, HashSet<&'a str>>,
) -> Option<usize> {
    map.get(orbittee).and_then(|orbitters| {
        orbitters.get(orbitter).map_or_else(
            || {
                orbitters
                    .iter()
                    .filter_map(|orbittee| contains_orbitter(orbittee, orbitter, map))
                    .map(|depth| depth + 1)
                    .next()
            },
            |_| Some(1),
        )
    })
}

fn part2_alternative<'a>(orbits: &HashMap<&'a str, HashSet<&'a str>>) -> usize {
    let (count, depth) =
        std::iter::successors(Some("SAN"), |orbitter| orbittee_of(orbitter, orbits))
            .map(|orbitter| Some(contains_orbitter(orbitter, "YOU", orbits)))
            .enumerate()
            .find(|(_, depth)| depth.unwrap().is_some())
            .map(|(count, depth)| (count, depth.unwrap().unwrap()))
            .unwrap();

    count + depth - 2
}

fn main() {
    let mut orbits: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in PUZZLE.lines() {
        let (orbittee, orbitter) = parse(line);
        orbits
            .entry(orbittee)
            .or_insert_with(HashSet::default)
            .insert(orbitter);
        orbits.entry(orbitter).or_insert_with(HashSet::default);
    }
    let p1 = part1(&orbits);
    let p2 = part2(&orbits);
    let p2_alternative = part2_alternative(&orbits);

    println!(
        "Part 1: {}\nPart 2: {}\nPart 2 alternative: {}",
        p1, p2, p2_alternative
    );
}
