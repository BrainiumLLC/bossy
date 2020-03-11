use crate::ExitStatus;
use std::{process, str};

/// All your output, in one convenient place! Wow!
#[derive(Debug)]
pub struct Output {
    inner: process::Output,
}

impl Output {
    pub(crate) fn new(inner: process::Output) -> Self {
        Self { inner }
    }

    pub fn status(&self) -> ExitStatus {
        self.inner.status
    }

    pub fn success(&self) -> bool {
        self.status().success()
    }

    pub fn stdout(&self) -> &[u8] {
        &self.inner.stdout
    }

    pub fn stdout_str(&self) -> Result<&str, str::Utf8Error> {
        str::from_utf8(self.stdout())
    }

    pub fn stderr(&self) -> &[u8] {
        &self.inner.stderr
    }

    pub fn stderr_str(&self) -> Result<&str, str::Utf8Error> {
        str::from_utf8(self.stderr())
    }
}
