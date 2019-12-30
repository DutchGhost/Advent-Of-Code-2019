use std::mem;

pub struct Grouped<'a, T: 'a> {
    slice: &'a [T],
}

impl<'a, T: 'a> Grouped<'a, T> {
    pub fn groups(slice: &'a [T]) -> Self {
        Self { slice }
    }
}

impl<'a, T: Ord + 'a> Iterator for Grouped<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.slice.is_empty() {
            return None;
        }

        let mvd = mem::take(&mut self.slice);

        let (fst, res) = mvd.split_first()?;

        let count = res.iter().take_while(|elem| *elem == fst).count();

        let (ret, rest) = mvd.split_at(count + 1);

        self.slice = rest;

        Some(ret)
    }
}
