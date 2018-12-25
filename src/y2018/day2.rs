use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::util::{Answer, IterExtra, Lines, Result};

pub struct Checksum(pub [usize; 2]);

impl Answer for Checksum {
    type Input = Lines;
    type Output = usize;
    fn ans(&self, inputs: impl Iterator<Item = String>) -> Result<Self::Output> {
        let checksum = inputs
            .map(|id| {
                id.chars()
                    .frequencies()
                    .into_iter()
                    .map(|(_, f)| f)
                    .collect::<HashSet<_>>()
            })
            .fold(HashMap::new(), |mut freq_count, freq_set| {
                for freq in &self.0 {
                    if freq_set.contains(freq) {
                        *freq_count.entry(freq).or_insert(0) += 1;
                    }
                }
                freq_count
            })
            .values()
            .product();
        Ok(checksum)
    }
}

pub struct ExactDiff(pub usize);

impl Answer for ExactDiff {
    type Input = Lines;
    type Output = String;
    fn ans(&self, inputs: impl Iterator<Item = String>) -> Result<Self::Output> {
        inputs
            .make_clonable()
            .tuple_combinations()
            .filter_map(|(id1, id2)| {
                if id1.len() != id2.len() {
                    return None;
                }
                let common_chars = || {
                    id1.chars()
                        .zip(id2.chars())
                        .filter_map(|(a, b)| if a == b { Some(a) } else { None })
                };
                if common_chars().count() != id1.len() - self.0 {
                    return None;
                }
                Some(common_chars().collect())
            })
            .single()
    }
}
