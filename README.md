# `cargo msrv-table`

**This cargo subcommand is only a proof of concept (PoC) with no error handling and lots of code duplication. Use at your own risk!**

**If you're interested in fostering this into a real subcommand, contact me.**

A [`cargo`](https://github.com/rust-lang/cargo) subcommand to generate a
Makrdown table of Minimum Supported Rust Version by crate MAJOR.MINOR version
(in Semver terminology)

## Pre-requisits

This subcommand requires the following packages be installed on the system or
available in `$PATH`:

* [`jql`](https://github.com/yamafaktory/jql)
* [`rustup`](https://github.com/rust-lang/rustup)
* [`cargo-edit`](https://github.com/killercup/cargo-edit)
* `cargo`
* `rustc`
* `grep`
* `uniq`
* `timeout` (if `--timeout` is used)

## Warning

This subcommand can take a *long* time to run. It works by creating a faux
project of the target crate, and builds each Rust version from 1.0 until
current, and attempts to compile the crate with each version. It does this for
*every* published `MAJOR.MINOR`version of the crate that has been published to [crates.io](https://crates.io)

Future versions of this subcommand may allow filtering those versions down to
fewer numbers by skipping versions, or limiting ranges.

This subcommand also downlods a large ammount of data (all stable Rust version).

## License

This project is released under the terms of either the MIT or Apache 2.0 license at
your option.
