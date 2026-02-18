//! Small suite of pseudorandom value generators.
//! Copyright (C) 2026  RosieTheGhostie
//!
//! See [the crate-level documentation](crate) for the full copyright notice.

use core::num::NonZeroUsize;
use std::ffi::OsString;

use rand::{
    distr::{
        StandardUniform, Uniform,
        uniform::{Error, SampleUniform},
    },
    prelude::*,
};

use crate::number_range::NumberRange;

pub fn extra_args<R>(
    mut rng: R,
    range: NumberRange<usize>,
    max_len: NonZeroUsize,
) -> Result<impl Iterator<Item = OsString>, Error>
where
    R: RngExt,
{
    let n_args = n_from_range(&mut rng, range)?;
    let arg_len_distribution = Uniform::new_inclusive(0, max_len.get())?;

    Ok((0..n_args).map(move |_| {
        let arg_len = rng.sample(arg_len_distribution);
        os_string(&mut rng, arg_len)
    }))
}

pub fn stdin<R>(mut rng: R, range: NumberRange<usize>) -> Result<Box<[u8]>, Error>
where
    R: RngExt,
{
    n_from_range(&mut rng, range).map(|n| bytes(rng, n))
}

pub fn hex_string<R>(rng: R, len: usize) -> String
where
    R: RngExt,
{
    rng.sample_iter(Uniform::new(0, 16).unwrap())
        .take(len)
        .map(|nibble| char::from_digit(nibble, 16).unwrap())
        .collect()
}

fn n_from_range<T, R>(mut rng: R, range: NumberRange<T>) -> Result<T, Error>
where
    R: RngExt,
    T: SampleUniform,
{
    Uniform::new_inclusive(range.start(), range.end()).map(|distribution| rng.sample(distribution))
}

fn string<R>(rng: R, n: usize) -> String
where
    R: RngExt,
{
    rng.sample_iter::<char, _>(StandardUniform)
        .take(n)
        .collect()
}

fn os_string<R>(rng: R, n: usize) -> OsString
where
    R: RngExt,
{
    string(rng, n).into()
}

pub fn bytes<R>(mut rng: R, n: usize) -> Box<[u8]>
where
    R: RngExt,
{
    // SAFETY: All the bytes will be written to before they are read.
    let mut buffer = unsafe { Box::new_zeroed_slice(n).assume_init() };
    rng.fill_bytes(&mut buffer);

    buffer
}
