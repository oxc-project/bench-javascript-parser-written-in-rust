# Benchmark for Oxc, Swc and Rome parser

The purpose of this benchmark is for people who wants to evaluate and compare the performance characteristics of these parsers.

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

For multi-threaded environment, it uses the total number of logical cores as the total number of files to parser per bench iteration.

### Mac i7 6-core

oxc performs

* 2.56 times faster than swc in single-threaded environment.
* 3.17 times faster than swc in multi-threaded environment.

```
group                               base
-----                               ----
group                               base
-----                               ----
typescript.js/multi-thread/oxc      1.00   198.6±12.65ms    39.4 MB/sec
typescript.js/multi-thread/rome     1.00   963.8±34.54ms     8.1 MB/sec
typescript.js/multi-thread/swc      1.00   531.4±15.02ms    14.7 MB/sec
typescript.js/single-thread/oxc     1.00     91.3±5.82ms    85.7 MB/sec
typescript.js/single-thread/rome    1.00    382.0±8.11ms    20.5 MB/sec
typescript.js/single-thread/swc     1.00   245.2±37.59ms    31.9 MB/sec
```

<img src="./violin.svg">
