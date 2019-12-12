use super::join::{Join, Join3, Join4, Join5};

pub enum Poll<T> {
    Ready(T),
    Running,
}

impl<T> Poll<T> {
    pub fn is_ready(&self) -> bool {
        match self {
            Self::Ready(_) => true,
            _ => false,
        }
    }
}

pub trait Future {
    type Output;

    fn poll(&mut self) -> Poll<Self::Output>;
}

pub trait FutureExt: Future {
    fn join<F: Future>(self, other: F) -> Join<Self, F>
    where
        Self: Sized,
    {
        Join::new(self, other)
    }

    fn join3<F2, F3>(self, f2: F2, f3: F3) -> Join3<Self, F2, F3>
    where
        Self: Sized,
        F2: Future,
        F3: Future,
    {
        Join3::new(self, f2, f3)
    }

    fn join4<F2, F3, F4>(self, f2: F2, f3: F3, f4: F4) -> Join4<Self, F2, F3, F4>
    where
        Self: Sized,
        F2: Future,
        F3: Future,
        F4: Future,
    {
        Join4::new(self, f2, f3, f4)
    }

    fn join5<F2, F3, F4, F5>(self, f2: F2, f3: F3, f4: F4, f5: F5) -> Join5<Self, F2, F3, F4, F5>
    where
        Self: Sized,
        F2: Future,
        F3: Future,
        F4: Future,
        F5: Future,
    {
        Join5::new(self, f2, f3, f4, f5)
    }

    fn execute(&mut self) -> Self::Output {
        loop {
            match self.poll() {
                Poll::Running => continue,
                Poll::Ready(r) => return r,
            }
        }
    }
}

impl<F: ?Sized> FutureExt for F where F: Future {}
