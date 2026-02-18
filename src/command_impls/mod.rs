//! Implementations of each [command](crate::cli::Commands).
//! Copyright (C) 2026  RosieTheGhostie
//!
//! See [the crate-level documentation](crate) for the full copyright notice.

pub use fuzz::fuzz;
pub use recreate::recreate;

mod fuzz;
mod recreate;
