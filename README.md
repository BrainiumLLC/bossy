# `bossy`

[![crates.io badge](http://meritbadge.herokuapp.com/bossy)](https://crates.io/crates/bossy)
[![docs.rs badge](https://docs.rs/bossy/badge.svg)](https://docs.rs/bossy)
[![Travis badge](https://travis-ci.org/BrainiumLLC/bossy.svg?branch=master)](https://travis-ci.org/BrainiumLLC/bossy)

Opinionated convenience wrapper for `std::process::Command` and friends.

```rust
use bossy::Command;

fn main() -> bossy::Result<()> {
    Command::impure("ls").with_arg("-l").run_and_wait()?;
    Ok(())
}
```

You can run [the example](examples/ls.rs) to see the exact same code as above, but like, with output.

This isn't a ton of documentation, but this is a pretty thin wrapper, so documentation for `std::process` will typically apply here as well.

