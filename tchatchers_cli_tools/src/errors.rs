use std::{
    fmt::Display,
    process::{ExitCode, Termination},
};

use bb8::RunError;

/// Common errors returned during the runtime.
///
/// These errors are wrapped and then returned as a positive integer error.
#[derive(Debug, Clone, PartialEq, Eq, From, Constructor, Error)]
pub struct CliError {
    /// The error message printed to the user.
    message: String,
    /// The error kind.
    kind: ErrorKind,
}

impl Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}::{}] {}", self.kind, self.kind as u8, self.message)
    }
}

/// The different types of errors handled by the runtime.
#[derive(Debug, Clone, PartialEq, Eq, From, Display, Copy)]
#[repr(u8)]
pub enum ErrorKind {
    /// An error linked with a DB statement execution.
    ///
    /// It can indicate a bad .env not leading to a proper connection to the remote host (client error),
    /// or a statement execution error such as a badly written request.
    StatementExecution = 1,
    /// An error linked to the writing within a file, or access to a particular file.
    IoError,
    /// Error linked with template generation.
    TemplateError,
    /// Error with inconsistent args received.
    UnreachableError,
    /// Error linked with acquiring or performing operations on redis pool.
    RedisError,
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

impl From<askama::Error> for CliError {
    fn from(value: askama::Error) -> Self {
        Self::new(value.to_string(), ErrorKind::TemplateError)
    }
}

impl From<redis::RedisError> for CliError {
    fn from(value: redis::RedisError) -> Self {
        Self::new(value.to_string(), ErrorKind::RedisError)
    }
}

impl<E> From<RunError<E>> for CliError {
    fn from(_value: RunError<E>) -> Self {
        Self::new("Redis pool error".into(), ErrorKind::RedisError)
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
