static PUZZLE: &'static str = include_str!(r"..\..\..\Inputs\day07.txt");

fn parse_input(s: &str) -> Vec<isize> {
    s.split(",").map(|w| w.parse().unwrap()).collect::<Vec<_>>()
}

mod intcode;
use intcode::{ChanneledMachine, Intcode, Machine};

mod channel;
use channel::Channel;

use std::{cmp, iter};

fn run_settings(program: Vec<isize>, settings: &[isize]) -> isize {
    let mut signal = 0;

    for setting in settings {
        signal = match Machine::new(program.clone())
            .run(iter::once(*setting).chain(iter::once(signal)))
        {
            Ok(signal) => signal,
            Err(e) => panic!(e),
        }
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
            let c_a = Channel::new(a);
            let c_b = Channel::new(b);
            let c_c = Channel::new(c);
            let c_d = Channel::new(d);
            let c_e = Channel::new(e);

            let (ca_tx, cb_recv) = c_a.split();
            let (cb_tx, cc_recv) = c_b.split();
            let (cc_tx, cd_recv) = c_c.split();
            let (cd_tx, ce_recv) = c_d.split();
            let (ce_tx, ca_recv) = c_e.split();

            let mut a = ChanneledMachine::new(program.clone(), ca_tx, ca_recv);
            let mut b = ChanneledMachine::new(program.clone(), cb_tx, cb_recv);
            let mut c = ChanneledMachine::new(program.clone(), cc_tx, cc_recv);
            let mut d = ChanneledMachine::new(program.clone(), cd_tx, cd_recv);
            let mut e = ChanneledMachine::new(program.clone(), ce_tx, ce_recv);

            let mut v = vec![&mut a, &mut b, &mut c, &mut d, &mut e];

            let mut did_send_0 = false;

            loop {
                if !did_send_0 {
                    if let Ok(_) = ce_tx.send(0) {
                        did_send_0 = true;
                    };
                }
                if v.is_empty() {
                    break;
                }

                match v.pop() {
                    Some(machine) => {
                        if !machine.make_progress() {
                            v.insert(0, machine);
                        }
                    }
                    None => panic!(),
                }
            }

            ca_recv.recv().unwrap()
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
