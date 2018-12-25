use std::collections::HashMap;

use intrusive_collections::{
    intrusive_adapter,
    linked_list::{CursorMut, Link, LinkedList},
    Adapter,
};

use crate::util::{Answer, ByWhitespace, Error, Result};

struct Node {
    link: Link,
    value: usize,
}

impl Node {
    fn new(value: usize) -> Box<Self> {
        Box::new(Self {
            link: Link::new(),
            value,
        })
    }
}

#[allow(clippy::expl_impl_clone_on_copy)]
intrusive_adapter!(MyAdapter = Box<Node>: Node { link: Link });

fn shift<A: Adapter<Link = Link>>(cursor: &mut CursorMut<A>, offset: isize) {
    let forward = offset >= 0;
    let mut count = 0;
    while count < offset.abs() {
        if forward {
            cursor.move_next();
        } else {
            cursor.move_prev();
        }
        if !cursor.is_null() {
            count += 1;
        }
    }
}

struct Game {
    cursor: CursorMut<'static, MyAdapter>,
    scores: HashMap<u32, usize>,
}

impl Game {
    fn new() -> Self {
        let marbles = Box::leak(Box::new(LinkedList::new(MyAdapter::new())));
        marbles.push_back(Node::new(0));
        Self {
            cursor: marbles.cursor_mut(),
            scores: HashMap::new(),
        }
    }

    fn add(&mut self, player: u32, marble: usize) {
        if marble % 23 == 0 {
            shift(&mut self.cursor, -7);
            let score = self.cursor.remove().unwrap().value + marble;
            *self.scores.entry(player).or_default() += score;
        } else {
            shift(&mut self.cursor, 1);
            self.cursor.insert_after(Node::new(marble));
            shift(&mut self.cursor, 1);
        }
    }
}

// Input is "<players> <marbles>"
pub struct WinningScore;
impl Answer for WinningScore {
    type Input = ByWhitespace<usize>;
    type Output = usize;
    fn ans(&self, mut input: impl Iterator<Item = usize>) -> Result<Self::Output> {
        let players = input.next().ok_or(Error::Custom("No Num Player"))?;
        let marbles = input.next().ok_or(Error::Custom("No Num Marbles"))?;
        let mut g = Game::new();
        for marble in 1..=marbles {
            let player = ((marble % players) + 1) as u32;
            g.add(player, marble);
        }
        g.scores
            .values()
            .cloned()
            .max()
            .ok_or(Error::Custom("No Marbles"))
    }
}
