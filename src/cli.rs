//! Command-line interface (CLI) definition.
//! Copyright (C) 2026  RosieTheGhostie
//!
//! See [the crate-level documentation](crate) for the full copyright notice.

use core::num::{NonZeroI32, NonZeroUsize};
use std::{collections::HashSet, ffi::OsString};

use clap::Parser;

use crate::number_range::NumberRange;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// The program to fuzz.
    pub program: OsString,

    /// Any pre-defined command-line arguments to pass to the program.
    pub args: Vec<OsString>,

    /// The number of attempts to take before giving up.
    #[arg(long)]
    pub n_tries: NonZeroUsize,

    /// The range of numbers of extra command-line arguments to generate.
    #[arg(long, default_value_t)]
    pub n_args: NumberRange<usize>,

    /// The maximum length of any extra command-line arguments in UTF-8 codepoints.
    #[arg(long, default_value_t = NonZeroUsize::new(256).unwrap())]
    pub max_arg_len: NonZeroUsize,

    /// The range of numbers of bytes to pass in to the program via stdin.
    #[arg(long, default_value_t)]
    pub n_input_bytes: NumberRange<usize>,

    /// Non-zero exit codes to ignore.
    #[arg(long)]
    pub ignore: Vec<NonZeroI32>,
}

impl Cli {
    pub fn ok_exit_codes(&self) -> HashSet<i32> {
        let mut set = HashSet::with_capacity(self.ignore.len() + 1);

        set.insert(0);
        for exit_code in &self.ignore {
            set.insert(exit_code.get());
        }

        set
    }
}
