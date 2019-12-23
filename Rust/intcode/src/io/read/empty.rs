use std::marker::PhantomData;

use crate::device::Poll;
use crate::io::Read;

#[inline(always)]
pub const fn empty<T: ?Sized>() -> Empty<T> {
    Empty::new()
}

/// A reader that never yields any items.
/// Created by the [`empty`] function.
pub struct Empty<T: ?Sized> {
    _phantom: PhantomData<T>
}

impl <T: ?Sized> Empty<T> {
    
    #[inline(always)]
    const fn new() -> Self {
        Self { _phantom: PhantomData }
    }
}

impl <T> Read for Empty<T> {
    type Item = T;

    fn poll_read(&mut self) -> Poll<Option<Self::Item>> {
        Poll::Ready(None)
    }
}