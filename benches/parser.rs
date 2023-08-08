use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rayon::prelude::*;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

// wget https://cdn.jsdelivr.net/npm/typescript@5.1.6/lib/typescript.js
// cargo bench

fn oxc_parse(source_text: &str) {
    use oxc::{allocator::Allocator, parser::Parser, span::SourceType};
    let allocator = Allocator::default();
    let source_type = SourceType::default();
    Parser::new(&allocator, source_text, source_type).parse();
}

fn swc_parse(source_text: &str) {
    use swc_ecma_parser::{Parser, StringInput, Syntax};
    _ = Parser::new(
        Syntax::Es(Default::default()),
        StringInput::new(source_text, Default::default(), Default::default()),
        None,
    )
    .parse_module();
}

fn criterion_benchmark(c: &mut Criterion) {
    let filename = "typescript.js";
    let source = std::fs::read_to_string(filename).unwrap();

    let mut group = c.benchmark_group(filename);
    group.sample_size(30);
    group.throughput(Throughput::Bytes(source.len() as u64));

    group.bench_with_input(
        BenchmarkId::new("oxc", "single-thread"),
        &source,
        |b, source| {
            b.iter(|| oxc_parse(source));
        },
    );

    group.bench_with_input(
        BenchmarkId::new("swc", "single-thread"),
        &source,
        |b, source| {
            b.iter(|| swc_parse(source));
        },
    );

    group.bench_with_input(
        BenchmarkId::new("oxc", "multi-thread"),
        &source,
        |b, source| {
            b.iter(|| {
                (0..50).into_par_iter().for_each(|_| oxc_parse(source));
            });
        },
    );

    group.bench_with_input(
        BenchmarkId::new("swc", "multi-thread"),
        &source,
        |b, source| {
            b.iter(|| {
                (0..50).into_par_iter().for_each(|_| swc_parse(source));
            });
        },
    );

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
