use std::{
    collections::{hash_map::Entry, HashMap},
    hash::Hash,
    vec,
};

use crate::util::{Error, Result};

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct Duplicates<I: Iterator> {
    iter: I,
    seen: HashMap<I::Item, ()>,
}

impl<I: Iterator> Iterator for Duplicates<I>
where
    I::Item: Eq + Hash,
{
    type Item = I::Item;

    #[inline]
    // TODO: use Set Entry API (https://github.com/rust-lang/rfcs/issues/1490)
    fn next(&mut self) -> Option<I::Item> {
        loop {
            match self.seen.entry(self.iter.next()?) {
                Entry::Occupied(o) => return Some(o.replace_key()),
                Entry::Vacant(v) => v.insert(()),
            };
        }
    }
}

pub trait Extra: Iterator + Sized {
    fn single(mut self) -> Result<Self::Item> {
        match (self.next(), self.next()) {
            (None, _) => Err(Error::Custom("Expected one, got none")),
            (Some(item), None) => Ok(item),
            (Some(_), Some(_)) => Err(Error::Custom("Expected one, got many")),
        }
    }

    fn make_clonable(self) -> vec::IntoIter<Self::Item> {
        self.collect::<Vec<_>>().into_iter()
    }

    fn duplicates(self) -> Duplicates<Self>
    where
        Self::Item: Eq + Hash,
    {
        Duplicates {
            iter: self,
            seen: HashMap::new(),
        }
    }

    fn only_uniques_by<V: Eq + Hash>(self, mut f: impl FnMut(&Self::Item) -> V) -> Vec<Self::Item> {
        let mut seen = HashMap::new();
        for item in self {
            match seen.entry(f(&item)) {
                Entry::Occupied(mut o) => {
                    o.insert(None);
                }
                Entry::Vacant(v) => {
                    v.insert(Some(item));
                }
            };
        }
        seen.into_iter().flat_map(|(_, v)| v).collect()
    }

    fn frequencies(self) -> HashMap<Self::Item, usize>
    where
        Self::Item: Eq + Hash,
    {
        self.fold(HashMap::new(), |mut acc, i| {
            *acc.entry(i).or_insert(0) += 1;
            acc
        })
    }
}

impl<I: Iterator> Extra for I {}
