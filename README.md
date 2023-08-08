# Benchmark for Oxc, Swc and Rome parser

The purpose of this benchmark is for people who wants to evaluate and compare the performance characteristics of these parsers.

## Results

### Mac i7 6 cores

#### oxc vs swc

* 2.68 times faster in single-threaded environment.
* 2.67 times faster in multi-threaded environment.

#### oxc vs rome

* 4.18 times faster in single-threaded environment.
* 4.85 times faster in multi-threaded environment.

```
group                               base
-----                               ----
typescript.js/single-thread/oxc     1.00     91.3±5.82ms    85.7 MB/sec
typescript.js/single-thread/swc     1.00   245.2±37.59ms    31.9 MB/sec
typescript.js/single-thread/rome    1.00    382.0±8.11ms    20.5 MB/sec

typescript.js/multi-thread/oxc      1.00   198.6±12.65ms    39.4 MB/sec
typescript.js/multi-thread/swc      1.00   531.4±15.02ms    14.7 MB/sec
typescript.js/multi-thread/rome     1.00   963.8±34.54ms     8.1 MB/sec
```

<img src="./bar-graph.svg">

### Mac M2 8 cores

#### oxc vs swc

* 2.67 times faster in single-threaded environment.
* 2.43 times faster in single-threaded-no-drop environment.
* 2.61 times faster in multi-threaded environment.

#### oxc vs rome

* 3.87 times faster in single-threaded environment.
* 3.66 times faster in single-threaded-no-drop environment.
* 4.38 times faster in multi-threaded environment.

#### AST drop

Notice there is a significant AST drop time for swc (132.2 - 119.1 = 14.1ms) but not for oxc (49.4 - 48.6 = 0.8ms).
This is due to allocating the oxc AST into a memory arena ([bumpalo](https://crates.io/crates/bumpalo)).

```
group                                       base
-----                                       ----
typescript.js/single-thread/oxc             1.00     49.4±1.73ms   158.3 MB/sec
typescript.js/single-thread/swc             1.00    132.2±1.36ms    59.2 MB/sec
typescript.js/single-thread/rome            1.00    191.4±0.91ms    40.9 MB/sec

typescript.js/single-thread-no-drop/oxc     1.00     48.6±0.50ms   161.0 MB/sec
typescript.js/single-thread-no-drop/swc     1.00    118.1±0.59ms    66.3 MB/sec
typescript.js/single-thread-no-drop/rome    1.00    178.0±1.39ms    44.0 MB/sec

typescript.js/multi-thread/oxc              1.00    82.0±12.43ms    95.5 MB/sec
typescript.js/multi-thread/swc              1.00    214.3±5.45ms    36.5 MB/sec
typescript.js/multi-thread/rome             1.00    359.5±8.11ms    21.8 MB/sec
```

## Run

Run the following command on your machine for replication.

```bash
cargo bench
```

## Input

* File: https://cdn.jsdelivr.net/npm/typescript@5.1.6/lib/typescript.js
* File Size: 7.8M
* Uses `mimalloc` as the global allocator
* Uses the following release profile

```toml
[profile.release]
opt-level     = 3
lto           = "fat"
codegen-units = 1
strip         = "symbols"
debug         = false
panic         = "abort"
```

## Environments
* For single-threaded-no-drop environment, the timing does not take AST drop into account, see [`iter_with_large_drop`](https://docs.rs/criterion/0.5.1/criterion/struct.Bencher.html#method.iter_with_large_drop).
* For multi-threaded environment, the benchmark uses the total number of physical cores as the total number of files to parse per bench iteration. For example it parses 6 files in parallel for my Mac i7 6 cores.
