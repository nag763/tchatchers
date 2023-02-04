use std::{
    fmt::Display,
    process::{ExitCode, Termination},
};

#[derive(Debug, Clone, PartialEq, Eq, From, Constructor, Error)]
pub struct CliError {
    message: String,
    kind: ErrorKind,
}

impl Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}::{}] {}", self.kind, self.kind as u8, self.message)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, From, Display, Copy)]
#[repr(u8)]
pub enum ErrorKind {
    StatementExecution = 1,
    IoError,
}

impl From<sqlx::Error> for CliError {
    fn from(value: sqlx::Error) -> Self {
        Self::new(value.to_string(), ErrorKind::StatementExecution)
    }
}

impl From<std::io::Error> for CliError {
    fn from(value: std::io::Error) -> Self {
        Self::new(value.to_string(), ErrorKind::IoError)
    }
}

impl std::process::Termination for CliError {
    fn report(self) -> std::process::ExitCode {
        eprintln!("The process ended with the following error");
        eprintln!("{self}");
        ExitCode::FAILURE
    }
}

impl From<CliError> for ExitCode {
    fn from(value: CliError) -> Self {
        value.report()
    }
}
