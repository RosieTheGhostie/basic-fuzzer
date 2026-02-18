use std::{
    ffi::OsStr,
    fs::File,
    io::{self, prelude::*},
    process::CommandArgs,
};

use rand::RngExt;

use crate::{cli::FuzzArguments, command::Command, generate};

pub fn fuzz(args: FuzzArguments) -> anyhow::Result<()> {
    let ok_exit_codes = args.ok_exit_codes();

    let command = Command::new(&args.program).with_args(args.args);
    let mut rng = rand::rng();

    for _try in 0..args.n_tries.get() {
        let input = generate::stdin(&mut rng, args.n_input_bytes)?;

        let mut command = command
            .clone()
            .with_args(generate::extra_args(
                &mut rng,
                args.n_args,
                args.max_arg_len,
            )?)
            .build();
        let mut child = command.spawn()?;
        child.stdin.take().unwrap().write_all(&input)?;

        match child.wait()?.code() {
            Some(code) if ok_exit_codes.contains(&code) => continue,
            Some(code) => eprintln!("Program terminated with exit code {code}"),
            None => eprintln!("Program terminated via signal"),
        }

        return record_input_and_args(rng, &input, command.get_args()).map_err(anyhow::Error::from);
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
