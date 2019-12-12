use std::convert::TryFrom;

#[derive(Debug)]
pub struct InvalidMode<N>(N);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Mode {
    Position,
    Immediate,
    Relative,
}

impl TryFrom<isize> for Mode {
    type Error = InvalidMode<isize>;

    fn try_from(n: isize) -> Result<Self, Self::Error> {
        match n {
            0 => Ok(Self::Position),
            1 => Ok(Self::Immediate),
            2 => Ok(Self::Relative),
            n => Err(InvalidMode(n)),
        }
    }
}

#[derive(Debug)]
pub struct InvalidOpCode<N>(N);

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub(crate) enum Mnemonic {
    Add,
    Mul,
    Save,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    AdjustBase,
    Halt,
}

impl TryFrom<isize> for Mnemonic {
    type Error = InvalidOpCode<isize>;

    fn try_from(opcode: isize) -> Result<Self, Self::Error> {
        match opcode % 100 {
            1 => Ok(Self::Add),
            2 => Ok(Self::Mul),
            3 => Ok(Self::Save),
            4 => Ok(Self::Output),
            5 => Ok(Self::JumpIfTrue),
            6 => Ok(Self::JumpIfFalse),
            7 => Ok(Self::LessThan),
            8 => Ok(Self::Equals),
            9 => Ok(Self::AdjustBase),
            99 => Ok(Self::Halt),
            n => Err(InvalidOpCode(n)),
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Opcode {
    pub(crate) mnemonic: Mnemonic,
    pub(crate) modes: [Mode; 3],
}

#[derive(Debug)]
pub enum OpcodeError<C, M> {
    InvalidOpCode(InvalidOpCode<C>),
    InvalidMode(InvalidMode<M>),
}

impl<C, M> From<InvalidOpCode<C>> for OpcodeError<C, M> {
    fn from(e: InvalidOpCode<C>) -> Self {
        Self::InvalidOpCode(e)
    }
}

impl<C, M> From<InvalidMode<M>> for OpcodeError<C, M> {
    fn from(e: InvalidMode<M>) -> Self {
        Self::InvalidMode(e)
    }
}

impl Opcode {
    pub fn parse(code: isize) -> Result<Self, OpcodeError<isize, isize>> {
        let mnemonic = Mnemonic::try_from(code)?;

        let modes = [
            Mode::try_from(code / 100 % 10)?,
            Mode::try_from(code / 1000 % 10)?,
            Mode::try_from(code / 10_000 % 10)?,
        ];

        Ok(Self { mnemonic, modes })
    }
}
