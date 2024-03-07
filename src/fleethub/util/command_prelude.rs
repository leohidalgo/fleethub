pub use clap::{ArgMatches, Command};

pub use crate::util::errors::{CliError, CliResult};
pub use crate::util::context::GlobalContext;

pub fn subcommand(name: &'static str) -> Command {
    Command::new(name)
}
