static PUZZLE: &'static str = include_str!(r"..\..\..\Inputs\day07.txt");

fn parse_input(s: &str) -> Vec<isize> {
    s.split(",").map(|w| w.parse().unwrap()).collect::<Vec<_>>()
}

use intcode::{
    channel::Channel,
    future::{
        sink::Stdout,
        stream::{once, Stream, StreamExt},
        FutureExt, Poll,
    },
    machine::Machine,
};

use std::cmp;

fn run_settings(program: Vec<isize>, settings: &[isize]) -> isize {
    let mut signal = 0;

    for setting in settings {
        let mut stdout = Stdout::new();

        let mut machine = Machine::new(
            program.clone(),
            once(*setting).chain(once(signal)),
            &mut stdout,
        );
        let _ = machine.execute();

        signal = stdout.into_inner().unwrap();
    }

    signal
}

macro_rules! multi_for {
    ([$name:ident $(,$names:ident)*] in [$e:expr $(,$es:expr)*] $b:block) => {
        for $name in $e {
            multi_for!([$($names),*] in [$($es),*] $b);
        }
    };

    ([] in [] $b:block) => {$b}
}

macro_rules! filter {
    ($name:ident $(,$names:ident)*) => {
        false $(|| $name == $names)* || filter!($($names),*)
    };
    () => {false}
}

fn part1(s: &str) -> isize {
    let program = parse_input(s);

    let mut highest = 0;
    multi_for! {
        [a, b, c, d, e] in [0..=4, 0..=4, 0..=4, 0..=4, 0..=4] {
            if filter!(a, b, c, d, e) { continue }
            let output = run_settings(program.clone(), &[a, b, c, d, e]);
            highest = cmp::max(highest, output);
        }
    }

    highest
}

fn channeled_run(program: Vec<isize>, seq: [isize; 5]) -> isize {
    match seq {
        [a, b, c, d, e] => {
            let c_a = Channel::empty();
            let c_b = Channel::empty();
            let c_c = Channel::empty();
            let c_d = Channel::empty();
            let c_e = Channel::empty();

            let (a_transmit, b_recv) = c_a.split();
            let (b_transmit, c_recv) = c_b.split();
            let (c_transmit, d_recv) = c_c.split();
            let (d_transmit, e_recv) = c_d.split();
            let (e_transmit, mut a_recv) = c_e.split();

            let a = Machine::new(
                program.clone(),
                once(a).chain(once(0)).chain(a_recv),
                a_transmit,
            );
            let b = Machine::new(program.clone(), once(b).chain(b_recv), b_transmit);
            let c = Machine::new(program.clone(), once(c).chain(c_recv), c_transmit);
            let d = Machine::new(program.clone(), once(d).chain(d_recv), d_transmit);
            let e = Machine::new(program.clone(), once(e).chain(e_recv), e_transmit);

            let mut joined = a.join5(b, c, d, e);

            let _ = joined.execute();
            match a_recv.poll_next() {
                Poll::Ready(Some(elem)) => return elem,
                _ => panic!(),
            }
        }
    }
}

fn part2(s: &str) -> isize {
    let program = parse_input(s);

    let mut highest = 0;

    multi_for! {
        [a, b, c, d, e] in [5..=9, 5..=9, 5..=9, 5..=9, 5..=9] {
            if filter!(a, b, c, d, e) { continue }
            let output = channeled_run(program.clone(), [a, b, c, d, e]);
            highest = cmp::max(highest, output);
        }
    }

    highest
}

fn main() {
    let p1 = part1(PUZZLE);
    let p2 = part2(PUZZLE);
    println!("Part 1: {}\nPart 2: {}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_me() {
        let out = run_settings(
            vec![
                3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
            ],
            &[4, 3, 2, 1, 0],
        );

        assert_eq!(out, 43210);
    }

    #[test]
    fn assert_channeled_run() {
        let out = channeled_run(
            vec![
                3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001,
                54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53,
                55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
            ],
            [9, 7, 8, 5, 6],
        );

        assert_eq!(out, 18216);
    }
}
