#![allow(non_snake_case)]
use super::{Future, Poll};

enum MaybeDone<Fut: Future> {
    Future(Fut),
    Done(Fut::Output),
    Gone,
}

fn maybe_done<Fut: Future>(future: Fut) -> MaybeDone<Fut> {
    MaybeDone::Future(future)
}

impl<Fut> MaybeDone<Fut>
where
    Fut: Future,
{
    pub fn take_output(&mut self) -> Option<Fut::Output> {
        match self {
            Self::Done(_) => {}
            Self::Future(_) | Self::Gone => return None,
        };
        if let Self::Done(output) = core::mem::replace(self, MaybeDone::Gone) {
            Some(output)
        } else {
            unreachable!()
        }
    }
}

impl<Fut: Future> Future for MaybeDone<Fut> {
    type Output = ();

    fn poll(&mut self) -> Poll<Self::Output> {
        let res = match self {
            Self::Future(f) => ready!(f.poll()),
            Self::Done(_) => return Poll::Ready(()),
            Self::Gone => panic!("MaybeDone polled after value taken"),
        };

        *self = MaybeDone::Done(res);
        Poll::Ready(())
    }
}

macro_rules! generate {
    ($(
        ($Join:ident, <$($Fut:ident),*>),
    )*) => ($(
        pub struct $Join<$($Fut: Future),*> {
            $($Fut: MaybeDone<$Fut>,)*
        }

        impl<$($Fut: Future),*> $Join<$($Fut),*> {
            pub(crate) fn new($($Fut: $Fut),*) -> $Join<$($Fut),*> {
                $Join {
                    $($Fut: maybe_done($Fut)),*
                }
            }
        }

        impl<$($Fut: Future),*> Future for $Join<$($Fut),*> {
            type Output = ($($Fut::Output),*);

            fn poll(&mut self) -> Poll<Self::Output> {
                let mut all_done = true;
                $(
                    all_done &= self.$Fut.poll().is_ready();
                )*

                if all_done {
                    Poll::Ready(($(self.$Fut.take_output().unwrap()), *))
                } else {
                    Poll::Running
                }
            }
        }
    )*)
}

generate! {
    (Join, <Fut1, Fut2>),

    (Join3, <Fut1, Fut2, Fut3>),

    (Join4, <Fut1, Fut2, Fut3, Fut4>),

    (Join5, <Fut1, Fut2, Fut3, Fut4, Fut5>),
}
// pub struct Join<F1: Future, F2: Future> {
//     f1: MaybeDone<F1>,
//     f2: MaybeDone<F2>,
// }

// impl <F1, F2> Join<F1, F2>
// where
//     F1: Future,
//     F2: Future,
// {
//     pub fn new(f1: F1, f2: F2) -> Self {
//         Self {
//             f1: MaybeDone::Future(f1),
//             f2: MaybeDone::Future(f2),
//         }
//     }
// }

// impl <F1, F2> Future for Join<F1, F2>
// where
//     F1: Future,
//     F2: Future,
// {
//     type Output = (F1::Output, F2::Output);

//     fn poll(&mut self) -> Poll<Self::Output> {
//         let mut all_done = true;

//         all_done &= self.f1.poll().is_ready();
//         all_done &= self.f1.poll().is_ready();

//         if all_done {
//             Poll::Ready((self.f1.take_output().unwrap(), self.f2.take_output().unwrap()))
//         } else {
//             Poll::Running
//         }
//     }
// }
