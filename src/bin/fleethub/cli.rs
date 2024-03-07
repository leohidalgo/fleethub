use clap::crate_version;

use crate::command_prelude::*;
use crate::commands;

pub fn main(gctx: &GlobalContext) -> CliResult {
    let args = cli(gctx).get_matches();

    let (cmd, args) = match args.subcommand() {
        Some((cmd, args)) => (cmd, args),
        _ => {
            cli(gctx).print_help()?;
            return Ok(())
        }
    };

    let exec = match commands::builtin_exec(cmd) {
        Some(f) => f,
        _ => return Err(CliError::code(1))
    };

    exec(gctx, args)?;

    Ok(())
}

fn cli(gctx: &GlobalContext) -> Command {
    Command::new("fleethub")
        .version(crate_version!())
        .subcommands(commands::builtin())
}
