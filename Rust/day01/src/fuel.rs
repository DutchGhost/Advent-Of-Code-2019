pub trait FuelTransform: Sized {
    fn calculate(&self) -> Option<Self>;
}

impl FuelTransform for i64 {
    fn calculate(&self) -> Option<Self> {
        match (self / 3) - 2 {
            n if n > 0 => Some(n),
            _ => None,
        }
    }
}

pub trait Fuel: Copy {
    fn fuel(&self) -> FuelIterator<Self> {
        FuelIterator { fuel: Some(*self) }
    }
}

impl<T: Copy> Fuel for T {}

pub struct FuelIterator<N> {
    fuel: Option<N>,
}

impl<N: FuelTransform + Copy> Iterator for FuelIterator<N> {
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        match self.fuel {
            None => None,
            Some(fuel) => {
                self.fuel = fuel.calculate();
                self.fuel
            }
        }
    }
}
