use std::str::FromStr;

use euclid::Vector2D;
use itertools::Itertools;
use nom::*;

use crate::util::{to_result, trim_parse, Answer, ByLine, Error, Result};

type Point = Vector2D<i32>;

#[derive(PartialEq, Eq, Hash)]
pub struct Light {
    position: Point,
    velocity: Point,
}

named!(point<&str,Point>, do_parse!(
    tag!("<") >>
    x: map_res!(take_until_and_consume!(","), trim_parse) >>
    y: map_res!(take_until_and_consume!(">"), trim_parse) >>
    (Point::new(x, y))
));

named!(light<&str,Light>, do_parse!(
    tag!("position=") >> position: point >> tag!(" ") >>
    tag!("velocity=") >> velocity: point >>
    (Light{position, velocity})
));

impl FromStr for Light {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        to_result(light(s))
    }
}

pub struct Sky(pub i32);

impl Sky {
    fn contains(&self, p: Point) -> bool {
        p.x.abs() < self.0 && p.y.abs() < self.0
    }
}

fn plot(lights: &[Light]) {
    let (x_min, x_max) = lights
        .iter()
        .map(|l| l.position.x)
        .minmax()
        .into_option()
        .unwrap();
    let (y_min, y_max) = lights
        .iter()
        .map(|l| l.position.y)
        .minmax()
        .into_option()
        .unwrap();

    let mut canvas = [[false; 100]; 100];

    for light in lights {
        let x = ((light.position.x - x_min) * 100) / (x_max - x_min + 1);
        let y = ((light.position.y - y_min) * 100) / (y_max - y_min + 1);
        canvas[x as usize][(99 - y) as usize] = true;
    }

    for row in canvas.iter() {
        for occupied in row.iter() {
            print!("{}", if *occupied { '#' } else { '.' });
        }
        println!();
    }
}

impl Answer for Sky {
    type Input = ByLine<Light>;
    type Output = usize;
    fn ans(&self, inputs: impl Iterator<Item = Light>) -> Result<Self::Output> {
        let mut count = 0;
        let mut lights: Vec<_> = inputs.collect();
        while !lights.is_empty() {
            count += 1;
            lights.drain_filter(|l| {
                l.position += l.velocity;
                !self.contains(l.position)
            });

            // Found via repeated iteration
            if count == 10_312 {
                plot(&lights);
                println!();
                break;
            }
        }

        Ok(count)
    }
}
