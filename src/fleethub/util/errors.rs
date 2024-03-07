pub type CliResult = Result<(), CliError>;

pub struct CliError {
    pub error: Option<anyhow::Error>,
    pub exit_code: i32
}

impl CliError {
    pub fn new(error: anyhow::Error, exit_code: i32) -> Self {
        Self { error: Some(error), exit_code }
    }

    pub fn code(exit_code: i32) -> Self {
        Self { error: None, exit_code }
    }
}

impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> Self {
        Self::new(err.into(), 1)
    }
}
