#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Mode {
    Position,
    Immediate
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
    modes: [Mode; 3]
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
            },
            Mode::Immediate => self.read(self.ip() + index)
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
    fn ip(&self) -> usize { self.ip }
    fn set_ip(&mut self, new_ip: usize) { self.ip = new_ip; }
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
            Op::Halt => self.halt()
        }
    }
}

impl <I> Intcode for I where I: Read + Write + Memory {}

impl Machine {
    pub fn new(memory: Vec<isize>) -> Self {
        Self { ip: 0, memory }
    }
}