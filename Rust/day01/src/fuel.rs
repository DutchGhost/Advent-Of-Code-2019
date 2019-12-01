/// Describes how to calculate the fuel needed.
pub trait FuelTransform: Sized {
    fn transform(&self) -> Option<Self>;
}

impl FuelTransform for i64 {
    fn transform(&self) -> Option<Self> {
        match (self / 3) - 2 {
            n if n > 0 => Some(n),
            _ => None,
        }
    }
}

pub trait Fuel: FuelTransform + Copy {
    fn fuel(&mut self) -> Option<Self> {
        *self = self.transform()?;
        Some(*self)
    }
}

impl<T: FuelTransform + Copy> Fuel for T {}
