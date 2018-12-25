use crate::util::{Answer, ByWhitespace, Lines, Result};

struct Recipies {
    scores: Vec<u8>,
    active: [usize; 2],
}

impl Recipies {
    fn new() -> Self {
        Self {
            scores: vec![3, 7],
            active: [0, 1],
        }
    }

    fn combine(&mut self) {
        let s: u8 = self.active.iter().map(|&i| self.scores[i]).sum();

        if s >= 10 {
            self.scores.push(s / 10);
            self.scores.push(s % 10);
        } else {
            self.scores.push(s)
        }

        for i in &mut self.active {
            *i = (*i + self.scores[*i] as usize + 1) % self.scores.len();
        }
    }
}

pub struct ScoreList(pub usize);
impl Answer for ScoreList {
    type Input = ByWhitespace<usize>;
    type Output = String;
    fn ans(&self, mut inputs: impl Iterator<Item = usize>) -> Result<Self::Output> {
        let input = inputs.next()?;

        let mut recipies = Recipies::new();

        while recipies.scores.len() < input + self.0 {
            recipies.combine()
        }

        let output = recipies.scores[input..input + self.0]
            .iter()
            .map(|d| -> char { (d + b'0').into() })
            .collect();
        Ok(output)
    }
}

pub struct FirstOccurance;
impl Answer for FirstOccurance {
    type Input = Lines;
    type Output = usize;
    fn ans(&self, mut inputs: impl Iterator<Item = String>) -> Result<Self::Output> {
        let needle: Vec<u8> = inputs.next()?.bytes().map(|b| b - b'0').collect();

        let mut recipies = Recipies::new();
        loop {
            if needle.len() < recipies.scores.len() {
                let c = recipies.scores.len() - needle.len();
                for &cand in &[c, c - 1] {
                    if recipies.scores[cand..].starts_with(&needle) {
                        return Ok(cand);
                    }
                }
            }
            recipies.combine()
        }
    }
}
