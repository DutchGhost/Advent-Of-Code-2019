use crate::channel::{Rx, Tx};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Mode {
    Position,
    Immediate,
}

impl From<isize> for Mode {
    fn from(mode: isize) -> Self {
        [Self::Position, Self::Immediate][mode as usize]
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Op {
    Add,
    Mul,
    Save,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}

impl From<isize> for Op {
    fn from(code: isize) -> Self {
        match code % 100 {
            1 => Self::Add,
            2 => Self::Mul,
            3 => Self::Save,
            4 => Self::Output,
            5 => Self::JumpIfTrue,
            6 => Self::JumpIfFalse,
            7 => Self::LessThan,
            8 => Self::Equals,
            99 => Self::Halt,
            n => panic!(format!("Invalid opcode: {}", n)),
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Opcode {
    opcode: Op,
    modes: [Mode; 3],
}

impl Opcode {
    fn parse(code: isize) -> Self {
        let opcode = Op::from(code);

        let modes = [
            Mode::from(code / 100 % 10),
            Mode::from(code / 1000 % 10),
            Mode::from(code / 10_000 % 10),
        ];

        Self { opcode, modes }
    }
}

pub enum Poll<'a, T> {
    Running,
    Exit,
    Input(&'a mut T),
    Output(T),
}

pub enum Poll2 {
    Running,
    Exit,
    WaitInput,
    WaitOutput,
    ReceivedInput,
}

#[derive(Debug)]
pub struct RunError {
    s: &'static str,
}

impl RunError {
    pub const fn new(s: &'static str) -> Self {
        Self { s }
    }
}

#[derive(Debug)]
pub struct Machine {
    ip: usize,
    memory: Vec<isize>,
}

pub trait InstructionPointer {
    fn ip(&self) -> usize;

    fn set_ip(&mut self, new_ip: usize);
}

pub trait Memory {
    fn memory(&mut self) -> &mut [isize];
}

pub trait Read: InstructionPointer {
    fn read(&self, addr: usize) -> isize;

    fn read_arg(&self, index: usize, mode: Mode) -> isize {
        match mode {
            Mode::Position => {
                let addr = self.read(self.ip() + index);
                assert!(addr > -1);
                self.read(addr as usize)
            }
            Mode::Immediate => self.read(self.ip() + index),
        }
    }
}

pub trait Write: Read {
    fn write(&mut self, addr: usize, value: isize);

    fn write_arg(&mut self, index: usize, value: isize) {
        let addr = self.read(self.ip() + index);
        self.write(addr as usize, value)
    }
}

impl InstructionPointer for Machine {
    fn ip(&self) -> usize {
        self.ip
    }
    fn set_ip(&mut self, new_ip: usize) {
        self.ip = new_ip;
    }
}

impl Memory for Machine {
    fn memory(&mut self) -> &mut [isize] {
        &mut self.memory
    }
}

impl Read for Machine {
    fn read(&self, addr: usize) -> isize {
        self.memory[addr]
    }
}

impl Write for Machine {
    fn write(&mut self, addr: usize, value: isize) {
        self.memory[addr] = value;
    }
}

pub trait Intcode: Read + Write + Memory {
    fn opcode(&self) -> Opcode {
        Opcode::parse(self.read(self.ip()))
    }

    fn add(&mut self, modes: &[Mode]) -> Poll<isize> {
        let i1 = self.read_arg(1, modes[0]);
        let i2 = self.read_arg(2, modes[1]);
        self.write_arg(3, i1 + i2);
        self.set_ip(self.ip() + 4);

        Poll::Running
    }

    fn mul(&mut self, modes: &[Mode]) -> Poll<isize> {
        let i1 = self.read_arg(1, modes[0]);
        let i2 = self.read_arg(2, modes[1]);
        self.write_arg(3, i1 * i2);
        self.set_ip(self.ip() + 4);

        Poll::Running
    }

    fn jump_if_false(&mut self, modes: &[Mode]) -> Poll<isize> {
        let cond = self.read_arg(1, modes[0]);
        let ip = self.read_arg(2, modes[1]);

        if cond == 0 {
            self.set_ip(ip as usize);
        } else {
            self.set_ip(self.ip() + 3)
        }

        Poll::Running
    }

    fn jump_if_true(&mut self, modes: &[Mode]) -> Poll<isize> {
        let cond = self.read_arg(1, modes[0]);
        let ip = self.read_arg(2, modes[1]);

        if cond != 0 {
            self.set_ip(ip as usize);
        } else {
            self.set_ip(self.ip() + 3);
        }

        Poll::Running
    }

    fn less_than(&mut self, modes: &[Mode]) -> Poll<isize> {
        let i1 = self.read_arg(1, modes[0]);
        let i2 = self.read_arg(2, modes[1]);

        let value = (i1 < i2) as isize;
        self.write_arg(3, value);
        self.set_ip(self.ip() + 4);

        Poll::Running
    }

    fn equals(&mut self, modes: &[Mode]) -> Poll<isize> {
        let i1 = self.read_arg(1, modes[0]);
        let i2 = self.read_arg(2, modes[1]);

        let value = (i1 == i2) as isize;

        self.write_arg(3, value);
        self.set_ip(self.ip() + 4);

        Poll::Running
    }

    fn save(&mut self) -> Poll<isize> {
        let addr = self.read(self.ip() + 1);

        self.set_ip(self.ip() + 2);
        let out = Poll::Input(&mut self.memory()[addr as usize]);

        out
    }

    fn output(&mut self, modes: &[Mode]) -> Poll<isize> {
        let out = Poll::Output(self.read_arg(1, modes[0]));
        self.set_ip(self.ip() + 2);

        out
    }

    fn halt(&self) -> Poll<isize> {
        Poll::Exit
    }

    fn step(&mut self) -> Poll<isize> {
        let Opcode { opcode, modes } = self.opcode();
        match opcode {
            Op::Add => self.add(&modes),
            Op::Mul => self.mul(&modes),
            Op::Save => self.save(),
            Op::Output => self.output(&modes),
            Op::JumpIfTrue => self.jump_if_true(&modes),
            Op::JumpIfFalse => self.jump_if_false(&modes),
            Op::LessThan => self.less_than(&modes),
            Op::Equals => self.equals(&modes),
            Op::Halt => self.halt(),
        }
    }

    fn run<I>(&mut self, into_iter: I) -> Result<isize, RunError>
    where
        I: IntoIterator<Item = isize>,
    {
        let mut exit = 0;
        let mut iter = into_iter.into_iter();

        loop {
            match self.step() {
                Poll::Running => {}
                Poll::Output(result) => exit = result,
                Poll::Input(input) => {
                    *input = iter.next().ok_or(RunError::new("Failed to give input"))?
                }
                Poll::Exit => break,
            }
        }

        Ok(exit)
    }
}

impl<I> Intcode for I where I: Read + Write + Memory {}

impl Machine {
    pub fn new(memory: Vec<isize>) -> Self {
        Self { ip: 0, memory }
    }
}

#[derive(Debug)]
pub struct ChanneledMachine<'a> {
    machine: Machine,
    recv: Rx<'a, isize>,
    snd: Tx<'a, isize>,
    pub(crate) is_done: bool,
}

impl<'a> ChanneledMachine<'a> {
    pub fn new(memory: Vec<isize>, snd: Tx<'a, isize>, recv: Rx<'a, isize>) -> Self {
        Self {
            machine: Machine::new(memory),
            snd,
            recv,
            is_done: false,
        }
    }

    pub fn step<'s>(&'s mut self) -> Poll2 {
        if self.is_done {
            return Poll2::Exit;
        }
        match self.machine.step() {
            Poll::Running => Poll2::Running,
            Poll::Exit => {
                self.is_done = true;
                Poll2::Exit
            }
            Poll::Input(i) => match self.recv.recv() {
                Err(_) => {
                    self.machine.set_ip(self.machine.ip() - 2);
                    Poll2::WaitInput
                }
                Ok(elem) => {
                    *i = elem;
                    Poll2::ReceivedInput
                }
            },
            Poll::Output(i) => match self.snd.send(i) {
                Ok(()) => Poll2::Running,
                Err(_) => {
                    self.machine.set_ip(self.machine.ip() - 2);
                    Poll2::WaitOutput
                }
            },
        }
    }

    /// Returns true when the machine is done,
    /// false otherwise.
    pub fn make_progress(&mut self) -> bool {
        loop {
            match self.step() {
                Poll2::Exit => return true,
                Poll2::Running => continue,
                Poll2::ReceivedInput => continue,
                Poll2::WaitInput => return false,
                Poll2::WaitOutput => return false,
            }
        }
    }
}
