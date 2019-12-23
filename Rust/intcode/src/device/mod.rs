pub mod macros;

pub enum Poll<T> {
    Ready(T),
    Pending,
}

pub trait Device {
    type Output;

    fn poll(&mut self) -> Poll<Self::Output>;
}