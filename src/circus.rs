use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry::*;
use std::io::BufRead;
use util::{force_parse, Ans};

struct ProtoProgram<'a> {
    weight: i32,
    children: Vec<&'a str>,
}

struct Input<'a> {
    root: &'a str,
    map: HashMap<&'a str, ProtoProgram<'a>>,
}

fn parse(line: &str) -> (&str, ProtoProgram) {
    let mut iter = line.split_whitespace();
    let name = iter.next().expect("Missing Name");
    let weight = iter.next().expect("Missing Weight");
    let delims: &[_] = &['(', ')'];
    let weight = force_parse(weight.trim_matches(delims));

    let children = match iter.next() {
        None => Vec::new(),
        Some("->") => iter.map(|s| s.trim_matches(',')).collect(),
        Some(s) => panic!("Found invalid string {:?}", s),
    };
    (name, ProtoProgram { weight, children })
}

fn run<V>(mut r: impl BufRead, process: fn(Input) -> V) -> V {
    let mut input = String::new();
    r.read_to_string(&mut input).unwrap();

    let mut map = HashMap::new();
    for (name, program) in input.lines().map(parse) {
        if map.insert(name, program).is_some() {
            panic!("Duplicate parent {:?}", name);
        }
    }

    let mut names: HashSet<_> = map.keys().collect();
    for (_, program) in map.iter() {
        for child in program.children.iter() {
            if !names.remove(child) {
                panic!("Unknown or duplicate child {:?}", child)
            }
        }
    }

    let root = names.drain().next().unwrap();
    process(Input { root, map })
}

pub struct RootName;
impl Ans for RootName {
    type Value = String;
    fn compute(&self, r: impl BufRead) -> String {
        run(r, |input| input.root.to_owned())
    }
}

pub struct CorrectWeight;
impl Ans for CorrectWeight {
    type Value = i32;
    fn compute(&self, r: impl BufRead) -> i32 {
        0
    }
}
