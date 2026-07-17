# bls — the better ls command

A modern replacement for `ls` written in Rust. Displays a colorized, formatted listing of files and directories with last modified timestamps.

## Features

- Colorized output (directories in blue, timestamps in green)
- Last modified timestamp for every entry
- Directories listed first, then files
- Hidden file filtering (dotfiles hidden by default)
- List any directory by passing a path argument

## Installation

```sh
cargo install --path .
```

## Usage

```sh
# List current directory
bls

# List a specific directory
bls /path/to/dir

# Show hidden files
bls -a
```

## Example Output

```
        LastWriteTime     Length   Name
  2026-07-15 14:30          -   src
  2026-07-10 09:15          -   .git
  2026-07-15 14:30        312   Cargo.toml
  2026-07-14 11:00       1024   main.rs
```

## Dependencies

- [clap](https://crates.io/crates/clap) — CLI argument parsing
- [colored](https://crates.io/crates/colored) — Terminal colors
- [chrono](https://crates.io/crates/chrono) — Date/time formatting
