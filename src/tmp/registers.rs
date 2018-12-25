use std::collections::HashMap;
use std::io::BufRead;
use std::str::FromStr;
use nom::alpha;
use util::{force_string, nom_unwrap, Ans};

type Val = i32;
type Op = fn(Val, Val) -> bool;

struct Instruction<'a> {
    incr_reg: &'a str,
    incr: Val,
    test_reg: &'a str,
    op: Op,
    test: Val,
}

named!(not_space(&str) -> &str, take_while!(|c| !char::is_whitespace(c)));

named!(val(&str) -> Val, map_res!(not_space, FromStr::from_str));

fn to_op(token: &str) -> Option<Op> {
    match token {
        "==" => Some(|x, y| x == y),
        "!=" => Some(|x, y| x != y),
        "<" => Some(|x, y| x < y),
        ">" => Some(|x, y| x > y),
        "<=" => Some(|x, y| x <= y),
        ">=" => Some(|x, y| x >= y),
        _ => None,
    }
}

named!(instruction(&str) -> Instruction, ws!(do_parse!(
    incr_reg: alpha >>
    sign: alt_complete!(
        tag!("inc") => {|_| 1} |
        tag!("dec") => {|_| -1} 
    ) >>
    amount: val >>
    tag!("if") >>
    test_reg: alpha >>
    op: map_opt!(not_space, to_op) >>
    test: val >>
    (Instruction{incr_reg, incr: amount * sign, test_reg, op, test})
)));

fn largest_values(r: impl BufRead) -> (Val, Val) {
    let input = force_string(r);
    let mut registers = HashMap::new();
    let max_during = input
        .lines()
        .filter_map(|line| {
            let i = nom_unwrap(instruction(line));
            let test_val = *registers.entry(i.test_reg).or_default();
            if (i.op)(test_val, i.test) {
                let incr_val = registers.entry(i.incr_reg).or_default();
                *incr_val += i.incr;
                Some(*incr_val)
            } else {
                None
            }
        })
        .max()
        .expect("No registers incremented");
    let max_final = *registers.values().max().expect("No registers used");
    (max_final, max_during)
}

pub struct LargestFinal;
impl Ans for LargestFinal {
    type Value = Val;
    fn compute(&self, r: impl BufRead) -> Val {
        largest_values(r).0
    }
}

pub struct LargestDuring;
impl Ans for LargestDuring {
    type Value = Val;
    fn compute(&self, r: impl BufRead) -> Val {
        largest_values(r).1
    }
}
