use std::io::BufRead;
use nom::anychar;
use util::{force_string, nom_unwrap, Ans};

#[derive(Debug)]
enum Item<'a> {
    Group(Vec<Item<'a>>),
    Garbage(Vec<Chunk<'a>>),
}

#[derive(Debug)]
enum Chunk<'a> {
    Normal(&'a str),
    Cancelled(char),
}

named!(chunk(&str) -> Chunk, map!(take_till1!(|c| c == '!' || c == '>'), Chunk::Normal));

named!(cancel(&str) -> Chunk, map!(preceded!(tag!("!"), anychar), Chunk::Cancelled));

named!(garbage_inner(&str) -> Vec<Chunk>, many0!(alt_complete!(chunk | cancel)));

named!(garbage(&str) -> Item, map!(
    delimited!(tag!("<"), garbage_inner, tag!(">")),
    Item::Garbage
));

named!(group_inner(&str) -> Vec<Item>, separated_list_complete!(tag!(","), item));

named!(group(&str) -> Item, map!(
    delimited!(tag!("{"), group_inner, tag!("}")),
    Item::Group
));

named!(item(&str) -> Item, alt_complete!(group | garbage));

fn total_score(i: Item, level: i32) -> i32 {
    match i {
        Item::Group(items) => {
            let s: i32 = items.into_iter().map(|i| total_score(i, level + 1)).sum();
            level + s
        }
        Item::Garbage(_) => 0,
    }
}

pub struct TotalScore;
impl Ans for TotalScore {
    type Value = i32;
    fn compute(&self, r: impl BufRead) -> i32 {
        let input = force_string(r);
        let parsed = nom_unwrap(item(input.trim()));
        total_score(parsed, 1)
    }
}

fn garbage_len(i: Item) -> usize {
    match i {
        Item::Group(items) => items.into_iter().map(garbage_len).sum(),
        Item::Garbage(chunks) => chunks
            .into_iter()
            .map(|chunk| match chunk {
                Chunk::Normal(s) => s.len(),
                Chunk::Cancelled(_) => 0,
            })
            .sum(),
    }
}

pub struct GarbageLen;
impl Ans for GarbageLen {
    type Value = usize;
    fn compute(&self, r: impl BufRead) -> usize {
        let input = force_string(r);
        let parsed = nom_unwrap(item(input.trim()));
        garbage_len(parsed)
    }
}
