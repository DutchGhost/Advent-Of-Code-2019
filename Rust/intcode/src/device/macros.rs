#[macro_export]
macro_rules! ready2 {
    ($e:expr) => {
        match $e {
            $crate::device::Poll::Ready(t) => t,
            $crate::device::Poll::Pending => return $crate::device::Poll::Pending,
        }
    };
}