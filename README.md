# Times: Benchmark for Timing and Comparing Algorithms in Rust

[<img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/liborty/times/HEAD?logo=github">](https://github.com/liborty/times)
[<img alt="crates.io" src="https://img.shields.io/crates/v/times?logo=rust">](https://crates.io/crates/times)
[<img alt="crates.io" src="https://img.shields.io/crates/d/times?logo=rust">](https://crates.io/crates/times)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/times?logo=rust">](https://docs.rs/times)

## Usage

```rust
use times::*;
```

## Introduction

This crate will be typically added in `Cargo.toml` under `[dev-dependecies]`  and then used by source files under `tests` or `benches` directories. To be used whenever the runtime speed comparisons are of interest, which is practically always.

Is suitable for testing algorithms that work on a whole `Vec` of data, for example sort. The correctness of the results
should be tested separately. Here the results are thrown away and only the execution time is measured.

Random data are automatically generated using `ran` crate and then algorithms from a given array of closures are repeatedly run and their statistics are collected (arithmetic mean of the execution *times* and its standard error). The repeated runs  average out the temporary effects of changing machine load and changing data. All the algorithms are run over the same data but the data is changed for each repeat run.

Standard error (`ste`) estimates the doubt about the accuracy of any (repeated) measurements. Thus high `ste` means poor accuracy. Accuracy can generally be increased  by increasing the number of repeats. Here the extraneous influence of the machine load is also reduced as the length of the data vectors increases.

We generate new random data for each repeated run, so the differences in `ste` between algorithms inform us about the their relative stability under changing data. Some algorithms can suffer from data sensitivity (poor worst-case performance) and this may be indicated by relatively high `ste`.

The tests are also automatically repeated over different lengths of the input data vectors, in steps of their *orders of magnitude*: 10,100,1000,10000, etc. This is to enable comparisons of the algorithms' rankings of merit, as the difficulty of the problem increases. The algorithms with lower computational complexity and/or faster implementations will start convincingly winning. 

A word of warning: it is not recommended to set the magnitudes range to more than 5, as it may take a long time to run and the process may have to be externally terminated. Depending, of course, on the algorithms and the speed of the machine.

## Main Features

* Ease of Use - just specify:
  * the type of the random test data,
  * the range of magnitudes of data vectors lengths,
  * the number of repeats with different data,
  * some labels/names to identify the algorithms by
  * the closures invoking the algorithms (in the same order as their labels).

* Sorted output. The algorithms are automatically sorted by their execution times within each magnitude of data category. In other words, the fastest algorithm in each data category will be listed first and the slowest last.

## Provided Testing Functions

* Functions for testing algorithms on vectors of three different end-types of data: `benchu8, benchu64, benchf64`. 

* Mutable versions for testing mutable algorithms: `mutbenchu8, mutbenchu64, mutbenchf64`. A mutable version has to be used even when just one of the tested algorithms mutates its input.

 * Versions for algorithms working on n-dimensional data (matrices): `benchvvu8, benchvvu64 and benchvvf64`.

 Other end-types may be included later.

## Conclusion

Please see `tests/test.rs` for examples of how to specify the closures and call these functions on them.

These benchmarks show that the fastest standard Rust destructive sort, labelled as `mutsort` here, is indeed the fastest but only up to about 100 sort items. Then all three versions of my `hashsort` overtake it.

## Appendix - Recent Releases

**Version 0.1.6** Corrected a minor bug in report headline.

**Version 0.1.5** Corrected the report headlines. Introduced standard errors. Added `benchvvu64`.

**Version 0.1.4** Added `benchvvu8` for closures acting on (immutable) `Vec<Vec<u8>>` and similarly `benchvvf64` for `Vec<Vec<f64>>`.

**Version 0.1.3** Benchmark functions for `Vec` data types: u8,u64,f64, in plain and mutable varieties
