use std::{
    cmp::Eq,
    collections::{HashMap, HashSet},
    hash::Hash,
};

pub trait Orbitter {
    type Orbit;

    fn empty() -> Self;

    fn add(&mut self, orbitter: Self::Orbit);

    fn remove(&mut self, orbitter: &Self::Orbit);
}

impl<O: Hash + Eq> Orbitter for HashSet<O> {
    type Orbit = O;

    fn empty() -> Self {
        Self::new()
    }

    fn add(&mut self, orbitter: Self::Orbit) {
        self.insert(orbitter);
    }

    fn remove(&mut self, orbitter: &Self::Orbit) {
        self.remove(orbitter);
    }
}

pub struct OrbitGraph<T, O> {
    graph: HashMap<T, O>,
}

impl<T: Eq + Hash, O: Orbitter> OrbitGraph<T, O> {
    pub fn new() -> Self {
        Self {
            graph: HashMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.graph.is_empty()
    }

    pub fn orbit_index(&self, orbit: &T) -> Option<&O> {
        self.graph.get(orbit)
    }

    pub fn orbit(&mut self, orbittee: T) -> &mut O {
        self.graph.entry(orbittee).or_insert_with(O::empty)
    }

    pub fn values<'a>(&'a self) -> impl Iterator<Item = &'a O> + 'a {
        self.graph.values()
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (&'a T, &'a O)> + 'a {
        self.graph.iter()
    }

    pub fn remove_orbit(&mut self, orbittee: &T) {
        self.graph.remove(orbittee);
    }
}
