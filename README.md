# Times [![crates.io](https://img.shields.io/crates/v/times?logo=rust)](https://crates.io/crates/times) [![crates.io](https://img.shields.io/crates/d/times?logo=rust)](https://crates.io/crates/times) [![GitHub last commit](https://img.shields.io/github/last-commit/liborty/times/HEAD?logo=github)](https://github.com/liborty/times) [![Actions Status](https://github.com/liborty/times/actions/workflows/test.yml/badge.svg)](https://github.com/liborty/times/actions)

## Author: Libor Spacek

Benchmark for Timing and Comparing Algorithms in Rust.  
Written in 100% safe Rust.

## Usage

```rust
use times::*;
```

## Introduction

This crate will be typically added in `Cargo.toml` under `[dev-dependecies]`  and then used by source files under `tests` or `benches` directories. It can be used whenever the runtime speed comparisons are of interest, which is practically always.

`Times` is suitable for testing algorithms that work on a whole `Vec` of data, for example sort. Or even whole matrices of data `&[Vec<T>]`.

The correctness of the results
should be tested separately. Here the results produced by the algorithms are thrown away and only the execution time in nanoseconds is recorded.

Random data are automatically generated using `ran` crate and then algorithms from a given array of closures are repeatedly run and their statistics are collected (median of the execution *times* and the measurements standard error). Repeated runs reduce the temporary effects of changing machine loads. The effects of outliers are minimised by using `mad` instead of standard deviation. Mad stand for median of absolute differences from median; it is the most stable measure of data spread.

All the algorithms are run over the same data for exact comparisons but the data is changed for each repeat run.

The error estimates the doubt about the reliability of repeated measurements. High value means poor reliability. Relative measurement accuracy can be often increased by increasing the number of repeats. The extraneous influence of the machine load is also reduced as the length of the data vectors increases.

We generate new random data for each repeated run. The differences in errors between the algorithms inform us about their relative stability under changing data. Some algorithms suffer from data sensitivity (poor worst-case performance) and this may be indicated by relatively high errors, e.g. for `rust-sort` (the standard Rust sort).

The tests are also automatically repeated over different lengths of the input data vectors, in specified range and step. This enables comparisons of algorithms as the difficulty of the problem increases. The algorithms with lower computational complexity and/or faster implementations will start to win more convincingly at greater lengths.

When the data length becomes too large, then the process may have to be externally terminated. Depending, of course, on the algorithms and the speed of the machine. It is recommended to use modest values at first.

## Main Features

* Mad, as more stable standard error.

* Ease of Use - just specify:
  * the type of the random test data,
  * range of data vectors lengths, and step
  * the number of repeats over different random data,
  * some names to identify the algorithms by
  * the closures invoking the algorithms (in the same order as their labels).

* Sorted output.

    The algorithms are automatically sorted by their execution times within each length of data category, i.e. the fastest algorithm in each data category will be listed first and the slowest last.

## Provided Testing Functions

Four different and-types of data are fully supported: `u8,u16,u64,f64`. Other end-types may be added later. See `tests/tests.rs`.

* Simple function `bench` for timing closures that take no or constant arguments.

* Bench functions for testing algorithms on input slices, e.g. on input data type `&[f64]`:
`benchu8, benchu16, benchu64 and benchf64`.

* Bench functions for testing algorithms that mutate their input, e.g. data type: `&mut [f64]`.
A mutable version has to be used whenever any one of the tested algorithms mutates its input:
`mutbenchu8, mutbenchu16, mutbenchu64 and mutbenchf64`.

* Bench functions for algorithms taking nd data (matrices), e.g. `&[Vec<f64>]`: `benchvvu8, benchvvu16, benchvvu64 and benchvvf64`.

## Conclusion

Please see [`tests/test.rs`](https://github.com/liborty/times/blob/main/tests/tests.rs) for examples of how to specify the closures and call these functions on them.

## Appendix - Recent Releases

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
