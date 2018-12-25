use std::{collections::HashMap, str::FromStr};

use nom::*;

use crate::util::{number, to_result, Answer, ByLine, Error, Result};

pub enum Entry {
    Start(usize),
    Down(usize),
    Up(usize),
}

named!(timestamp<&str,usize>, do_parse!(
    tag!("[") >> digit >> tag!("-") >> digit >> tag!("-") >> digit >>
    tag!(" ") >> digit >> tag!(":") >> minute: number >> tag!("] ") >>
    (minute)
));

named!(start<&str, Entry>, do_parse!(
    timestamp >> tag!("Guard #") >> n: number >> tag!(" begins shift") >>
    (Entry::Start(n))
));

named!(down<&str, Entry>, do_parse!(
    minute: timestamp >> tag!("falls asleep") >> (Entry::Down(minute))
));

named!(up<&str, Entry>, do_parse!(
    minute: timestamp >> tag!("wakes up") >> (Entry::Up(minute))
));

named!(entry<&str,Entry>, alt!(start | down | up));

impl FromStr for Entry {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        to_result(entry(s))
    }
}

struct Guard {
    total: usize,
    counts: [usize; 60],
}

impl Default for Guard {
    fn default() -> Self {
        Self {
            total: 0,
            counts: [0; 60],
        }
    }
}

fn guards(inputs: impl Iterator<Item = Entry>) -> Result<HashMap<usize, Guard>> {
    let mut current = 0;
    let mut down = None;

    let mut guards = HashMap::<usize, Guard>::new();

    for input in inputs {
        match input {
            Entry::Start(g) => {
                current = g;
                down = None;
            }
            Entry::Down(t) => {
                if down.is_some() {
                    return Err(Error::Custom("Multiple sleep events without wake"));
                }
                down = Some(t);
            }
            Entry::Up(end) => {
                let start = down.ok_or(Error::Custom("Multiple wake event without sleep"))?;
                down = None;
                let guard = guards.entry(current).or_default();
                guard.total += end - start;
                for minute in start..end {
                    guard.counts[minute] += 1;
                }
            }
        }
    }

    Ok(guards)
}

const NO_GUARDS: Error = Error::Custom("No guards fell asleep");

pub struct SleepyMinute;
impl Answer for SleepyMinute {
    type Input = ByLine<Entry>;
    type Output = usize;
    fn ans(&self, inputs: impl Iterator<Item = Entry>) -> Result<Self::Output> {
        let guards = guards(inputs)?;

        let (id, guard) = guards
            .into_iter()
            .max_by_key(|(_, g)| g.total)
            .ok_or(NO_GUARDS)?;

        let (minute, _) = guard
            .counts
            .iter()
            .enumerate()
            .max_by_key(|(_, &count)| count)
            .ok_or(NO_GUARDS)?;

        Ok(id * minute)
    }
}

pub struct SleepyGuard;
impl Answer for SleepyGuard {
    type Input = ByLine<Entry>;
    type Output = usize;
    fn ans(&self, inputs: impl Iterator<Item = Entry>) -> Result<Self::Output> {
        let guards = guards(inputs)?;

        let (id, minute, _) = guards
            .iter()
            .flat_map(|(&id, g)| g.counts.iter().enumerate().map(move |(m, &c)| (id, m, c)))
            .max_by_key(|&(_, _, c)| c)
            .ok_or(NO_GUARDS)?;

        Ok(id * minute)
    }
}
