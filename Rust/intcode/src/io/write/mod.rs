use crate::device::Poll;

/// Write a value.
/// 
/// This trait is used to write value's.
pub trait Write<T> {
    /// The error a write can return.
    type Error;
    
    /// Attempts to write a value.
    /// It has 3 possible return values:
    /// - `Poll::Ready(Ok(()))`, indicating a succesfull write,
    /// - `Poll::Ready(Err(e))` indicating a write that didn't succeed,
    /// - `Poll::Pending` indicating a write couldn't be done yet.
    fn poll_write(&mut self, value: T) -> Poll<Result<(), Self::Error>>;
}

impl <W: Write<T> + ?Sized, T> Write<T> for &mut W {
    type Error = W::Error;

    fn poll_write(&mut self, value: T) -> Poll<Result<(), Self::Error>> {
        (&mut **self).poll_write(value)
    }
}