use crate::device::Poll;

mod empty;
pub use empty::{Empty, empty};

mod chain;
pub use chain::Chain;

/// Read a value
/// 
/// This trait is used to read value's.
pub trait Read {
    /// The type being read.
    type Item;

    /// Reads an item.
    /// 
    /// # Implementation
    /// This function may not panic.
    /// An implementor should return `Poll::Ready(Some(item))` on a succesfull read,
    /// `Poll::Pending` if there may be an item to read in the future, and
    /// `Poll::Ready(None)` otherwise.
    fn poll_read(&mut self) -> Poll<Option<Self::Item>>;
}

impl <R: Read + ?Sized> Read for &mut R {
    type Item = R::Item;

    fn poll_read(&mut self) -> Poll<Option<Self::Item>> {
        (&mut **self).poll_read()
    }
}

impl <R> ReadExt for R where R: Read {}

pub trait ReadExt: Read {
    /// Adapter for chaining two readers.
    /// 
    /// The resulting Reader emits elements from the first reader,
    /// and when the first reader reaches the end, emits selements from
    /// the second reader.
    fn chain<O>(self, other: O) -> Chain<Self, O>
    where
        Self: Sized,
    {
        Chain::new(self, other)
    }
}