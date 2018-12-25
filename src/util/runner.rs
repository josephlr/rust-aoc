use std::{
    io::BufRead,
    iter::*,
    marker::PhantomData,
    str::{from_utf8, FromStr},
};

use itertools::{process_results, unfold};

use crate::util::{Error, Result};

pub struct Bytes;
pub struct Lines;
pub struct ByLine<T>(PhantomData<T>);
pub struct ByWhitespace<T>(PhantomData<T>);

pub trait Input: Sized {
    type Item;
    fn next_input(reader: &mut impl BufRead) -> Result<Option<Self::Item>>;
}

impl Input for () {
    type Item = !;
    fn next_input(_: &mut impl BufRead) -> Result<Option<Self::Item>> {
        Ok(None)
    }
}

#[allow(clippy::use_self)]
impl Input for String {
    type Item = String;
    fn next_input(reader: &mut impl BufRead) -> Result<Option<Self::Item>> {
        let mut buf = String::new();
        Ok(match reader.read_to_string(&mut buf)? {
            0 => None,
            _ => Some(buf),
        })
    }
}

impl Input for Bytes {
    type Item = u8;
    fn next_input(reader: &mut impl BufRead) -> Result<Option<Self::Item>> {
        let mut byte = [0];
        Ok(match reader.read(&mut byte)? {
            0 => None,
            _ => Some(byte[0]),
        })
    }
}

impl Input for Lines {
    type Item = String;
    fn next_input(reader: &mut impl BufRead) -> Result<Option<Self::Item>> {
        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 {
            Ok(None)
        } else {
            if line.ends_with('\n') {
                line.pop();
            }
            Ok(Some(line))
        }
    }
}

impl<T: FromStr> Input for ByLine<T>
where
    Error: From<<T as FromStr>::Err>,
{
    type Item = T;
    fn next_input(reader: &mut impl BufRead) -> Result<Option<Self::Item>> {
        Ok(match Lines::next_input(reader)? {
            Some(line) => Some(line.parse()?),
            None => None,
        })
    }
}

// TODO: Handle invalid UTF8 better and make more efficient
fn clear_whitespace(reader: &mut impl BufRead) -> Result<()> {
    loop {
        let buf = reader.fill_buf()?;
        let len = buf.len();
        if len == 0 {
            return Ok(());
        }
        for (i, byte) in buf.iter().enumerate() {
            if !(byte.is_ascii_whitespace()) {
                reader.consume(i);
                return Ok(());
            }
        }
        reader.consume(len);
    }
}

fn read_until_whitespace(reader: &mut impl BufRead) -> Result<String> {
    let mut s = String::new();
    loop {
        let buf = reader.fill_buf()?;
        let len = buf.len();
        if len == 0 {
            return Ok(s);
        }
        for i in 0..len {
            if buf[i].is_ascii_whitespace() {
                s.push_str(from_utf8(&buf[0..i])?);
                reader.consume(i);
                return Ok(s);
            }
        }
        s.push_str(from_utf8(buf)?);
        reader.consume(len);
    }
}

impl<T: FromStr> Input for ByWhitespace<T>
where
    Error: From<<T as FromStr>::Err>,
{
    type Item = T;
    fn next_input(reader: &mut impl BufRead) -> Result<Option<Self::Item>> {
        clear_whitespace(reader)?;
        let s = read_until_whitespace(reader)?;
        if s.is_empty() {
            Ok(None)
        } else {
            Ok(Some(s.parse()?))
        }
    }
}

pub trait Answer {
    type Input: Input;
    type Output;

    fn ans(
        &self,
        inputs: impl Iterator<Item = <Self::Input as Input>::Item>,
    ) -> Result<Self::Output>;

    fn run<R: BufRead>(&self, r: R) -> Result<String>
    where
        Self::Output: ToString,
    {
        let inputs = unfold(r, |r| match Self::Input::next_input(r) {
            Ok(Some(i)) => Some(Ok(i)),
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        });
        let answer = process_results(inputs, |inputs| self.ans(inputs))?;
        Ok(answer?.to_string())
    }
}
