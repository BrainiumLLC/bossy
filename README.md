# `bossy`

[![crates.io badge](http://meritbadge.herokuapp.com/bossy)](https://crates.io/crates/bossy)
[![docs.rs badge](https://docs.rs/bossy/badge.svg)](https://docs.rs/bossy)
[![CI Status](https://github.com/BrainiumLLC/bossy/workflows/CI/badge.svg)](https://github.com/BrainiumLLC/bossy/actions)

Opinionated convenience wrappers for `std::process::Command` and friends.

This crate arose from patterns I found while working on [`cargo-mobile`](https://github.com/BrainiumLLC/cargo-mobile), which does a *ton* of subprocessing. In my not-entirely-humble opinion, `bossy` makes working with commands super convenient!

```rust
use bossy::Command;
use std::{io::Write as _, path::Path};

// `bossy::Error` contains detailed error information; the process failing to
// spawn, the process exiting with a non-zero status, stderr contents, etc.
// For commands with piped output, you'll even have access to the stdout
// contents.
fn main() -> bossy::Result<()> {
    // We generate a ton of helpful logging, if you're into that sort of thing.
    simple_logger::init().unwrap();

    let path = Path::new("src");
    println!("{:?} directory contents:", path);
    // `impure` indicates that this command inherits the parent process's
    // environment. For more reproducability, you can use `pure` to get a
    // completely empty environment.
    let status = Command::impure_parse("ls -l")
        // `std::process::Command::arg` takes `&mut self` and returns
        // `&mut Self`; our equivalent of that is `add_arg`, but I personally
        // prefer using `with_arg`, which takes `self` and returns `Self`.
        .with_arg(path)
        // We use more explicit names for our run methods than
        // `std::process::Command` does:
        // - `run` (equivalent to `spawn`)
        // - `run_and_wait` (equivalent to `status`)
        // - `run_and_wait_for_output` (equivalent to `output`)
        .run_and_wait()?;
    // `bossy::ExitStatus` is just a re-export of `std::process::ExitStatus`.
    println!("exited with code {:?}", status.code());

    let readme_output = Command::impure_parse("cat README.md")
        // Just like with `std::process::Command::output`, this will
        // automatically pipe stdout and stderr.
        .run_and_wait_for_output()?;
    // `bossy::Output` has cute conveniences for the very common task of
    // converting output to a string.
    println!(
        "README.md contents:\n{}",
        readme_output
            .stdout_str()
            .expect("README.md contained invalid utf-8")
    );

    let mut handle = Command::impure("shasum")
        // We also have methods that let you set these using `bossy::Stdio`
        // (which is currently just a re-export of `std::process::Stdio`), but
        // this spares you some typing and an import.
        .with_stdin_piped()
        .with_stdout_piped()
        .with_stderr_piped()
        .run()?;
    handle
        .stdin()
        // This will only be `None` if you forgot to set stdin to piped above.
        .expect("developer error: `handle` stdin not captured")
        .write_all(readme_output.stdout())
        .expect("failed to write to `handle` stdin");
    // `bossy::Handle` is very similar to `std::process::Child`, but will
    // log an error message if it's dropped without being waited on.
    let shasum_output = handle.wait_for_output()?;
    println!(
        "README.md SHA-1 sum: {}",
        shasum_output
            .stdout_str()
            .expect("shasum output contained invalid utf-8")
    );

    Ok(())
}
```

You can run [the example](examples/commands.rs) to see the exact same code as above, but like, with output:

```sh
cargo run --example commands
```

This isn't a ton of documentation, but this is a pretty thin wrapper, so documentation for `std::process` will typically apply here as well.

