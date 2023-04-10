use std::{
    ffi::OsString,
    fs::{self, OpenOptions},
    io::Write,
};

use clap::{builder::ArgPredicate, Args};

use crate::errors::{CliError, ErrorKind};

#[derive(Args, Clone, Debug)]
pub struct OutputStream {
    /// Writes the result of the command in a file.
    #[arg(
        short = 'o',
        long = "output",
        conflicts_with = "standard_output",
        help = "Writes the result of the command in a file."
    )]
    pub output_file: Option<OsString>,

    /// Write the result of the command in the standard output.
    #[arg(
        long = "standard-output",
        help = "Write the result of of the command in the standard output.",
        default_value = "true",
        default_value_if("output_file", ArgPredicate::IsPresent, "false")
    )]
    pub standard_output: bool,
}

impl OutputStream {
    /// Returns `true` if the output stream has already been filled.
    pub fn stream_already_filled(&self) -> bool {
        if !self.standard_output {
            if let Some(output_file) = &self.output_file {
                return fs::read(output_file).is_ok();
            }
        }
        false
    }

    /// Write all bytes from the buffer `buf` to the output stream.
    ///
    /// # Arguments
    ///
    /// - buf : the content to write to the stream.
    pub fn write_all(&self, buf: &[u8]) -> Result<(), CliError> {
        match &self.output_file {
            Some(output_file) if !self.standard_output => {
                let mut out = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(output_file)?;
                out.write_all(buf)?;
            }
            None if self.standard_output => {
                let mut out = std::io::stdout();
                out.write_all(buf)?;
            }
            _ => {
                return Err(CliError::new(
                    "Command output could not be determined".into(),
                    ErrorKind::UnreachableError,
                ))
            }
        }
        Ok(())
    }
}
