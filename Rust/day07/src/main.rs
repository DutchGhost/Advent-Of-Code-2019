static PUZZLE: &'static str = include_str!(r"..\..\..\Inputs\day07.txt");

fn parse_input(s: &str) -> Vec<isize> {
    s.split(",").map(|w| w.parse().unwrap()).collect::<Vec<_>>()
}

mod intcode;
use intcode::{ChanneledMachine, Intcode, Machine, Poll, Poll2};

mod channel;
use channel::Channel;

fn run_program(program: Vec<isize>, setting: isize, input_signal: isize) -> isize {
    let mut exit = 0;
    let mut machine = Machine::new(program);

    let inputs = [setting, input_signal];
    let mut inputs_idx = 0;
    loop {
        match machine.step() {
            Poll::Running => {}
            Poll::Output(result) => exit = result,
            Poll::Input(i) => {
                *i = inputs[inputs_idx];
                inputs_idx += 1;
            }
            Poll::Exit => break,
        }
    }
    exit
}

fn run_settings(program: Vec<isize>, settings: &[isize]) -> isize {
    let mut signal = 0;

    for setting in settings {
        signal = run_program(program.clone(), *setting, signal);
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
            if output > highest {

                highest = output;
            }
        }
    }

    highest
}

fn run_till_first_input(machine: &mut ChanneledMachine) {
    loop {
        match machine.step() {
            Poll2::Running => continue,
            Poll2::Exit => unreachable!(),
            Poll2::ReceivedInput => break,
            Poll2::WaitOutput => unreachable!(),
            Poll2::WaitInput => unreachable!(),
        }
    }
}

fn run_till_done(machine: &mut ChanneledMachine) -> bool {
    loop {
        match machine.step() {
            Poll2::Exit => return true,
            Poll2::Running => continue,
            Poll2::ReceivedInput => continue,
            Poll2::WaitInput => return false,
            Poll2::WaitOutput => return false,
        }
    }
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

            run_till_first_input(&mut a);
            run_till_first_input(&mut b);
            run_till_first_input(&mut c);
            run_till_first_input(&mut d);
            run_till_first_input(&mut e);

            ce_tx.send(0);

            let mut done = Vec::new();
            let mut v = vec![("a", a), ("b", b), ("c", c), ("d", d), ("e", e)];
            loop {
                if v.is_empty() {
                    break;
                }

                match v.pop() {
                    Some((letter, mut machine)) => {
                        if !run_till_done(&mut machine) {
                            v.insert(0, (letter, machine));
                        } else {
                            done.push((letter, machine))
                        }
                    }
                    None => panic!(),
                }
            }

            let result = done
                .into_iter()
                .find(|(l, _)| *l == "e")
                .map(|(_, m)| m.last_output)
                .unwrap()
                .unwrap();
            result
        }
    }
}

fn part2(s: &str) -> isize {
    let program = parse_input(s);

    let mut highest = 0;

    multi_for! {
        [a, b, c, d, e] in [5..=9, 5..=9, 5..=9, 5..=9, 5..=9] {
            if filter!(a, b, c, d, e) { continue }

            let result = channeled_run(program.clone(), [a, b, c, d, e]);

            if result > highest {
                highest = result;
            }
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
                3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28,
                -1, 28, 1005, 28, 6, 99, 0, 0, 5,
            ],
            [9, 8, 7, 6, 5],
        );

        assert_eq!(out, 139629729);
    }
}
