//! A [`RangeInclusive`]-like `struct` for command-line parsing.
//! Copyright (C) 2026  RosieTheGhostie
//!
//! See [the crate-level documentation](crate) for the full copyright notice.

use core::{
    fmt::{self, Display, Formatter},
    ops::RangeInclusive,
    str::FromStr,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct NumberRange<T> {
    start: T,
    end: T,
}

impl<T> NumberRange<T> {
    pub const fn start(&self) -> &T {
        &self.start
    }

    pub const fn end(&self) -> &T {
        &self.end
    }
}

impl<T> NumberRange<T>
where
    T: Ord,
{
    pub fn new(start: T, end: T) -> Option<Self> {
        (end >= start).then(|| Self { start, end })
    }
}

impl<T> NumberRange<T>
where
    T: FromStr,
    <T as FromStr>::Err: ToString,
{
    fn parse_value(s: &str) -> Result<T, String> {
        s.parse()
            .map_err(|value: <T as FromStr>::Err| value.to_string())
    }
}

impl<T> From<T> for NumberRange<T>
where
    T: Clone,
{
    fn from(value: T) -> Self {
        Self {
            start: value.clone(),
            end: value,
        }
    }
}

impl<T> From<NumberRange<T>> for RangeInclusive<T> {
    fn from(value: NumberRange<T>) -> Self {
        Self::new(value.start, value.end)
    }
}

impl<T> Default for NumberRange<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            start: T::default(),
            end: T::default(),
        }
    }
}

impl<T> Display for NumberRange<T>
where
    T: Display + Eq,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let NumberRange { start, end } = self;
        if start == end {
            write!(f, "{start}")
        } else {
            write!(f, "{start}..={end}")
        }
    }
}

impl<T> FromStr for NumberRange<T>
where
    T: Clone + FromStr + Ord,
    <T as FromStr>::Err: ToString,
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((start, end)) = s.split_once("..=") {
            let start: T = Self::parse_value(start)?;
            let end: T = Self::parse_value(end)?;
            Self::new(start, end).ok_or_else(|| "Cannot construct a backwards range".to_string())
        } else {
            Self::parse_value(s).map(Self::from)
        }
    }
}
