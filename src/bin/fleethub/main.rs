pub use fleethub::command_prelude;

use crate::command_prelude::*;

mod cli;
mod commands;

fn main() {
    let gctx = GlobalContext::default();

    let result = cli::main(&gctx);

    match result {
        Ok(_) => { },
        Err(e) => fleethub::exit_with_code(e.exit_code)
    }
}
