//! Running child processes.
//!
//! [`run`] leaves a child's output on this process's streams; [`capture`]
//! takes its standard output and carries its standard error into the error,
//! for a caller that would rather report the failure itself.

use std::process::Command;

use crate::error::Error;

/// Run `command` to completion.
///
/// # Errors
///
/// Fails if the child cannot be started, or ends unsuccessfully.
pub fn run(command: &mut Command) -> Result<(), Error> {
    let status = command.status().map_err(|source| Error::Io {
        program: program(command),
        source,
    })?;
    if status.success() {
        Ok(())
    } else {
        Err(Error::Exit {
            program: program(command),
            status,
            stderr: Vec::new(),
        })
    }
}

/// Run `command` and return its standard output.
///
/// # Errors
///
/// Fails if the child cannot be started, ends unsuccessfully, or writes
/// standard output that is not UTF-8.
pub fn capture(command: &mut Command) -> Result<String, Error> {
    let output = command.output().map_err(|source| Error::Io {
        program: program(command),
        source,
    })?;
    if !output.status.success() {
        return Err(Error::Exit {
            program: program(command),
            status: output.status,
            stderr: output.stderr,
        });
    }
    String::from_utf8(output.stdout).map_err(|source| Error::Utf8 {
        program: program(command),
        source,
    })
}

/// The name `command` runs, for a report of its failure.
#[must_use]
pub fn program(command: &Command) -> String {
    command.get_program().to_string_lossy().into_owned()
}
