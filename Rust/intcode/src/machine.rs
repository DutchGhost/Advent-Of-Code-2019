use std::{
    convert::{TryFrom, TryInto},
    num::TryFromIntError,
};

use crate::{
    //future::{sink::Sink, stream::Stream, Future, Poll},
    device::{Poll, Device},
    io::{Read, Write},
    opcode::{Mnemonic, Mode, Opcode, OpcodeError},
};

#[derive(Debug)]
pub enum MachineError {
    IndexOutOfBounds { len: usize, index: usize },
    InvalidMode { mode: usize },
    WriteInImmediateMode,
    TryFromIntError,
    ReaderExhausted,
    SinkSendError,
    SinkPrepareError,
    OpcodeError(OpcodeError<isize, isize>),
}

impl From<TryFromIntError> for MachineError {
    fn from(_: TryFromIntError) -> Self {
        Self::TryFromIntError
    }
}

impl From<!> for MachineError {
    fn from(never: !) -> Self {
        match never {}
    }
}

impl From<OpcodeError<isize, isize>> for MachineError {
    fn from(e: OpcodeError<isize, isize>) -> Self {
        Self::OpcodeError(e)
    }
}

pub struct Machine<T, IO: Read<Item = T> + Write<T>> {
    ip: usize,
    base: isize,
    memory: Vec<T>,
    io: IO,
}

impl<T, IO: Read<Item = T> + Write<T>> Machine<T, IO> {
    #[inline(always)]
    pub fn new(memory: Vec<T>, io: IO) -> Self {
        Self {
            ip: 0,
            base: 0,
            memory,
            io,
        }
    }
}

macro_rules! oob {
    ($len: expr, $index: expr) => {
        $crate::machine::MachineError::IndexOutOfBounds {
            len: $len,
            index: $index,
        }
    };
}

impl<T, IO: Read<Item = T> + Write<T>> Machine<T, IO> {
    #[inline(always)]
    fn ip(&self) -> usize {
        self.ip
    }

    #[inline(always)]
    fn base(&self) -> isize {
        self.base
    }

    #[inline]
    fn write(&mut self, addr: usize, value: T) -> Result<(), MachineError> {
        let len = self.memory.len();
        let elem = self.memory.get_mut(addr).ok_or(oob!(len, addr))?;
        *elem = value;
        Ok(())
    }
}

impl<T: Clone, IO: Read<Item = T> + Write<T>> Machine<T, IO> {
    #[inline]
    fn read(&self, index: usize) -> Result<T, MachineError> {
        self.memory
            .get(index)
            .cloned()
            .ok_or(oob!(self.memory.len(), index))
    }
}

impl<T: Clone + TryInto<isize> + TryInto<usize>, IO: Read<Item = T> + Write<T>> Machine<T, IO>
where
    MachineError: From<<T as TryInto<isize>>::Error> + From<<T as TryInto<usize>>::Error>,
{
    #[inline]
    fn read_operand(&self, index: usize, mode: Mode) -> Result<T, MachineError> {
        let offset = self.ip() + index;

        let addr = match mode {
            Mode::Immediate => offset as isize,
            Mode::Position => TryInto::<isize>::try_into(self.read(offset)?)?,
            Mode::Relative => TryInto::<isize>::try_into(self.read(offset)?)? + self.base(),
        };

        self.read(usize::try_from(addr)?)
    }

    #[inline]
    fn write_operand(&mut self, index: usize, value: T, mode: Mode) -> Result<(), MachineError> {
        let offset = self.ip() + index;
        let addr = match mode {
            Mode::Immediate => return Err(MachineError::WriteInImmediateMode),
            Mode::Position => TryInto::<isize>::try_into(self.read(offset)?)?,
            Mode::Relative => TryInto::<isize>::try_into(self.read(offset)?)? + self.base(),
        };

        self.write(usize::try_from(addr)?, value)
    }
}

pub trait Intcode {
    type Output;
    fn opcode(&self) -> Result<Opcode, MachineError>;

    fn add(&mut self, modes: &[Mode]) -> Poll<Self::Output>;

    fn mul(&mut self, modes: &[Mode]) -> Poll<Self::Output>;

    fn save(&mut self, modes: &[Mode]) -> Poll<Self::Output>;

    fn output(&mut self, modes: &[Mode]) -> Poll<Self::Output>;

    fn jump_if_true(&mut self, modes: &[Mode]) -> Poll<Self::Output>;

    fn jump_if_false(&mut self, modes: &[Mode]) -> Poll<Self::Output>;

    fn less_than(&mut self, modes: &[Mode]) -> Poll<Self::Output>;

    fn equals(&mut self, modes: &[Mode]) -> Poll<Self::Output>;

    fn adjust_base(&mut self, modes: &[Mode]) -> Poll<Self::Output>;

    fn halt(&mut self, modes: &[Mode]) -> Poll<Self::Output>;
}

impl<IO: Read<Item = isize> + Write<isize>> Intcode for Machine<isize, IO> {
    type Output = Result<(), MachineError>;

    fn opcode(&self) -> Result<Opcode, MachineError> {
        Ok(Opcode::parse(self.read(self.ip)?)?)
    }

    fn add(&mut self, modes: &[Mode]) -> Poll<Self::Output> {
        let i1 = try_unwrap!(self.read_operand(1, modes[0]));
        let i2 = try_unwrap!(self.read_operand(2, modes[1]));
        try_unwrap!(self.write_operand(3, i1 + i2, modes[2]));

        self.ip += 4;
        Poll::Pending
    }

