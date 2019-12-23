use super::{Read, Write, Poll};

/// A socket combines a [`Read`] -er and a [`Write`] -er into one.
/// The Sockets acts as a way to combine any reader and writer into
/// one structure that is both readable and writable.
/// 
/// The socket will simple delegate any read calls to its reader,
/// and any write calls to its writer.
pub struct Socket<R, W> {
    reader: R,
    writer: W,
}

impl <R, W> Socket<R, W> {
    pub fn new(reader: R, writer: W) -> Self {
        Self { reader, writer }
    }
}

impl <R, W> Read for Socket<R, W>
where
    R: Read
{
    type Item = R::Item;

    #[inline]
    fn poll_read(&mut self) -> Poll<Option<Self::Item>> {
        Read::poll_read(&mut self.reader)
    }
}

impl <R, W, T> Write<T> for Socket<R, W>
where
    W: Write<T>
{
    type Error = W::Error;

    #[inline]
    fn poll_write(&mut self, value: T) -> Poll<Result<(), Self::Error>> {
        Write::poll_write(&mut self.writer, value)
    } 
}