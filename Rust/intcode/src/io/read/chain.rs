use crate::io::{Read};

use crate::ready2;
use crate::device::{Poll};

/// Chain's 
pub struct Chain<R1, R2> {
    first: Option<R1>,
    second: R2,
}

impl<R1, R2> Chain<R1, R2> {
    pub(crate) fn new(first: R1, second: R2) -> Self {
        Self {
            first: Some(first),
            second,
        }
    }
}

impl<R1, R2> Read for Chain<R1, R2>
where
    R1: Read,
    R2: Read<Item = R1::Item>,
{
    type Item = R1::Item;

    fn poll_read(&mut self) -> Poll<Option<Self::Item>> {
        if let Some(first) = self.first.as_mut() {
            if let Some(item) = ready2!(first.poll_read()) {
                return Poll::Ready(Some(item));
            }
        }

        self.first = None;
        self.second.poll_read()
    }
}
