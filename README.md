# `cargo msrv-table`

**This cargo subcommand is only a proof of concept (PoC) with no error handling and lots of code duplication. Use at your own risk!**

**If you're interested in fostering this into a real subcommand, contact me.**

A [`cargo`](https://github.com/rust-lang/cargo) subcommand to generate a
table of Minimum Supported Rust Version by crate `MAJOR.MINOR` version
(in [SemVer](https://semver.org) terminology)

For example:

``` sh
$ cargo msrv-table clap

[...]

clap    MSRV
===     ===
2.33	1.24.1
2.32	1.24.1
2.31	1.24.1
2.30	1.24.1
2.29	1.24.1
2.28	1.24.1
2.27	1.24.1
2.26	1.24.1
2.25	1.24.1
2.24	1.24.1
2.23	1.24.1
2.22	1.24.1
2.21	1.24.1
2.20	1.21.0
2.19	1.12.1
2.18	1.12.1
2.17	1.12.1
2.16	1.12.1
2.15	1.12.1
2.14	1.12.1
2.13	1.12.1
2.12	1.12.1
2.11	1.12.1
2.10	1.12.1
2.9	1.12.1
2.8	1.12.1
2.7	1.12.1
2.6	1.12.1
2.5	1.12.1
2.4	1.12.1
2.3	1.12.1
2.2	1.12.1
2.1	1.6.0
2.0	1.4.0
1.5	1.4.0
1.4	1.2.0
1.3	1.1.0
1.2	1.1.0
1.1	1.0.0
1.0	1.0.0
```

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

## Considerations

By default this subcommand will try to build all published
`MAJOR.MINOR.MAX_PATCH` versions of a crate against each stable Rust compiler
(`1.MINOR.MAX_PATCH`).

### PreReleases

It skips pre-release (`<=0.y.z`) unless `--no-skip-prereleases` is used.

### Pre-Download Rust Versions

On first run, one of the longest wait items is downloading all the Rust versions
to check. You can pre-download the Rust versions using `rustup`

``` sh
$ for VER in {0.0,1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0,11.0,12.1,13.0,14.0,15.1,16.0,17.0,18.0,19.0,20.0,21.0,22.1,23.0,24.1,25.0,26.2,27.2,28.0,29.2,30.1,31.0,32.0,33.0,34.2,35.0,36.0,37.0,38.0,39.0,40.0,41.1}; do rustup install 1.$VER; done
```

### Cleanup

Likewise, you may wish to delete all those Rust versions:


``` sh
$ for VER in {0.0,1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0,11.0,12.1,13.0,14.0,15.1,16.0,17.0,18.0,19.0,20.0,21.0,22.1,23.0,24.1,25.0,26.2,27.2,28.0,29.2,30.1,31.0,32.0,33.0,34.2,35.0,36.0,37.0,38.0,39.0,40.0,41.1}; do rustup uninstall 1.$VER; done
```

### Eager Ending

By default Rust versions are traversed in *ascending* order, and will end
traversal once a successful build is found. However, if you choose to use
`--rust-order=descending` you may also want to disable this eagerness to avoid a
false positive. This can be disabled by using `--no-eager-end` which will 
continue to build against earlier Rust versions even after a failed build.

### Timeouts

A timeout can be supplied for the `cargo build` command which will consider a
build failed after `N` seconds. This is useful for early builds that may hang.

### `cargo` features

There is no consideration for cargo features

### Target Platform

There is no consideration for target platform

## License

This project is released under the terms of either the MIT or Apache 2.0 license at
your option.
