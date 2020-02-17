static PUZZLE: &'static str = include_str!(r"..\..\..\Inputs\day06.txt");

use std::collections::{HashMap, HashSet};

mod get;
use get::Get;

fn parse(s: &str) -> (&str, &str) {
    let mut splitter = s.split(")");

    (splitter.next().unwrap(), splitter.next().unwrap())
}

fn orbitters_of<K: ?Sized, S>(orbittee: &K, map: &S) -> usize
where
    S: Get<K>,
    for<'out> &'out S::Output: IntoIterator<Item = &'out K>,
{
    Get::get(map, &orbittee)
        .map(|orbits| {
            orbits
                .into_iter()
                .fold(0, |count, orbitter| count + 1 + orbitters_of(orbitter, map))
        })
        .unwrap_or(0)
}

fn part1<I, T, U>(orbits: &I) -> usize
where
    I: Get<T>,
    for<'iter> &'iter I: IntoIterator<Item = (&'iter T, &'iter U)>,
    for<'out> &'out <I as Get<T>>::Output: IntoIterator<Item = &'out T>,
{
    orbits
        .into_iter()
        .map(|(orbittee, _): (&T, &U)| orbitters_of(orbittee, orbits))
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
                    .find_map(|orbittee| contains_orbitter(orbittee, orbitter, map))
                    .map(|depth| depth + 1)
            },
            |_| Some(1),
        )
    })
}

fn part2_alternative<'a>(orbits: &HashMap<&'a str, HashSet<&'a str>>) -> usize {
    std::iter::successors(Some("SAN"), |orbitter| orbittee_of(orbitter, orbits))
        .enumerate()
        .find_map(|(count, orbitter)| Some((count, contains_orbitter(orbitter, "YOU", orbits)?)))
        .map(|(count, depth)| count + depth - 2)
        .unwrap()
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
