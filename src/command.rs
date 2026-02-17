//! Simplified builder interface for [`std::process::Command`] that implements [`Clone`].
//! Copyright (C) 2026  RosieTheGhostie
//!
//! See [the crate-level documentation](crate) for the full copyright notice.

use std::{
    ffi::{OsStr, OsString},
    process::Stdio,
};

#[derive(Clone, Debug)]
pub struct Command<'a> {
    pub program: &'a OsStr,
    pub args: Vec<OsString>,
}

impl<'a> Command<'a> {
    pub const fn new(program: &'a OsStr) -> Self {
        Self {
            program,
            args: Vec::new(),
        }
    }

    pub fn with_args<I>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = OsString>,
    {
        self.args.extend(args);
        self
    }

    pub fn build(self) -> std::process::Command {
        let mut command = std::process::Command::new(self.program);
        command
            .args(self.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        command
    }
}
