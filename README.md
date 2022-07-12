# times

[<img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/liborty/times/HEAD?logo=github">](https://github.com/liborty/times)
[<img alt="crates.io" src="https://img.shields.io/crates/v/times?logo=rust">](https://crates.io/crates/times)
[<img alt="crates.io" src="https://img.shields.io/crates/d/times?logo=rust">](https://crates.io/crates/times)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/times?logo=rust">](https://docs.rs/times)

# Times - Easy Benchmark for Timing and Comparing Algorithms within Rust

## Usage

```rust
use times::*;
```

## Introduction

This crate will be typically included under [dev-dependecies] in `Cargo.toml` and then used by source files under `tests` or `benches` directories. Whenever the runtime speed comparisons are of interest, which they practically always are.
