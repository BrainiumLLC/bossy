//! Opinionated convenience wrapper for `std::process::Command` and friends.
//!
//! Note that this re-exports [`std::process::ChildStdin`],
//! [`std::process::ExitStatus`], and [`std::process::Stdio`], so the docs for
//! those items below might seem a bit out of place.

mod error;
mod handle;
mod output;

mod result {
    pub type Result<T> = std::result::Result<T, crate::error::Error>;
}

pub use self::{error::*, handle::*, output::*, result::*};
pub use std::process::{ChildStdin, ExitStatus, Stdio};

use std::{ffi::OsStr, process};

/// Build and run commands to your heart's content.
#[derive(Debug)]
pub struct Command {
    inner: process::Command,
    display: String,
}

impl Command {
    fn push_display(&mut self, component: &OsStr) {
        if !self.display.is_empty() {
            self.display.push(' ');
        }
        self.display.push_str(component.to_string_lossy().as_ref());
    }

    /// Start building a command that inherits all env vars from the environment.
    pub fn impure(name: impl AsRef<OsStr>) -> Self {
        let name = name.as_ref();
        let mut this = Self {
            inner: process::Command::new(name),
            display: Default::default(),
        };
        this.push_display(name);
        this
    }

    /// Start building a command with a completely clean environment. Note that
    /// at minimum, you'll often want to add `PATH` and `TERM` to the environment
    /// for things to function as expected.
    pub fn pure(name: impl AsRef<OsStr>) -> Self {
        let mut this = Self::impure(name);
        this.inner.env_clear();
        this
    }

    /// Get the command's string representation.
    pub fn display(&self) -> &str {
        &self.display
    }

    pub fn set_stdin(&mut self, cfg: impl Into<Stdio>) -> &mut Self {
        let cfg = cfg.into();
        log::debug!("setting stdin to {:?} on command {:?}", cfg, self.display);
        self.inner.stdin(cfg);
        self
    }

    pub fn with_stdin(mut self, cfg: impl Into<Stdio>) -> Self {
        self.set_stdin(cfg);
        self
    }

    pub fn set_stdin_piped(&mut self) -> &mut Self {
        self.set_stdin(Stdio::piped());
        self
    }

    pub fn with_stdin_piped(mut self) -> Self {
        self.set_stdin_piped();
        self
    }

    pub fn set_stdout(&mut self, cfg: impl Into<Stdio>) -> &mut Self {
        let cfg = cfg.into();
        log::debug!("setting stdout to {:?} on command {:?}", cfg, self.display);
        self.inner.stdout(cfg);
        self
    }

    pub fn with_stdout(mut self, cfg: impl Into<Stdio>) -> Self {
        self.set_stdout(cfg);
        self
    }

    pub fn set_stdout_piped(&mut self) -> &mut Self {
        self.set_stdout(Stdio::piped());
        self
    }

    pub fn with_stdout_piped(mut self) -> Self {
        self.set_stdout_piped();
        self
    }

    pub fn set_stderr(&mut self, cfg: impl Into<Stdio>) -> &mut Self {
        let cfg = cfg.into();
        log::debug!("setting stderr to {:?} on command {:?}", cfg, self.display);
        self.inner.stderr(cfg);
        self
    }

    pub fn with_stderr(mut self, cfg: impl Into<Stdio>) -> Self {
        self.set_stderr(cfg);
        self
    }

    pub fn set_stderr_piped(&mut self) -> &mut Self {
        self.set_stderr(Stdio::piped());
        self
    }

    pub fn with_stderr_piped(mut self) -> Self {
        self.set_stderr_piped();
        self
    }

    pub fn add_env_var(&mut self, key: impl AsRef<OsStr>, val: impl AsRef<OsStr>) -> &mut Self {
        let key = key.as_ref();
        let val = val.as_ref();
        log::debug!(
            "adding env var {:?} = {:?} to command {:?}",
            key,
            val,
            self.display
        );
        self.inner.env(key, val);
        self
    }

    pub fn with_env_var(mut self, key: impl AsRef<OsStr>, val: impl AsRef<OsStr>) -> Self {
        self.add_env_var(key, val);
        self
    }

    pub fn add_env_vars(
        &mut self,
        vars: impl IntoIterator<Item = (impl AsRef<OsStr>, impl AsRef<OsStr>)>,
    ) -> &mut Self {
        for (key, val) in vars.into_iter() {
            self.add_env_var(key, val);
        }
        self
    }

    pub fn with_env_vars(
        mut self,
        vars: impl IntoIterator<Item = (impl AsRef<OsStr>, impl AsRef<OsStr>)>,
    ) -> Self {
        self.add_env_vars(vars);
        self
    }

    pub fn add_arg(&mut self, name: impl AsRef<OsStr>) -> &mut Self {
        let name = name.as_ref();
        log::debug!("adding arg {:?} to command {:?}", name, self.display);
        self.inner.arg(name);
        self.push_display(name);
        self
    }

    pub fn with_arg(mut self, name: impl AsRef<OsStr>) -> Self {
        self.add_arg(name);
        self
    }

    pub fn add_args(&mut self, args: impl IntoIterator<Item = impl AsRef<OsStr>>) -> &mut Self {
        for arg in args.into_iter() {
            self.add_arg(arg);
        }
        self
    }

    pub fn with_args(mut self, args: impl IntoIterator<Item = impl AsRef<OsStr>>) -> Self {
        self.add_args(args);
        self
    }

    fn run_inner(&mut self) -> Result<Handle> {
        Error::from_child_result(self.display.clone(), self.inner.spawn())
    }

    /// Run the command and give you a delightful [`Handle`] to it. This allows
    /// you to decide when blocking should happen, but if you don't care, then
    /// [`Command::run_and_wait`] and [`Command::run_and_wait_for_output`] are
    /// better picks.
    pub fn run(&mut self) -> Result<Handle> {
        log::info!("running command {:?}", self.display);
        self.run_inner()
    }

    /// Run the command and block until it exits.
    pub fn run_and_wait(&mut self) -> Result<ExitStatus> {
        log::info!("running command {:?} and waiting for exit", self.display);
        self.run_inner()?.wait()
    }

    /// Run the command and block until its output is collected. This will
    /// automatically set stdout and stderr to use [`Stdio::piped`], so if you
    /// don't want that to be happen, then you're screwed.
    pub fn run_and_wait_for_output(&mut self) -> Result<Output> {
        log::info!("running command {:?} and waiting for output", self.display);
        self.set_stdout_piped()
            .set_stderr_piped()
            .run_inner()?
            .wait_for_output()
    }
}
