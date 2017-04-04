# rustproj

The repository to study [The Rust Programming Language](https://www.rust-lang.org).

## Installing Rust

Run this:

```
curl https://sh.rustup.rs -sSf | sh
```

And add `~/.cargo/bin` to your `PATH` environment variable.

## Emacs

### Install associeted tools

like this:

```
cargo install rustfmt
cargo install racer
```

[`rustfmt`](https://github.com/rust-lang-nursery/rustfmt) is the code formatter.
And [`racer`](https://github.com/phildawes/racer) is the code completion.

### configure racer

Run `rustup component add rust-src` and add `RUST_SRC_PATH` to your environment variable.
(e.g. `RUST_SRC_PATH=~/.multirust/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/src`)

### Install Emacs packages

Install these packages.

- rust-mode
- racer
- flycheck-rust
