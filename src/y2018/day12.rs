use std::collections::HashSet;

use crate::util::{Answer, Lines, Result};

fn parse(c: char) -> bool {
    c == '#'
}

fn get(state: &[bool], i: i64) -> bool {
    if i < 0 {
        return false;
    }
    *state.get(i as usize).unwrap_or(&false)
}

type Mapping = HashSet<Vec<bool>>;

fn new_mapping(inputs: impl Iterator<Item = String>) -> Mapping {
    inputs
        .map(|input| input.chars().take(5).map(parse).collect())
        .collect()
}

fn has_plant(m: &Mapping, state: &[bool], i: i64) -> bool {
    let key: Vec<_> = ((i - 2)..=(i + 2)).map(|idx| get(state, idx)).collect();
    m.contains(&key)
}

const INITIAL: &str = "#.#.#....##...##...##...#.##.#.###...#.##...#....#.#...#.##.........#.#...#..##.#.....#..#.###";

pub struct PlantSum(pub i64);
impl Answer for PlantSum {
    type Input = Lines;
    type Output = i64;
    fn ans(&self, inputs: impl Iterator<Item = String>) -> Result<Self::Output> {
        let mut state: Vec<bool> = INITIAL.chars().map(parse).collect();
        let mut num_before: i64 = 0;
        let mut len = state.len() as i64;

        let mapping = new_mapping(inputs);

        for _ in 0..self.0 {
            state = ((-2)..(len + 2))
                .map(|i| has_plant(&mapping, &state, i))
                .collect();

            num_before += 2;
            len += 4;
        }

        // Long term pattern is: SUM = 508 + 20*G

        let sum: i64 = state
            .iter()
            .enumerate()
            .filter_map(|(i, &plant)| {
                if plant {
                    Some(i as i64 - num_before)
                } else {
                    None
                }
            })
            .sum();
        Ok(sum)
    }
}
