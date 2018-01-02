use std::collections::HashSet;
use std::hash::Hash;
use std::io::BufRead;
use itertools::Itertools;
use util::{force_lines, Ans, IterUtil};

pub trait Validator {
    fn is_valid<'a>(password: impl Iterator<Item = &'a str>) -> bool;
}

fn is_unique<T: Hash + Eq>(mut items: impl Iterator<Item = T>) -> bool {
    let mut set = HashSet::new();
    items.all(|item| set.insert(item))
}

pub struct NoDuplicates;
impl Validator for NoDuplicates {
    fn is_valid<'a>(password: impl Iterator<Item = &'a str>) -> bool {
        is_unique(password)
    }
}

pub struct NoAnagrams;
impl Validator for NoAnagrams {
    fn is_valid<'a>(password: impl Iterator<Item = &'a str>) -> bool {
        is_unique(password.map(|word| {
            let mut bytes = word.bytes().collect_vec();
            bytes.sort();
            bytes
        }))
    }
}

impl<T: Validator> Ans<Phantom> for T {
    type Value = usize;
    fn compute(&self, r: impl BufRead) -> usize {
        force_lines(r).count_which(|line| Self::is_valid(line.split_whitespace()))
    }
}

#[allow(dead_code)]
pub struct Phantom;
