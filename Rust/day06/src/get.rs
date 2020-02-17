use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

/// Given a reference to `K`,
/// finds the corresponding element
/// in a container.
pub trait Get<K: ?Sized> {
    type Output;

    fn get(&self, key: &K) -> Option<&Self::Output>;
}

impl<K: ?Sized, T> Get<K> for &'_ T
where
    T: Get<K>,
{
    type Output = T::Output;

    #[inline(always)]
    fn get(&self, key: &K) -> Option<&Self::Output> {
        (&**self).get(key)
    }
}

impl<K, V, Q: ?Sized> Get<Q> for HashMap<K, V>
where
    K: Borrow<Q> + Hash + Eq,
    Q: Hash + Eq,
{
    type Output = V;

    #[inline(always)]
    fn get(&self, key: &Q) -> Option<&Self::Output> {
        HashMap::get(self, &key)
    }
}
