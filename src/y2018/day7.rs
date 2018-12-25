use std::{
    cmp::Reverse,
    collections::{hash_map::Entry, BinaryHeap, HashMap},
    hash::Hash,
    str::FromStr,
};

use nom::*;

use crate::util::{to_result, Answer, ByLine, Error, Result};

pub struct Dependancy<J> {
    first: J,
    next: J,
}

named!(dependancy<&str,Dependancy<String>>, do_parse!(
    tag!("Step ") >> first: map!(alphanumeric, String::from) >>
    tag!(" must be finished before step ") >>
    next: map!(alphanumeric, String::from) >> tag!(" can begin.") >>
    (Dependancy{first, next})
));

impl FromStr for Dependancy<String> {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        to_result(dependancy(s))
    }
}

struct Scheduler<J: Eq + Ord + Hash> {
    depends_on: HashMap<J, Vec<J>>,
    remaining_deps: HashMap<J, usize>,
    ready: BinaryHeap<Reverse<J>>,
}

impl<J: Eq + Ord + Hash> Scheduler<J> {
    // Inputs should be deduplicated before being passed
    fn new(inputs: impl Iterator<Item = Dependancy<J>>) -> Self
    where
        J: Clone,
    {
        let mut depends_on = HashMap::<J, Vec<J>>::new();
        let mut remaining_deps = HashMap::<J, usize>::new();
        for Dependancy { first, next } in inputs {
            let nexts = depends_on.entry(first).or_default();
            nexts.push(next);
            let next = nexts.last().unwrap();
            if let Some(count) = remaining_deps.get_mut(next) {
                *count += 1;
            } else {
                remaining_deps.insert(next.clone(), 1);
            }
        }

        let ready = depends_on
            .keys()
            .filter_map(|j| {
                if remaining_deps.contains_key(j) {
                    None
                } else {
                    Some(Reverse(j.clone()))
                }
            })
            .collect();

        Self {
            depends_on,
            remaining_deps,
            ready,
        }
    }
    fn complete(&mut self, j: &J) {
        if let Some(jobs) = self.depends_on.remove(j) {
            for job in jobs {
                let mut e = match self.remaining_deps.entry(job) {
                    Entry::Occupied(e) => e,
                    _ => unreachable!(),
                };
                let count = e.get_mut();
                *count -= 1;
                if *count == 0 {
                    let (job, _) = e.remove_entry();
                    self.ready.push(Reverse(job));
                }
            }
        }
    }
    fn next_job(&mut self) -> Option<J> {
        Some(self.ready.pop()?.0)
    }
}

pub struct Order;
impl Answer for Order {
    type Input = ByLine<Dependancy<String>>;
    type Output = String;
    fn ans(&self, inputs: impl Iterator<Item = Dependancy<String>>) -> Result<Self::Output> {
        let mut s = Scheduler::new(inputs);
        let mut answer = String::new();

        while let Some(job) = s.next_job() {
            answer.push_str(&job);
            s.complete(&job);
        }
        Ok(answer)
    }
}

fn cost(s: &str) -> usize {
    (s.chars().next().unwrap() as usize - 'A' as usize) + 61
}

pub struct Workers(pub usize);
impl Answer for Workers {
    type Input = ByLine<Dependancy<String>>;
    type Output = usize;
    fn ans(&self, inputs: impl Iterator<Item = Dependancy<String>>) -> Result<Self::Output> {
        let mut s = Scheduler::new(inputs);
        let mut counter = 0;
        let mut running = Vec::new();

        loop {
            // Cleanup finished jobs
            running.retain(|(name, count)| {
                if *count == 0 {
                    s.complete(name);
                    false
                } else {
                    true
                }
            });

            // Assign new jobs
            while running.len() < self.0 {
                if let Some(job) = s.next_job() {
                    let c = cost(&job);
                    running.push((job, c));
                } else {
                    if running.is_empty() {
                        return Ok(counter);
                    }
                    break;
                }
            }

            // Advance counters
            for (_, count) in &mut running {
                *count -= 1;
            }
            counter += 1;
        }
    }
}
