use crate::command_prelude::*;

pub fn cli() -> Command {
    subcommand("start")
}

pub fn exec(gctx: &GlobalContext, args: &ArgMatches) -> CliResult {
    println!("start command");
    Ok(())
}
