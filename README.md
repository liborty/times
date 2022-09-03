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

Random data are automatically generated using `ran` crate and then algorithms from a given array of closures are repeatedly run and their statistics are collected (median of the execution *times* and its standard error). The repeated runs reduce the temporary effects of changing machine load and changing data. The effects of outliers are minimised by using medians and MAD (median of absolute differences, i.e. the most stable measure of data dispersion). All the algorithms are run over the same data but the data is changed for each repeat run.

Standard error `(ste)` estimates the doubt about the accuracy of any (repeated) measurements. Thus high `ste` means poor accuracy. Accuracy can often be increased by increasing the number of repeats. The extraneous influence of the machine load is also reduced as the length of the data vectors increases. We redefine `ste` here for extra stability as MAD as a percentage of the median.

We generate new random data for each repeated run. The differences in `ste` between algorithms inform us about their relative stability under changing data. Some algorithms suffer from data sensitivity (poor worst-case performance) and this may be indicated by relatively high `ste`, e.g. `mutsort`.

The tests are also automatically repeated over different lengths of the input data vectors, in specified range and step. This enables comparisons of algorithms as the difficulty of the problem increases. The algorithms with lower computational complexity and/or faster implementations will start to win more convincingly at greater lengths. When the given range is too big, then the process may have to be externally terminated. Depending, of course, on the algorithms and the speed of the machine.

## Main Features

* Redefined, more stable standard error.

* Ease of Use - just specify:
  * the type of the random test data,
  * range of data vectors lengths, and step
  * the number of repeats over different random data,
  * some names to identify the algorithms by
  * the closures invoking the algorithms (in the same order as their labels).

* Sorted output.

    The algorithms are automatically sorted by their execution times within each length of data category, i.e. the fastest algorithm in each data category will be listed first and the slowest last.

## Provided Testing Functions

* Simple function `bench` for timing closures that take no or constant arguments (see `tests/tests.rs`).

* Functions for testing algorithms on input vectors of three different end-types of data: `benchu8, benchu64, benchf64`.

* Mutable versions for testing mutable algorithms: `mutbenchu8, mutbenchu64, mutbenchf64`. A mutable version has to be used whenever one of the tested algorithms mutates its input.

* Versions for algorithms working on nd data (matrices): `benchvvu8, benchvvu64 and benchvvf64`.

 Other end-types may be included later.

## Conclusion

Please see [`tests/test.rs`](https://github.com/liborty/times/blob/main/tests/tests.rs) for examples of how to specify the closures and call these functions on them.

## Appendix - Recent Releases

**Version 1.0.4** Instead of magnitudes number, now takes the actual range and step of the data lengths. Is more flexible.

**Version 1.0.3** Updated the dependencies.

**Version 1.0.2** Added simple `bench` for timing closures that take no or constant arguments.

**Version 1.0.1** Redefined standard error as MAD as a percentage of Median (more stable measure). All listed times are now medians rather than means. Also, as there are now no sums of squares of nanoseconds, the danger of overflow on very slow tests is reduced.

**Version 1.0.0** Promoted to v 1.0.0 following period of non problematic use.
