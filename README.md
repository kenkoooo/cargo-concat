# cargo-concat

[![Crates.io](https://img.shields.io/crates/v/cargo-concat)](https://crates.io/crates/cargo-concat)
![GitHub](https://img.shields.io/github/license/kenkoooo/cargo-concat)

This tool allows you to put all the modules you use in the executable file into one file.

# Install

```sh
cargo install cargo-concat
```

# Usage

In your Cargo project directory, you can hit the following command to generate a concatenated file.

```
cargo concat --bin main --output out.rs
```

