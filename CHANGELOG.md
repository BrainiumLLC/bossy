# Unreleased

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
