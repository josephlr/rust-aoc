use std::io::BufRead;
use std::collections::HashSet;
use util::{force_lines, Ans, IterUtil};

pub struct TwoOrThree;
impl Ans for TwoOrThree {
    type Value = i32;
    fn compute(&self, r: impl BufRead) -> i32 {
        let (c2, c3) = force_lines(r).map(|id| {
            let f = id.chars().frequencies();
            (f.values().any(|&x| x == 2), f.values().any(|&x| x == 3))
        }).fold((0,0), |(acc2, acc3), (i2, i3)| {
            (if i2 { acc2 + 1} else {acc2}, if i3 { acc3 + 1} else {acc3})
        });
        c2 * c3
    }
}

fn matches(id1: &str, id2: &str) -> Option<String> {
    let common_chars = || {
        id1.chars().zip(id2.chars()).filter_map(|(a,b)| if a == b {Some(a)} else {None})
    };
    if common_chars().count() == id1.len() - 1 {
        Some(common_chars().collect())
    } else {
        None
    }
}

pub struct Common;
impl Ans for Common {
    type Value = String;
    fn compute(&self, r: impl BufRead) -> String {
        let mut seen: HashSet<String> = HashSet::new();
        for id1 in force_lines(r) {
            for id2 in seen.iter() {
                if let Some(ans) = matches(&id1, &id2) {
                    return ans;
                }
            }
            seen.insert(id1);
        }
        panic!("Could not find matching IDs")
    }
}