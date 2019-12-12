use std::{
    cell::Cell,
    fmt::{self, Debug},
};

use crate::future::{sink::Sink, Poll};

/// The transmitting end of a channel.
#[derive(Copy)]
pub struct Tx<'a, T> {
    inner: &'a Cell<Option<T>>,
    is_empty: &'a Cell<bool>,
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
    pub const fn new(inner: &'a Cell<Option<T>>, is_empty: &'a Cell<bool>) -> Self {
        Self { inner, is_empty }
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty.get()
    }
}

#[derive(Debug)]
pub enum SendError<T> {
    /// There already exists a value,
    /// so if we send, we'd overwrite
    /// without the reciver receiving
    /// the existing value.
    NotEmpty(T),
}

impl<'a, T> Sink<T> for Tx<'a, T> {
    type Error = SendError<T>;

    fn poll_ready(&mut self) -> Poll<Result<(), Self::Error>> {
        if self.is_empty() {
            return Poll::Ready(Ok(()));
        } else {
            println!("COULD NOT WRITE YET");
            return Poll::Running;
        }
    }

    fn send(&mut self, value: T) -> Result<(), Self::Error> {
        match self.inner.replace(None) {
            Some(elem) => {
                self.inner.replace(Some(elem));
                return Err(SendError::NotEmpty(value));
            }
            None => {
                self.inner.set(Some(value));
                self.is_empty.set(false);
                Ok(())
            }
        }
    }
}
