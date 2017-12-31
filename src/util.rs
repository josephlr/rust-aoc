use std::fmt::Display;
use std::io::BufRead;
use std::str::FromStr;

// We would normally do something like:
//
// ```
// pub trait Parseable: FromStr where Self::Err: Display {}
//
// impl<T: FromStr> Parseable for T where T::Err: Display {}
// ```
//
// and then use `s.parse()::<P>` instead of `P::parse(&s)`.
// However, rust-lang/rust#44491 prevents this from actually working.
pub trait Parseable: FromStr {
    type Err: Display;
    fn parse(s: &str) -> Result<Self, <Self as Parseable>::Err>;
}

impl<T: FromStr> Parseable for T
where
    T::Err: Display,
{
    type Err = T::Err;
    fn parse(s: &str) -> Result<Self, <Self as Parseable>::Err> {
        s.parse()
    }
}

pub fn force_parse<P: Parseable>(s: &str) -> P {
    P::parse(&s).unwrap_or_else(|e| panic!("could not parse {:?}: {}", s, e))
}

pub fn force_parse_line<P: Parseable>(r: &mut impl BufRead) -> P {
    let mut s = String::new();
    r.read_line(&mut s).unwrap();
    force_parse(s.trim())
}

pub trait Ans<Phantom = ()> {
    type Value: Display;
    fn compute(&self, r: impl BufRead) -> Self::Value;
}
