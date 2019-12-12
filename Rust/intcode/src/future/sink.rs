use super::Poll;

pub trait Sink<Item> {
    type Error;

    fn poll_ready(&mut self) -> Poll<Result<(), Self::Error>>;
    fn send(&mut self, item: Item) -> Result<(), Self::Error>;
}

impl<S: ?Sized, Item> Sink<Item> for &mut S
where
    S: Sink<Item>,
{
    type Error = S::Error;
    fn poll_ready(&mut self) -> Poll<Result<(), Self::Error>> {
        (**self).poll_ready()
    }

    fn send(&mut self, item: Item) -> Result<(), Self::Error> {
        (**self).send(item)
    }
}

/// Stdout just contains the last value that was written to it
pub struct Stdout<T> {
    inner: Option<T>,
}

impl<T> Stdout<T> {
    pub const fn new() -> Self {
        Self { inner: None }
    }

    pub fn into_inner(self) -> Option<T> {
        self.inner
    }
}

impl<T> Sink<T> for Stdout<T> {
    type Error = !;

    fn poll_ready(&mut self) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn send(&mut self, item: T) -> Result<(), Self::Error> {
        self.inner = Some(item);
        Ok(())
    }
}
