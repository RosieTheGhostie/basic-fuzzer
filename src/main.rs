//! basic-fuzzer: a basic CLI fuzzer.
//! Copyright (C) 2026  RosieTheGhostie
//!
//! This program is free software: you can redistribute it and/or modify
//! it under the terms of the GNU General Public License as published by
//! the Free Software Foundation, either version 3 of the License, or
//! (at your option) any later version.
//!
//! This program is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU General Public License for more details.
//!
//! You should have received a copy of the GNU General Public License
//! along with this program.  If not, see <https://www.gnu.org/licenses/>.

mod cli;
mod command;
mod generate;

use core::error::Error;
use std::{
    ffi::OsStr,
    fs::File,
    io::{self, prelude::*},
    process::CommandArgs,
};

use clap::Parser;
use rand::RngExt;

use cli::Cli;
use command::Command;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let ok_exit_codes = cli.ok_exit_codes();

    let command = Command::new(cli.program.as_os_str()).with_args(cli.args);
    let mut rng = rand::rng();

    for _try in 0..cli.n_tries.get() {
        let input = generate::stdin(&mut rng, cli.n_input_bytes)?;

        let mut command = command
            .clone()
            .with_args(generate::extra_args(&mut rng, cli.n_args, cli.max_arg_len)?)
            .build();
        let mut child = command.spawn()?;
        child.stdin.take().unwrap().write_all(&input)?;

        match child.wait()?.code() {
            Some(code) if ok_exit_codes.contains(&code) => continue,
            Some(code) => eprintln!("Program terminated with exit code {code}"),
            None => eprintln!("Program terminated via signal"),
        }

        return record_input_and_args(rng, &input, command.get_args())
            .map_err(Box::<dyn Error>::from);
    }

    eprintln!("Could not produce a failing state");
    Ok(())
}

fn record_input_and_args<R>(rng: R, input: &[u8], args: CommandArgs<'_>) -> io::Result<()>
where
    R: RngExt,
{
    let random_suffix = generate::hex_string(rng, 12);

    File::create_new(format!("input-{random_suffix}"))?.write_all(input)?;

    let mut args_file = File::create_new(format!("args-{random_suffix}"))?;
    for arg in args.into_iter().map(OsStr::as_encoded_bytes) {
        let arg_len: u64 = arg.len() as _;
        args_file.write_all(&arg_len.to_le_bytes())?;
        args_file.write_all(arg)?;
    }

    Ok(())
}
