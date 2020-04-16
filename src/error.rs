use crate::{ExitStatus, Handle, Output};
use std::{
    fmt::{self, Display},
    io, process, str,
};

/// The specific cause of an [`Error`].
#[derive(Debug)]
pub enum Cause {
    SpawnFailed(io::Error),
    WaitFailed(io::Error),
    CommandFailed(ExitStatus),
    CommandFailedWithOutput(Output),
}

impl Cause {
    fn from_io_err(err: io::Error) -> Self {
        Self::WaitFailed(err)
    }

    fn from_status(status: process::ExitStatus) -> Result<ExitStatus, Self> {
        if status.success() {
            Ok(status)
        } else {
            Err(Self::CommandFailed(status))
        }
    }

    fn from_output(output: process::Output) -> Result<Output, Self> {
        let output = Output::new(output);
        if output.success() {
            Ok(output)
        } else {
            Err(Self::CommandFailedWithOutput(output))
        }
    }

    fn status(&self) -> Option<ExitStatus> {
        if let Self::CommandFailed(status) = self {
            Some(status.clone())
        } else {
            self.output().map(|output| output.status())
        }
    }

    fn output(&self) -> Option<&Output> {
        if let Self::CommandFailedWithOutput(output) = self {
            Some(output)
        } else {
            None
        }
    }
}

/// The bearer of bad news.
#[derive(Debug)]
pub struct Error {
    command: String,
    cause: Cause,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn command_failed(
            f: &mut fmt::Formatter,
            command: &str,
            status: ExitStatus,
        ) -> fmt::Result {
            write!(f, "Command {:?} didn't complete successfully, ", command)?;
            if let Some(exit_code) = status.code() {
                write!(f, "exiting with code {}.", exit_code)
            } else {
                write!(f, "but returned no exit code.")
            }
        }

        match &self.cause {
            Cause::SpawnFailed(err) => write!(
                f,
                "Failed to spawn child process for command {:?}: {}",
                self.command, err
            ),
            Cause::WaitFailed(err) => write!(
                f,
                "Failed to wait for child process for command {:?} to exit: {}",
                self.command, err
            ),
            Cause::CommandFailed(status) => command_failed(f, &self.command, *status),
            Cause::CommandFailedWithOutput(output) => {
                command_failed(f, &self.command, output.status())?;
                if !output.stderr().is_empty() {
                    write!(
                        f,
                        "stderr contents: {}",
                        String::from_utf8_lossy(output.stderr())
                    )
                } else {
                    write!(f, "stderr was empty.")
                }
            }
        }
    }
}

impl Error {
    pub(crate) fn from_status_result(
        command: String,
        result: io::Result<process::ExitStatus>,
    ) -> Result<ExitStatus, Self> {
        result
            .map_err(Cause::from_io_err)
            .and_then(Cause::from_status)
            .map_err(|cause| Self { command, cause })
    }

    pub(crate) fn from_output_result(
        command: String,
        result: io::Result<process::Output>,
    ) -> Result<Output, Self> {
        result
            .map_err(Cause::from_io_err)
            .and_then(Cause::from_output)
            .map_err(|cause| Self { command, cause })
    }

    pub(crate) fn from_child_result(
        command: String,
        result: io::Result<process::Child>,
    ) -> Result<Handle, Self> {
        // `match` is favored here to avoid cloning `command`
        match result {
            Ok(child) => Ok(Handle::new(command, child)),
            Err(err) => Err(Self {
                command,
                cause: Cause::from_io_err(err),
            }),
        }
    }

    pub fn command(&self) -> &str {
        &self.command
    }

    pub fn status(&self) -> Option<ExitStatus> {
        self.cause.status()
    }

    pub fn output(&self) -> Option<&Output> {
        self.cause.output()
    }

    pub fn stdout(&self) -> Option<&[u8]> {
        self.output().map(|output| output.stdout())
    }

    pub fn stdout_str(&self) -> Option<Result<&str, str::Utf8Error>> {
        self.output().map(|output| output.stdout_str())
    }

    pub fn stderr(&self) -> Option<&[u8]> {
        self.output().map(|output| output.stderr())
    }

    pub fn stderr_str(&self) -> Option<Result<&str, str::Utf8Error>> {
        self.output().map(|output| output.stderr_str())
    }
}
