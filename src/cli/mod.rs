//! Command-line interface (CLI) definition.
//! Copyright (C) 2026  RosieTheGhostie
//!
//! See [the crate-level documentation](crate) for the full copyright notice.

use core::{
    fmt::{self, Display, Formatter},
    num::NonZeroU8,
    ops::{Deref, DerefMut, RangeInclusive},
    str::FromStr,
};
use std::{ffi::OsString, path::PathBuf};

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// The program to fuzz.
    pub program: PathBuf,

    /// Any pre-defined command-line arguments to pass to the program.
    pub args: Vec<OsString>,

    /// The range of numbers of extra command-line arguments to generate.
    #[arg(long, default_value_t)]
    pub n_args: NumberRange<usize>,

    /// Non-zero exit codes to ignore.
    #[arg(long)]
    pub ignore: Vec<NonZeroU8>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct NumberRange<T>(pub RangeInclusive<T>);

impl<T> Deref for NumberRange<T> {
    type Target = RangeInclusive<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for NumberRange<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Default for NumberRange<T>
where
    T: Default,
{
    fn default() -> Self {
        Self(T::default()..=T::default())
    }
}

impl<T> Display for NumberRange<T>
where
    T: Display + Eq,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let start = self.start();
        let end = self.end();
        if start == end {
            write!(f, "{}", self.0.start())
        } else {
            write!(f, "{}..={}", self.0.start(), self.0.end())
        }
    }
}

impl<T> From<T> for NumberRange<T>
where
    T: Clone,
{
    fn from(value: T) -> Self {
        Self(value.clone()..=value)
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
            if end >= start {
                Ok(Self(start..=end))
            } else {
                Err("Cannot construct a backwards range".to_string())
            }
        } else {
            Self::parse_value(s).map(Self::from)
        }
    }
}
