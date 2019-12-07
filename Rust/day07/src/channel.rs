use std::{
    cell::Cell,
    fmt::{self, Debug},
};

pub enum SendError<T> {
    /// There already exists a value,
    /// so if we send, we'd overwrite
    /// without the reciver receiving
    /// the existing value.
    NotEmpty(T),
}

#[derive(Debug)]
pub enum RecvError {
    /// The channel was empty while
    /// attempting to receive a value
    Empty,
}

/// A multiple consumer, multiple producer, single-threaded channel
/// with storage for only 1 value. A channer can be split into its
/// Transmitter (Tx), and Reciver (Rx) usiing the [`Channel::split`]
/// method.
pub struct Channel<T> {
    pub inner: Cell<Option<T>>,
}

impl<'a, T: Copy + Debug> Debug for Channel<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.get().fmt(f)
    }
}

impl<T> Channel<T> {
    #[inline]
    pub const fn new(value: T) -> Self {
        Self {
            inner: Cell::new(Some(value)),
        }
    }

    #[inline]
    pub const fn split<'a>(&'a self) -> (Tx<'a, T>, Rx<'a, T>) {
        (Tx::new(&self.inner), Rx::new(&self.inner))
    }
}

/// The transmitting end of a channel.
#[derive(Copy)]
pub struct Tx<'a, T> {
    inner: &'a Cell<Option<T>>,
}

impl<'a, T: Copy + Debug> Debug for Tx<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.get().fmt(f)
    }
}

impl<'a, T> Clone for Tx<'a, T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self { ..*self }
    }
}

impl<'a, T> Tx<'a, T> {
    #[inline(always)]
    pub const fn new(inner: &'a Cell<Option<T>>) -> Self {
        Self { inner }
    }

    #[inline]
    pub fn send(&self, value: T) -> Result<(), SendError<T>> {
        match self.inner.replace(None) {
            // There already exists a value.
            // Put it back, and return an error
            Some(v) => {
                self.inner.replace(Some(v));
                return Err(SendError::NotEmpty(value));
            }
            None => {}
        }

        self.inner.replace(Some(value));
        Ok(())
    }
}

/// The receiving end of a channel.
#[derive(Copy)]
pub struct Rx<'a, T> {
    inner: &'a Cell<Option<T>>,
}

impl<'a, T: Copy + Debug> Debug for Rx<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.get().fmt(f)
    }
}

impl<'a, T> Clone for Rx<'a, T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self { ..*self }
    }
}

impl<'a, T> Rx<'a, T> {
    #[inline(always)]
    pub const fn new(inner: &'a Cell<Option<T>>) -> Self {
        Self { inner }
    }

    #[inline]
    pub fn recv(&self) -> Result<T, RecvError> {
        match self.inner.replace(None) {
            Some(value) => Ok(value),
            None => Err(RecvError::Empty),
        }
    }
}
