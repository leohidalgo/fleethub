pub use util::command_prelude;

pub mod util;

pub fn exit_with_code(code: i32) -> ! {
    std::process::exit(code)
}
