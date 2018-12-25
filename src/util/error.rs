use std::{
    fmt, io, num, option,
    str::{FromStr, Utf8Error},
};

use nom::{error_position, IResult};

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Nom(String),
    Parse(String),
    Custom(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<Utf8Error> for Error {
    fn from(e: Utf8Error) -> Self {
        Error::Parse(e.to_string())
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IO(e)
    }
}

impl From<num::TryFromIntError> for Error {
    fn from(e: num::TryFromIntError) -> Self {
        Error::Parse(e.to_string())
    }
}

impl From<num::ParseIntError> for Error {
    fn from(e: num::ParseIntError) -> Self {
        Error::Parse(e.to_string())
    }
}

impl From<option::NoneError> for Error {
    fn from(_: option::NoneError) -> Self {
        Error::Custom("Unexpected option unwrap")
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn to_result<T>(r: IResult<&str, T>) -> Result<T> {
    match r {
        Ok(("", output)) => Ok(output),
        Ok((input, _)) => Err(Error::Nom(format!("{} leftover chars", input.len()))),
        Err(e) => Err(Error::Nom(e.to_string())),
    }
}

pub fn trim_parse<T: FromStr>(s: &str) -> std::result::Result<T, T::Err> {
    s.trim().parse()
}

// Nom parser for any itegral type written in ascii.
pub fn number<O: FromStr>(input: &str) -> IResult<&str, O> {
    let (input, output) = match nom::digit(input) {
        Ok(io) => io,
        Err(nom::Err::Incomplete(_)) => ("", input),
        Err(e) => return Err(e),
    };
    match output.parse() {
        Ok(n) => Ok((input, n)),
        Err(_) => Err(nom::Err::Error(error_position!(
            output,
            nom::ErrorKind::Digit
        ))),
    }
}
