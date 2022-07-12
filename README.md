# Times

[<img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/liborty/times/HEAD?logo=github">](https://github.com/liborty/times)
[<img alt="crates.io" src="https://img.shields.io/crates/v/times?logo=rust">](https://crates.io/crates/times)
[<img alt="crates.io" src="https://img.shields.io/crates/d/times?logo=rust">](https://crates.io/crates/times)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/times?logo=rust">](https://docs.rs/times)

# Times: Benchmark for Timing and Comparing Algorithms in Rust

## Usage

```rust
use times::*;
```

## Introduction

This crate will be typically included under [dev-dependecies] in `Cargo.toml` and then used by source files under `tests` or `benches` directories. Whenever the runtime speed comparisons are of interest, which they are practically always.

Suitable for testing algorithms that work on a whole Vec of data, for example sort. The correctness of the results has to be tested separately. Here the results are thrown away and only the execution time is measured.

Random data are automatically generated using `ran` crate and then algorithms from an array of closures are repeatedly run, timed and their statistics are collected (arithmetic mean execution time and its standard deviation). This will average out the temporary effects of changing machine load, and changing data. All the algorithms are run over the same data but the data is changed for each repeat.

The standard deviation is here an inverse measure of the timing reliability. The reliability will be improved by increasing the number of runs. The same as when performing  any measurements.

The tests are also automatically repeated over different lengths of the input data vectors, in steps of their magnitudes: 10,100,1000,10000, etc.. This makes it easy to compare the algorithms' comparative rankings, as the difficulty of the problem increases. The ones with lower overall computational complexity and/or faster implementations, will start convincingly winning.

## Main Features

* Ease of Use - just specify as arguments:
    * the type of the random test data generator,
    * the range of magnitudes (zeros),
    * the number of repeats,
    * some arbitrary names to identify the algorithms by
    * the closures invoking the algorithms (in the same order as their names). 

The algorithms are automatically sorted by their execution times, separately within each magnitude of data category. In other workds, the fastest algorithm in each data category will be always listed first and the slowest last.

## Conclusion

Future work: make it easier to change the end data_type. At the moment it is possible but it requires changing the hard-wired type in several places in the code (possibly with search-replace.) 

Please see `tests/tests.rs` for an example application. From the benchmark we conclude, that the fastest standard Rust destructive sort, named here `mutsort`, is indeed the best for up to about 100 items. Then all three versions fo my `hashsort` overtake it. The merit order of the algorithms then remains fixed from about 1000 upwards.
