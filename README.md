# Times [![crates.io](https://img.shields.io/crates/v/times?logo=rust)](https://crates.io/crates/times) [![crates.io](https://img.shields.io/crates/d/times?logo=rust)](https://crates.io/crates/times) [![GitHub last commit](https://img.shields.io/github/last-commit/liborty/times/HEAD?logo=github)](https://github.com/liborty/times) [![Actions Status](https://github.com/liborty/times/actions/workflows/test.yml/badge.svg)](https://github.com/liborty/times/actions)

## Author: Libor Spacek

Benchmarks for timing comparison of algorithms that take `$[T]` and `&[Vec<T>]` inputs and their mutable
variants. Written in 100% safe Rust.

## Usage

```rust
use times::*;
```

## Introduction

This crate will be typically added in `Cargo.toml` under `[dev-dependecies]`  and then used by source files in `tests` or `benches` directories. It can be used whenever the runtime speed comparisons are of interest, which is practically always.

`Times` is suitable for testing algorithms that work on a `Vec` of numerical data, for example sort. Also, whole matrices of data `&[Vec<T>]`.

The correctness of the results
should be tested separately. Here the results produced by the algorithms are thrown away and only the execution time in nanoseconds is recorded.

Random data are automatically generated using `ran` crate and then all the algorithms from a given array of closures are executed over the same data. These runs are repeated a specified number of times, each time with new random data. The run times statistics are collected for each algorithm (median of the execution *times* and their spread (`mad`). Mad stands for median of absolute differences from median; it is the most stable measure of data spread. Repeated runs reduce the side effects of changing machine loads, cache utilisation, etc. The effects of outliers are reduced by using `mad` instead of standard deviation.

`Mad` spread expresses doubt about the reliability of the measurements. High relative values mean poor reliability. Relative measurement inaccuracy (spread as a percentage) can be often reduced by increasing the number of repeats. The extraneous influence of the machine load can also be somewhat reduced by increasing the length of the data vectors.

New random data is generated for each repeated run. The residual spread for each algorithm measures its stability under changing data. Some algorithms suffer from data sensitivity (poor worst-case performance) and this may be indicated by relatively high spreads, e.g. for `rust-sort` (the standard Rust sort).

The tests are also automatically repeated over different lengths of the input data vectors, in specified range and step. This enables comparisons of algorithms as the difficulty of the problem increases. The algorithms with lower computational complexity and/or faster implementations will start to win more convincingly over greater data lengths.

When the data length becomes too large, then the process may have to be externally terminated. Depending, of course, on the algorithms and the speed of the machine. It is recommended to use modest range end value at first.

## Main Features

* Mad, as more stable measure of spread (measurement error).

* Easy to use - just specify:
  * range of data vectors lengths, and step,
  * the number of repeat runs (with different random data) for each length,
  * some names to identify the algorithms by on the pretty printout,
  * the closures invoking the algorithms (in the same order as their names).

* Sorted output

    The algorithms are automatically sorted by their execution times within each length of data category, e.g. the fastest algorithm for each data length is listed first and the slowest last. The last (yellow) column lists their relative execution times, with the fastest being always 1.

## Provided Testing Functions

Four different end-types of data are fully supported: `u8,u16,u64,f64`. See `tests/tests.rs`.

* Simple function `bench` for timing closures that take no or constant arguments

* Bench functions for testing algorithms on input slices, e.g. on input data types `&[T]`:
`benchu8, benchu16, benchu64 and benchf64`

* Bench functions for testing algorithms that mutate their input, e.g. data types `&mut [f64]`:
`mutbenchu8, mutbenchu16, mutbenchu64 and mutbenchf64`. These mutable versions have to be used whenever any one of the tested algorithms mutates its input

* Bench functions for algorithms taking `Vec<Vec<T>>` or `&[Vec<T>]` inputs: `benchvvu8, benchvvu16, benchvvu64 and benchvvf64`.

## Conclusion

Please see [`tests/test.rs`](https://github.com/liborty/times/blob/main/tests/tests.rs) for examples of how to specify the closures and call these functions on them.

## Appendix - Recent Releases

**Version 1.0.15** Upgraded to Ran v 2.0.0

**Version 1.0.14** Upgraded to Medians v 3.0.0, enabled checking for Nans, improved reports.

**Version 1.0.13** Fixed some dependency problems in tests.

**Version 1.0.12** Removed dependency on `devtimer`.

**Version 1.0.11** Upped dependency `indxvec` to 1.8, `medians` to 2.3.

**Version 1.0.10** Upped dependency `medians` to 2.2.

**Version 1.0.9** Added `mutbenchu16` and `benchvvu16`. Simplified the printouts.

**Version 1.0.8** Added `benchu16`. Updated dependencies `medians` to ^2.1 and `indxvec` to ^1.4.

**Version 1.0.7** Updated dependency `medians` to v.2.0.0 and `indxvec`.

**Version 1.0.6** Updated dependency `medians` and github actions.

**Version 1.0.5** Updated dependency `ran` to `1.0`.

**Version 1.0.4** Instead of magnitudes number, now takes the actual range and step of the data lengths. Is more flexible.
