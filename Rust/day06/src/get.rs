use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

pub trait Get<K: ?Sized> {
    type Output;

    fn get(&self, key: &K) -> Option<Self::Output>;
}

impl<'a, K, V: 'a, Q> Get<Q> for &'a HashMap<K, V>
where
    K: Borrow<Q> + Hash + Eq,
    Q: Hash + Eq,
{
    type Output = &'a V;

    fn get(&self, key: &Q) -> Option<Self::Output> {
        HashMap::get(self, &key)
    }
}
