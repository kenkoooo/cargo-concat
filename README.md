# cargo-concat

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

