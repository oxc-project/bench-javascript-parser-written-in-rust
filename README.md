# Benchmark for Oxc and Swc

The purpose of this benchmark is for people who wants to evaluate and compare the performance characteristics of the two parsers.

## Run

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

### Mac i7 6-core

oxc performs

* 2.56 times faster than swc in single-threaded environment.
* 3.17 times faster than swc in multi-threaded environment.

```
group                              base
-----                              ----
typescript.js/oxc/single-thread    1.00    96.5±12.03ms    81.1 MB/sec
typescript.js/swc/single-thread    1.00    246.5±5.05ms    31.8 MB/sec
typescript.js/oxc/multi-thread     1.00   913.3±47.24ms     8.6 MB/sec
typescript.js/swc/multi-thread     1.00       2.9±0.19s     2.7 MB/sec
```

<img src="./violin.svg">
