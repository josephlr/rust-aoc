use crate::util::{Answer, Bytes, Result};

fn react(b1: u8, b2: u8) -> bool {
    b1.eq_ignore_ascii_case(&b2) && b1 != b2
}

fn react_len(inputs: impl Iterator<Item = u8>) -> usize {
    let mut to_react = Vec::new();
    for b2 in inputs {
        if to_react.last().map_or(false, |&b1| react(b1, b2)) {
            to_react.pop();
        } else {
            to_react.push(b2);
        }
    }
    to_react.len()
}

pub struct FullReact;
impl Answer for FullReact {
    type Input = Bytes;
    type Output = usize;
    fn ans(&self, inputs: impl Iterator<Item = u8>) -> Result<Self::Output> {
        Ok(react_len(inputs.filter(|&c| c != b'\n')))
    }
}

pub struct BestReact;
impl Answer for BestReact {
    type Input = Bytes;
    type Output = usize;
    fn ans(&self, inputs: impl Iterator<Item = u8>) -> Result<Self::Output> {
        let input: Vec<u8> = inputs.filter(|&c| c != b'\n').collect();
        let alphabet = b'a'..=b'z';

        let l = alphabet
            .map(|c| {
                react_len(
                    input
                        .iter()
                        .cloned()
                        .filter(|b| b.to_ascii_lowercase() != c),
                )
            })
            .min()
            .unwrap();
        Ok(l)
    }
}
