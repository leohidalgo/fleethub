use fleethub::command_prelude::*;

pub mod start;

pub fn builtin() -> Vec<Command> {
    vec![
        start::cli()
    ]
}

pub type Exec = fn(&GlobalContext, &ArgMatches) -> CliResult;

pub fn builtin_exec(cmd: &str) -> Option<Exec> {
    let f = match cmd {
        "start" => start::exec,
        _ => return None
    };

    Some(f)
}
