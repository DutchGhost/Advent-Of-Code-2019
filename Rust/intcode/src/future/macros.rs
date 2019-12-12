#[macro_export]
macro_rules! ready {
    ($e:expr) => {
        match $e {
            $crate::future::Poll::Ready(t) => t,
            $crate::future::Poll::Running => return $crate::future::Poll::Running,
        }
    };
}

#[macro_export]
macro_rules! try_ready {
    ($e:expr) => {
        match $e {
            $crate::future::Poll::Ready(Ok(t)) => t,
            $crate::future::Poll::Ready(Err(e)) => {
                return $crate::future::Poll::Ready(Err(e.into()))
            }
            $crate::future::Poll::Running => return $crate::future::Poll::Running,
        }
    };
}

#[macro_export]
macro_rules! try_unwrap {
    ($e:expr) => {
        match $e {
            Ok(ok) => ok,
            Err(e) => return $crate::future::Poll::Ready(Err(e.into())),
        }
    };
}