    fn mul(&mut self, modes: &[Mode]) -> Poll<Self::Output> {
        let i1 = try_unwrap!(self.read_operand(1, modes[0]));
        let i2 = try_unwrap!(self.read_operand(2, modes[1]));

        try_unwrap!(self.write_operand(3, i1 * i2, modes[2]));

        self.ip += 4;
        Poll::Pending
    }

    fn save(&mut self, modes: &[Mode]) -> Poll<Self::Output> {
        let value = match ready!(self.io.poll_read()) {
            Some(value) => value,
            None => return Poll::Ready(Err(MachineError::ReaderExhausted)),
        };

        let addr = match modes[0] {
            Mode::Position => try_unwrap!(self.read(self.ip() + 1)),
            Mode::Relative => try_unwrap!(self.read(self.ip() + 1)) + self.base(),
            Mode::Immediate => return Poll::Ready(Err(MachineError::WriteInImmediateMode)),
        };

        let _ = try_unwrap!(self.write(try_unwrap!(usize::try_from(addr)), value));
        self.ip += 2;

        Poll::Pending
    }

    fn output(&mut self, modes: &[Mode]) -> Poll<Self::Output> {

        let value = try_unwrap!(self.read_operand(1, modes[0]));
        match ready!(self.io.poll_write(value)) {
            Ok(_) => {}
            Err(_) => return Poll::Ready(Err(MachineError::SinkPrepareError)),
        };

        self.ip += 2;

        Poll::Pending
    }

    fn jump_if_true(&mut self, modes: &[Mode]) -> Poll<Self::Output> {
        let cond = try_unwrap!(self.read_operand(1, modes[0]));
        let ip = try_unwrap!(self.read_operand(2, modes[1]));

        if cond != 0 {
            self.ip = try_unwrap!(usize::try_from(ip));
        } else {
            self.ip += 3;
        }

        Poll::Pending
    }

    fn jump_if_false(&mut self, modes: &[Mode]) -> Poll<Self::Output> {
        let cond = try_unwrap!(self.read_operand(1, modes[0]));
        let ip = try_unwrap!(self.read_operand(2, modes[1]));

        if cond == 0 {
            self.ip = try_unwrap!(usize::try_from(ip));
        } else {
            self.ip += 3;
        }

        Poll::Pending
    }

    fn less_than(&mut self, modes: &[Mode]) -> Poll<Self::Output> {
        let i1 = try_unwrap!(self.read_operand(1, modes[0]));
        let i2 = try_unwrap!(self.read_operand(2, modes[1]));

        let value = (i1 < i2) as isize;
        try_unwrap!(self.write_operand(3, value, modes[2]));

        self.ip += 4;

        Poll::Pending
    }

    fn equals(&mut self, modes: &[Mode]) -> Poll<Self::Output> {
        let i1 = try_unwrap!(self.read_operand(1, modes[0]));
        let i2 = try_unwrap!(self.read_operand(2, modes[1]));

        let value = (i1 == i2) as isize;
        try_unwrap!(self.write_operand(3, value, modes[2]));

        self.ip += 4;
        Poll::Pending
    }

    fn adjust_base(&mut self, modes: &[Mode]) -> Poll<Self::Output> {
        let new_base = try_unwrap!(self.read_operand(1, modes[0]));
        self.base += new_base;
        self.ip += 2;

        Poll::Pending
    }

    fn halt(&mut self, modes: &[Mode]) -> Poll<Self::Output> {
        Poll::Ready(Ok(()))
    }
}

impl<T, IO: Read<Item = T> + Write<T>> Device for Machine<T, IO>
where
    Self: Intcode
{
    type Output = <Self as Intcode>::Output;

    #[inline]
    fn poll(&mut self) -> Poll<Self::Output> {
        let Opcode { mnemonic, modes } = try_unwrap!(self.opcode());

        match mnemonic {
            Mnemonic::Add => self.add(&modes),
            Mnemonic::Mul => self.mul(&modes),
            Mnemonic::Save => self.save(&modes),
            Mnemonic::Output => self.output(&modes),
            Mnemonic::JumpIfTrue => self.jump_if_true(&modes),
            Mnemonic::JumpIfFalse => self.jump_if_false(&modes),
            Mnemonic::LessThan => self.less_than(&modes),
            Mnemonic::Equals => self.equals(&modes),
            Mnemonic::AdjustBase => self.adjust_base(&modes),
            Mnemonic::Halt => self.halt(&modes),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::future::FutureExt;

    struct Dummy(isize);

    impl Read for Dummy {
        type Item = isize;
        fn poll_read(&mut self) -> Poll<Option<Self::Item>> {
            Poll::Ready(Some(1))
        }
    }

    impl Write<isize> for Dummy {
        type Error = ();

        fn poll_write(&mut self, value: isize) -> Poll<Result<(), Self::Error>> {
            self.0 = value;
            Poll::Ready(Ok(()))
        }
    }
    #[test]
    fn test_read_ext() {
        let mut outputter = Dummy(0);
        let mut m = Machine::<isize, &mut Dummy>::new(
            vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            &mut outputter,
        );

        let r = m.execute();

        assert!(r.is_ok());
        assert_eq!(outputter.0, 999);
    }

    #[test]
    fn test_relative_offset() {
        let mut outputter = Dummy(0);
        let mut m = Machine::<isize, &mut Dummy>::new(
            vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0],
            &mut outputter,
        );

        let r = loop {
            match m.poll() {
                Poll::Ready(r) => break r,
                Poll::Pending => continue,
            }
        };

        assert!(r.is_ok());
        assert_eq!(outputter.0, 1219070632396864);
    }
}
