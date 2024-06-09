# Parser Benchmark for Oxc, Swc and Biome

The purpose of this benchmark is for people who wants to evaluate and compare the performance characteristics of these parsers.

The numbers indicate that Oxc is at least 3 times faster than Swc and 5 times faster than Biome.

## Results

### Codspeed

[![CodSpeed Badge][codspeed-badge]][codspeed-url]

[codspeed-badge]: https://img.shields.io/endpoint?url=https://codspeed.io/badge.json
[codspeed-url]: https://codspeed.io/oxc-project/bench-javascript-parser-written-in-rust/benchmarks

Codspeed measures performance by cpu instructions.

### Mac i7 6 cores

<img src="./bar-graph.svg">

### Mac mini M2 8 cores

#### cal.com.tsx

|               | oxc              | swc               |biome             |
| ------------- | ---------------- | ----------------- |----------------- |
| no-drop       | `3.9 ms` (1.00x) | `15.9 ms` (4.03x) |`20.8 ms` (5.29x) |
| parallel      | `7.4 ms` (1.00x) | `32.4 ms` (4.40x) |`40.6 ms` (5.50x) |
| single-thread | `3.9 ms` (1.00x) | `16.5 ms` (4.21x) |`20.8 ms` (5.30x) |

#### typescript.js

|               | oxc               | swc                |biome              |
| ------------- | ----------------- | ------------------ |------------------ |
| no-drop       | `29.4 ms` (1.00x) | `111.5 ms` (3.79x) |`159.0 ms` (5.41x) |
| parallel      | `49.5 ms` (1.00x) | `236.7 ms` (4.78x) |`337.9 ms` (6.83x) |
| single-thread | `29.4 ms` (1.00x) | `113.2 ms` (3.85x) |`167.9 ms` (5.71x) |

#### single-thread

This is the standard benchmark run in a single thread.

```rust
group.bench_with_input(id, &source, |b, source| {
    b.iter(|| Self::parse(source))
});
```

#### no-drop

This uses the [`iter_with_large_drop`](https://docs.rs/criterion/0.5.1/criterion/struct.Bencher.html#method.iter_with_large_drop) function, which does not take AST drop time into account.
Notice there is only a 0.3ms difference for oxc, but 7ms difference for swc.

AST drop time can become a bottleneck in applications such as as bundler,
where there are a few thousands of files need to be parsed.

```rust
group.bench_with_input(id, &source, |b, source| {
    b.iter_with_large_drop(|| Self::parse(source))
});
```

#### parallel

This benchmark uses the total number of physical cores as the total number of files to parse per bench iteration. For example it parses 6 files in parallel on my Mac i7 6 cores.

This can indicate the existence of global resource contention.

```rust
let cpus = num_cpus::get_physical();
group.bench_with_input(id, &source, |b, source| {
    b.iter(|| {
        (0..cpus).into_par_iter().for_each(|_| {
            Self::parse(source);
        });
    })
});
```

## Run

Run the following command on your machine for replication.

```bash
cargo bench
```

Generate the table

```bash
pnpm i
pnpm run table
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
