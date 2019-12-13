use super::future::Poll;

pub trait Stream {
    type Item;

    fn poll_next(&mut self) -> Poll<Option<Self::Item>>;
}

impl <S: ?Sized> Stream for &mut S
where
    S: Stream
{
    type Item = S::Item;

    fn poll_next(&mut self) -> Poll<Option<Self::Item>> {
        (**self).poll_next()
    }
}

impl<S: ?Sized> StreamExt for S where S: Stream {}

pub trait StreamExt: Stream {
    fn chain<S>(self, other: S) -> Chain<Self, S>
    where
        S: Stream<Item = Self::Item>,
        Self: Sized,
    {
        Chain::new(self, other)
    }
}

pub struct Once<T> {
    item: Option<T>,
}

pub fn once<T>(item: T) -> Once<T> {
    Once { item: Some(item) }
}

impl<T> Stream for Once<T> {
    type Item = T;

    fn poll_next(&mut self) -> Poll<Option<Self::Item>> {
        Poll::Ready(self.item.take())
    }
}

pub struct Empty<T> {
    inner: std::marker::PhantomData<T>,
}

pub fn empty<T>() -> Empty<T> {
    Empty { inner: std::marker::PhantomData }
}

impl <T> Stream for Empty<T> {
    type Item = T;
    
    fn poll_next(&mut self) -> Poll<Option<Self::Item>> {
        Poll::Ready(None)
    }
}

pub struct Chain<S1, S2> {
    first: Option<S1>,
    second: S2,
}

impl<S1, S2> Chain<S1, S2> {
    fn new(first: S1, second: S2) -> Self {
        Self {
            first: Some(first),
            second,
        }
    }
}

impl<S1, S2> Stream for Chain<S1, S2>
where
    S1: Stream,
    S2: Stream<Item = S1::Item>,
{
    type Item = S1::Item;

    fn poll_next(&mut self) -> Poll<Option<Self::Item>> {
        if let Some(first) = self.first.as_mut() {
            if let Some(item) = ready!(first.poll_next()) {
                return Poll::Ready(Some(item));
            }
        }

        self.first = None;
        self.second.poll_next()
    }
}
