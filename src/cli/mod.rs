//! Command-line interface (CLI) definition.
//! Copyright (C) 2026  RosieTheGhostie
//!
//! See [the crate-level documentation](crate) for the full copyright notice.

pub use number_range::NumberRange;

mod number_range;

use core::num::NonZeroU8;
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
