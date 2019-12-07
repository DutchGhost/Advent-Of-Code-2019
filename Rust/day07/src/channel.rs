use std::cell::Cell;

pub struct Channel<T> {
    inner: Cell<Option<T>>,
}

impl<T> Channel<T> {
    pub const fn new(value: T) -> Self {
        Self {
            inner: Cell::new(Some(value)),
        }
    }

    pub fn split<'a>(&'a self) -> (Tx<'a, T>, Rx<'a, T>) {
        (Tx::new(&self.inner), Rx::new(&self.inner))
    }
}

#[derive(Copy)]
pub struct Tx<'a, T> {
    inner: &'a Cell<Option<T>>,
}

impl<'a, T: Copy + std::fmt::Debug> std::fmt::Debug for Tx<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.inner.get().fmt(f)
    }
}

impl<'a, T> Clone for Tx<'a, T> {
    fn clone(&self) -> Self {
        Self { ..*self }
    }
}

impl<'a, T> Tx<'a, T> {
    pub fn new(inner: &'a Cell<Option<T>>) -> Self {
        Self { inner }
    }

    pub fn send(&self, value: T) -> Result<(), ()> {
        match self.inner.replace(Some(value)) {
            Some(_) => Err(()),
            None => Ok(()),
        }
    }
}

#[derive(Copy)]
pub struct Rx<'a, T> {
    inner: &'a Cell<Option<T>>,
}

impl<'a, T: Copy + std::fmt::Debug> std::fmt::Debug for Rx<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.inner.get().fmt(f)
    }
}

impl<'a, T> Clone for Rx<'a, T> {
    fn clone(&self) -> Self {
        Self { ..*self }
    }
}

impl<'a, T> Rx<'a, T> {
    pub fn new(inner: &'a Cell<Option<T>>) -> Self {
        Self { inner }
    }

    pub fn recv(&self) -> Option<T> {
        self.inner.replace(None)
    }
}
