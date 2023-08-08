use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rayon::prelude::*;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

// wget https://cdn.jsdelivr.net/npm/typescript@5.1.6/lib/typescript.js
// cargo bench

fn oxc_parse(source: &str) {
    let allocator = oxc_allocator::Allocator::default();
    let source_type = oxc_span::SourceType::default();
    _ = oxc_parser::Parser::new(&allocator, source, source_type).parse();
}

fn swc_parse(source: &str) {
    use swc_ecma_parser::{Parser, StringInput, Syntax};
    _ = Parser::new(
        Syntax::Es(Default::default()),
        StringInput::new(source, Default::default(), Default::default()),
        None,
    )
    .parse_module();
}

fn rome_parse(source: &str) {
    _ = rome_js_parser::parse_module(source);
}

fn criterion_benchmark(c: &mut Criterion) {
    let cpus = num_cpus::get();

    let filename = "typescript.js";
    let source = std::fs::read_to_string(filename).unwrap();

    let mut group = c.benchmark_group(filename);
    group.throughput(Throughput::Bytes(source.len() as u64));

    group.bench_with_input(
        BenchmarkId::new("single-thread", "oxc"),
        &source,
        |b, source| {
            b.iter(|| oxc_parse(source));
        },
    );

    group.bench_with_input(
        BenchmarkId::new("single-thread", "swc"),
        &source,
        |b, source| {
            b.iter(|| swc_parse(source));
        },
    );

    group.bench_with_input(
        BenchmarkId::new("single-thread", "rome"),
        &source,
        |b, source| {
            b.iter(|| rome_parse(source));
        },
    );

    group.bench_with_input(
        BenchmarkId::new("multi-thread", "oxc"),
        &source,
        |b, source| {
            b.iter(|| {
                (0..cpus).into_par_iter().for_each(|_| oxc_parse(source));
            });
        },
    );

    group.bench_with_input(
        BenchmarkId::new("multi-thread", "swc"),
        &source,
        |b, source| {
            b.iter(|| {
                (0..cpus).into_par_iter().for_each(|_| swc_parse(source));
            });
        },
    );

    group.bench_with_input(
        BenchmarkId::new("multi-thread", "rome"),
        &source,
        |b, source| {
            b.iter(|| {
                (0..cpus).into_par_iter().for_each(|_| rome_parse(source));
            });
        },
    );

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
