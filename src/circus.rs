use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use std::str::FromStr;
use nom::{alpha, digit};
use util::{force_string, Ans};

struct ProgData<'a> {
    weight: i32,
    children: Vec<&'a str>,
}

struct Input<'a> {
    root: &'a str,
    programs: HashMap<&'a str, ProgData<'a>>,
}

named!(program(&str) -> (&str, ProgData), do_parse!(
    name: alpha >>
    weight: map_res!(delimited!(
        tag!(" ("), digit, tag!(")")
    ), FromStr::from_str) >>
    children: map!(opt!(complete!(
        preceded!(tag!(" -> "),
            separated_nonempty_list_complete!(tag!(", "), alpha)
        )
    )), Option::unwrap_or_default) >>
    (name, ProgData{weight, children})
));

fn run<V>(r: impl BufRead, process: fn(Input) -> V) -> V {
    let input = force_string(r);

    let mut programs = HashMap::new();
    for line in input.lines() {
        let (rest, (name, data)) = program(line).unwrap();
        if !rest.is_empty() {
            panic!("Remaining input {:?}", rest);
        }
        if programs.insert(name, data).is_some() {
            panic!("Duplicate parent {:?}", name);
        }
    }

    let mut names: HashSet<_> = programs.keys().collect();
    for (_, data) in programs.iter() {
        for child in data.children.iter() {
            if !names.remove(child) {
                panic!("Unknown or duplicate child {:?}", child);
            }
        }
    }

    let root = names.drain().next().unwrap();
    process(Input { root, programs })
}

pub struct RootName;
impl Ans for RootName {
    type Value = String;
    fn compute(&self, r: impl BufRead) -> String {
        run(r, |input| input.root.to_owned())
    }
}

struct Tree {
    weight: i32,
    children: Vec<Tree>,
}

fn make(name: &str, programs: &mut HashMap<&str, ProgData>) -> Tree {
    let ProgData { weight, children } = programs
        .remove(name)
        .unwrap_or_else(|| panic!("Cycle detected containing {:?}", name));
    let children = children
        .into_iter()
        .map(|name| make(name, programs))
        .collect();
    Tree { weight, children }
}

enum Score {
    Total(i64),
    Change(i32),
}
use circus::Score::*;

#[derive(Clone, Copy)]
struct Ref {
    w: i32,
    t: i64,
}

#[derive(Clone, Copy)]
enum State {
    Empty,
    OneValOnce(Ref),
    OneValMany(i64),
    TwoVal(Ref, Ref),
    ManyVal(i64, Ref),
}
use circus::State::*;

fn score(tree: &Tree) -> Score {
    let mut state = Empty;
    for tree in tree.children.iter() {
        let w = tree.weight;
        match (score(tree), state) {
            (Change(new), _) => return Change(new),
            (Total(t), Empty) => {
                state = OneValOnce(Ref { w, t });
            }
            (Total(t), OneValOnce(r1)) => {
                state = match t == r1.t {
                    true => OneValMany(t),
                    false => TwoVal(Ref { w, t }, r1),
                };
            }
            (Total(t), OneValMany(t1)) => if t != t1 {
                state = ManyVal(t1, Ref { w, t });
            },
            (Total(t), TwoVal(r1, r2)) => {
                state = match () {
                    _ if t == r1.t => ManyVal(t, r2),
                    _ if t == r2.t => ManyVal(t, r1),
                    _ => panic!("Weights {}, {}, {} all appear", t, r1.t, r2.t),
                };
            }
            (Total(t), ManyVal(t1, r2)) => if t == r2.t {
                panic!("Mutiples weights for both {} and {}", t, t1)
            } else if t != t1 {
                panic!("Weights {}, {}, {} all appear", t, t1, r2.t)
            },
        }
    }

    let root_weight = tree.weight as i64;
    match state {
        Empty => Total(root_weight),
        OneValOnce(Ref { w: _, t }) => Total(t + root_weight),
        OneValMany(t) => Total(t * tree.children.len() as i64 + root_weight),
        TwoVal(_, _) => panic!("Weight to change is ambiguous"),
        ManyVal(t0, Ref { w, t }) => Change((t0 - t + w as i64) as i32),
    }
}

pub struct CorrectWeight;
impl Ans for CorrectWeight {
    type Value = i32;
    fn compute(&self, r: impl BufRead) -> i32 {
        run(r, |Input { root, mut programs }| {
            match score(&make(root, &mut programs)) {
                Total(_) => panic!("Entire tree was balanced"),
                Change(weight) => weight,
            }
        })
    }
}
