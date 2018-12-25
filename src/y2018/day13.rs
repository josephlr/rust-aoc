use std::{cmp::Ordering, collections::BinaryHeap};

use euclid::Vector2D;

use crate::util::{Answer, Error, IterExtra, Lines, Result};

type Point = Vector2D<i32>;
type Map = Vec<Vec<State>>;
type Queue = BinaryHeap<Cart>;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn unit(self) -> Point {
        match self {
            Direction::Up => Point::new(0, -1),
            Direction::Left => Point::new(-1, 0),
            Direction::Down => Point::new(0, 1),
            Direction::Right => Point::new(1, 0),
        }
    }
    fn turn(&mut self, t: Turn) {
        *self = match t {
            Turn::Straight => return,
            // Turning "Left"  makes the directions go counterclockwise.
            Turn::Left => match *self {
                Direction::Up => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Up,
            },
            // Turning "Right"  makes the directions go clockwise.
            Turn::Right => match *self {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            },
        };
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum State {
    None,
    Vert,
    Horz,
    Plus,
    UlLr,
    UrLl,
}

fn decode(c: char) -> Result<(State, Option<Direction>)> {
    match c {
        '^' => Ok((State::Vert, Some(Direction::Up))),
        'v' => Ok((State::Vert, Some(Direction::Down))),
        '<' => Ok((State::Horz, Some(Direction::Left))),
        '>' => Ok((State::Horz, Some(Direction::Right))),
        '|' => Ok((State::Vert, None)),
        '-' => Ok((State::Horz, None)),
        '+' => Ok((State::Plus, None)),
        '\\' => Ok((State::UlLr, None)),
        '/' => Ok((State::UrLl, None)),
        ' ' => Ok((State::None, None)),
        _ => Err(Error::Parse(format!("Unknown character: {}", c))),
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
enum Turn {
    Left,
    Straight,
    Right,
}

impl Turn {
    fn next(&mut self) {
        *self = match *self {
            Turn::Left => Turn::Straight,
            Turn::Straight => Turn::Right,
            Turn::Right => Turn::Left,
        };
    }
}

#[derive(Eq, Debug)]
struct Cart {
    pos: Point,
    dir: Direction,
    turn: Turn,
    gen: usize,
}

impl Cart {
    fn step(&mut self, map: &[Vec<State>]) -> Result<()> {
        self.gen += 1;
        self.pos += self.dir.unit();
        match map.get(self.pos.y as usize)?.get(self.pos.x as usize)? {
            State::None => {
                return Err(Error::Custom("We have left the track"));
            }
            State::UlLr => {
                // Change direction "\"
                self.dir = match self.dir {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                }
            }
            State::UrLl => {
                // Change direction "/"
                self.dir = match self.dir {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                }
            }
            State::Plus => {
                self.dir.turn(self.turn);
                self.turn.next();
            }
            _ => {}
        };
        Ok(())
    }
}

impl PartialEq for Cart {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cart {
    fn cmp(&self, other: &Self) -> Ordering {
        self.gen
            .cmp(&other.gen)
            .then(self.pos.y.cmp(&other.pos.y))
            .then(self.pos.x.cmp(&other.pos.x))
            .reverse()
    }
}

fn make_map(inputs: impl Iterator<Item = String>) -> Result<(Map, Queue)> {
    let mut carts = Queue::new();
    let map = inputs
        .enumerate()
        .map(|(y, input)| {
            let row = input.chars().enumerate();
            row.map(|(x, c)| {
                let (state, dir) = decode(c)?;
                if let Some(d) = dir {
                    let cart = Cart {
                        pos: Point::new(x as i32, y as i32),
                        dir: d,
                        turn: Turn::Left,
                        gen: 0,
                    };
                    carts.push(cart)
                }
                Ok(state)
            })
            .collect()
        })
        .collect::<Result<Map>>()?;
    Ok((map, carts))
}

pub struct FirstCrash;
impl Answer for FirstCrash {
    type Input = Lines;
    type Output = String;
    fn ans(&self, inputs: impl Iterator<Item = String>) -> Result<Self::Output> {
        let (map, mut carts) = make_map(inputs)?;

        loop {
            // Potential bug, will not work for carts colliding on different generations.
            if let Ok(collision) = carts.iter().map(|c| c.pos).duplicates().single() {
                return Ok(format!("{}", collision));
            }

            let mut top = carts.peek_mut().ok_or(Error::Custom("No carts present"))?;
            top.step(&map)?;
        }
    }
}

pub struct LastCart;
impl Answer for LastCart {
    type Input = Lines;
    type Output = String;
    fn ans(&self, inputs: impl Iterator<Item = String>) -> Result<Self::Output> {
        let (map, mut carts) = make_map(inputs)?;

        loop {
            carts = carts
                .into_iter()
                .only_uniques_by(|c| c.pos)
                .into_iter()
                .collect();
            if carts.len() < 2 {
                let mut last = carts.pop().ok_or(Error::Custom("All carts collided"))?;
                let pos = last.pos;
                last.step(&map)?;
                return Ok(format!("{} or {}", pos, last.pos));
            }

            let mut top = carts.peek_mut().unwrap();
            top.step(&map)?;
        }
    }
}
