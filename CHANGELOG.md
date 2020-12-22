# Unreleased

# 0.2.0 (2020-12-21)

- **Breaking:** the `stdout_str` and `stderr_str` methods on `bossy::Output` and `bossy::Error` now use `bossy::Error` instead of `std::str::Utf8Error` as their error type.
- Added `run_and_wait_for_str` and `run_and_wait_for_string` methods to `bossy::Command` to make it super easy to get stdout as a string.

# 0.1.4 (2020-12-09)

- Added convenience methods to `bossy::Command` for setting `bossy::Stdio::null`.
- Added `run_and_detach` method to `bossy::Command` to make it easy to spawn daemons and the like.

# 0.1.3 (2020-11-05)

- Implemented `std::fmt::Display` for `bossy::Command`.

# 0.1.2 (2020-08-06)

- Fix mistakes in README.

# 0.1.1 (2020-08-06)

- Improved formatting of errors.
- Added methods that allow you to parse arguments from strings.
- Added `stdout` and `stderr` methods to `bossy::Handle`.
- Implemented `std::error::Error` for `bossy::Error`.
