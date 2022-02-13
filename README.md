# chmap

`chmap` is a command line tool to work with Clang headermaps produced by Xcode. It is written in Rust.

`chmap` is the modern, cross-paltform version of the Swift [hmap](https://github.com/milend/hmap) tool.

# How to Get

## Requirements

You need a Rust toolchain, see [rustup](https://rustup.rs).

## Cargo

1. Clone the repository
2. If you want to run via `cargo`, use `cargo run -- arguments`
3. If you want to install, `cargo install --path chmap`

## `cheadermap` Crate

If you would like to use the [cheadermap](https://crates.io/crates/cheadermap) library crate, add the following to your `Cargo.toml`:

```toml
[dependencies]
cheadermap = "0.1.0"
```

# How to Use

To print the contents of an hmap file, execute:

    chmap print /path/to/file.hmap

For example, if you have just cloned the repository, execute `cargo run -- print /path/to/file.hmap`.

# Development

[Visual Studio Code](https://code.visualstudio.com) coupled with the [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) and [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) provide a good IDE experience.

## `rustfmt` & `clippy`

If you don't have `rustfmt` and `clippy`, you can install them by executing:

    rustup component add rustfmt
    rustup component add clippy

Before committing, ensure code is formatted and passes `clippy` without any warnings using:

    cargo fmt --all
    cargo clippy

## Testing

From the repo root, execute:

    cargo test