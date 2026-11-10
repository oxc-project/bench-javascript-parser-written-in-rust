# Parser Benchmark for Oxc, SWC, SWC Next and Biome

The purpose of this benchmark is for people who wants to evaluate and compare the performance characteristics of these parsers.

The active benchmarks compare Oxc, SWC and [SWC Next](https://github.com/swc-project/swc-next). Biome is currently disabled.

## Summary

SWC Next was fastest in all six CPU cases in the latest run. In single-thread mode, Oxc took 1.36–1.75x as long and SWC took 3.32–4.07x as long as SWC Next.

## CPU

### Codspeed Measurement

[![CodSpeed Badge][codspeed-badge]][codspeed-url]

[codspeed-badge]: https://img.shields.io/endpoint?url=https://codspeed.io/badge.json
[codspeed-url]: https://codspeed.io/oxc-project/bench-javascript-parser-written-in-rust/benchmarks

Codspeed measures performance by cpu instructions.

### MacBook Pro M3 Max, 14 cores (2026-11-10)

Measured on macOS 27.0.1 with Rust 1.99.1, Oxc 0.158.0, `swc_ecma_parser` 47.0.0, SWC Next 0.6.1, `mimalloc-safe` 0.1.70 and Criterion2 3.0.5.
The run uses 100 samples per case; parallel iterations parse 14 copies of the fixture.
Values are mean elapsed times, and ratios compare each parser with the fastest parser in that row.

#### cal.com.tsx

|               | oxc              | swc               | swc-next         |
| ------------- | ---------------- | ----------------- | ---------------- |
| no-drop       | `3.1 ms` (1.72x) | `6.3 ms` (3.50x)  | `1.8 ms` (1.00x) |
| parallel      | `5.4 ms` (1.60x) | `15.2 ms` (4.53x) | `3.4 ms` (1.00x) |
| single-thread | `3.1 ms` (1.75x) | `7.2 ms` (4.07x)  | `1.8 ms` (1.00x) |

#### typescript.js

|               | oxc               | swc                | swc-next          |
| ------------- | ----------------- | ------------------ | ----------------- |
| no-drop       | `20.1 ms` (1.38x) | `40.0 ms` (2.75x)  | `14.6 ms` (1.00x) |
| parallel      | `30.7 ms` (1.33x) | `107.1 ms` (4.64x) | `23.1 ms` (1.00x) |
| single-thread | `19.8 ms` (1.36x) | `48.3 ms` (3.32x)  | `14.5 ms` (1.00x) |

### Run benchmark locally

Run the following command on your machine for replication.

```bash
cargo bench
```

Run only SWC Next benchmarks with `cargo bench -- swc-next`, or parse a single file with `cargo run --release --bin swc-next -- files/cal.com.tsx`.

Generate the table

```bash
pnpm i
pnpm run table
```

## Maximum Resident Set Size

### MacBook Pro M3 Max (2026-11-10)

Measured with `./memory.sh` on the same machine as the latest CPU run. Values are mean peak resident set sizes in decimal MB; ratios compare each parser with the lowest value in that row.

| Fixture | oxc | swc | swc-next |
| --- | --- | --- | --- |
| cal.com.tsx | `12.1 MB` (1.44x) | `16.7 MB` (1.99x) | `8.4 MB` (1.00x) |
| typescript.js | `56.8 MB` (1.53x) | `92.0 MB` (2.48x) | `37.1 MB` (1.00x) |

## Setup

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

### single-thread

This is the standard benchmark run in a single thread.

```rust
group.bench_with_input(id, &source, |b, source| {
    b.iter(|| Self::parse(source))
});
```

### no-drop

This uses the [`iter_with_large_drop`](https://docs.rs/criterion/0.5.1/criterion/struct.Bencher.html#method.iter_with_large_drop) function, which does not take AST drop time into account.
In the latest `typescript.js` run, SWC takes 48.3ms with dropping versus 40.0ms without dropping.

AST drop time can become a bottleneck in applications such as as bundler,
where there are a few thousands of files need to be parsed.

```rust
group.bench_with_input(id, &source, |b, source| {
    b.iter_with_large_drop(|| Self::parse(source))
});
```

### parallel

This benchmark uses the total number of physical cores as the total number of files to parse per bench iteration. For example it parses 14 files in parallel on the 14-core M3 Max used above.

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

# [Sponsored By](https://oxc.rs/sponsor)

<p align="center">
  <a href="https://oxc.rs/sponsor">
    <img src="https://raw.githubusercontent.com/oxc-project/sponsors/main/sponsors.svg" alt="Our sponsors" />
  </a>
</p>
