use crate::{ChildStdin, Error, ExitStatus, Output};
use std::{io, process};

#[derive(Debug)]
struct Inner {
    command: String,
    inner: process::Child,
}

/// A handle to a child process. You **must** call either [`Handle::wait`] or
/// [`Handle::wait_for_output`] to consume the handle. If you don't, it'll get
/// mad at you.
#[derive(Debug)]
#[must_use = "handles must be `wait`ed on, or they won't stop"]
pub struct Handle {
    inner: Option<Inner>,
}

impl Drop for Handle {
    fn drop(&mut self) {
        if let Some(inner) = self.inner.as_ref() {
            log::error!(
                "handle for command {:?} dropped without being waited on",
                inner.command
            );
        }
    }
}

impl Handle {
    pub(crate) fn new(command: String, inner: process::Child) -> Self {
        Self {
            inner: Some(Inner { command, inner }),
        }
    }

    fn expect<T>(opt: Option<T>) -> T {
        opt.expect("developer error: `Handle` vacant")
    }

    fn as_mut(&mut self) -> &mut Inner {
        Self::expect(self.inner.as_mut())
    }

    fn take(mut self) -> Inner {
        Self::expect(self.inner.take())
    }

    pub fn stdin(&mut self) -> Option<&mut ChildStdin> {
        self.as_mut().inner.stdin.as_mut()
    }

    pub fn kill(&mut self) -> io::Result<()> {
        self.as_mut().inner.kill()
    }

    pub fn wait(self) -> crate::Result<ExitStatus> {
        let Inner { command, mut inner } = self.take();
        Error::from_status_result(command, inner.wait())
    }

    pub fn wait_for_output(self) -> crate::Result<Output> {
        let Inner { command, inner } = self.take();
        Error::from_output_result(command, inner.wait_with_output())
    }
}
