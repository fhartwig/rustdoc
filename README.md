# rustdoc

This is an out-of-tree mirror of rustdoc. The purpose of this is to have
a cargo-buildable version of rustdoc that is easier to develop on than the
one that comes with rustc.
Please note that this is not a fork of rustdoc, so please send all pull requests
to [the official repository](https://github.com/rust-lang/rust).

[![Build Status](https://travis-ci.org/fhartwig/standalone-rustdoc.svg?branch=master)](https://travis-ci.org/fhartwig/standalone-rustdoc)

## Building

You will need the nightly version of rust, which you can get
[here](https://www.rust-lang.org/downloads.html).

```
git clone git@github.com:fhartwig/standalone-rustdoc.git
cd rustdoc
cargo build --release
```

## running

First you need to set the `RUST_SYSROOT` environment variable to the directory
under which your rust installation lives (on my machine, that is `/usr/local`).
There should now be a executable called `rustdoc` under `target/release`, which
you can run on any rust source file.
