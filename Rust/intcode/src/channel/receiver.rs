use std::{
    cell::Cell,
    fmt::{self, Debug},
};

use crate::future::{stream::Stream, Poll};

/// The receiving end of a channel.
#[derive(Copy)]
pub struct Rx<'a, T> {
    inner: &'a Cell<Option<T>>,
    is_empty: &'a Cell<bool>,
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
    pub const fn new(inner: &'a Cell<Option<T>>, is_empty: &'a Cell<bool>) -> Self {
        Self { inner, is_empty }
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty.get()
    }
}

impl<'a, T> Stream for Rx<'a, T> {
    type Item = T;

    fn poll_next(&mut self) -> Poll<Option<Self::Item>> {
        if self.is_empty() {
            Poll::Running
        } else {
            let value = self.inner.replace(None);
            assert!(value.is_some());
            self.is_empty.set(true);
            Poll::Ready(value)
        }
    }
}
