use std::{
    cell::Cell,
    fmt::{self, Debug},
};

use super::{Rx, Tx};

/// A multiple consumer, multiple producer, single-threaded channel
/// with storage for only 1 value. A channer can be split into its
/// Transmitter (Tx), and Reciver (Rx) usiing the [`Channel::split`]
/// method.
pub struct Channel<T> {
    pub inner: Cell<Option<T>>,
    pub is_empty: Cell<bool>,
}

impl<T> Channel<T> {
    #[inline]
    pub const fn new(value: T) -> Self {
        Self {
            inner: Cell::new(Some(value)),
            is_empty: Cell::new(false),
        }
    }

    pub const fn empty() -> Self {
        Self {
            inner: Cell::new(None),
            is_empty: Cell::new(true),
        }
    }

    #[inline]
    pub const fn split<'a>(&'a self) -> (Tx<'a, T>, Rx<'a, T>) {
        (
            Tx::new(&self.inner, &self.is_empty),
            Rx::new(&self.inner, &self.is_empty),
        )
    }
}

impl<'a, T: Copy + Debug> Debug for Channel<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}
