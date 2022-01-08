use std::collections::HashSet;
use std::hash::Hash;

pub fn has_duplicates<T>(data: T) -> bool
    where
        T: IntoIterator,
        T::Item: Eq + Hash
{
    let mut items = HashSet::new();
    !data.into_iter().all(move |x| items.insert(x))
}
