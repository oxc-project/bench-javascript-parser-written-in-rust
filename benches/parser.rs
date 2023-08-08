use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rayon::prelude::*;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn oxc_parse(source: &str) -> oxc_allocator::Allocator {
    let allocator = oxc_allocator::Allocator::default();
    let source_type = oxc_span::SourceType::default();
    _ = oxc_parser::Parser::new(&allocator, source, source_type).parse();
    allocator
}

fn swc_parse(source: &str) -> swc_ecma_parser::PResult<swc_ecma_ast::Module> {
    use swc_ecma_parser::{Parser, StringInput, Syntax};
    Parser::new(
        Syntax::Es(Default::default()),
        StringInput::new(source, Default::default(), Default::default()),
        None,
    )
    .parse_module()
}

fn rome_parse(source: &str) -> rome_js_parser::Parse<rome_js_syntax::JsModule> {
    rome_js_parser::parse_module(source)
}

fn criterion_benchmark(c: &mut Criterion) {
    let cpus = num_cpus::get_physical();
    let filename = "typescript.js";
    let source = std::fs::read_to_string(filename).unwrap();

    let mut group = c.benchmark_group(filename);
    group.throughput(Throughput::Bytes(source.len() as u64));

    let id = "single-thread";
    group.bench_with_input(BenchmarkId::new(id, "oxc"), &source, |b, source| {
        b.iter(|| oxc_parse(source))
    });

    group.bench_with_input(BenchmarkId::new(id, "swc"), &source, |b, source| {
        b.iter(|| swc_parse(source))
    });

    group.bench_with_input(BenchmarkId::new(id, "rome"), &source, |b, source| {
        b.iter(|| rome_parse(source))
    });

    let id = "single-thread-no-drop";
    group.bench_with_input(BenchmarkId::new(id, "oxc"), &source, |b, source| {
        b.iter_with_large_drop(|| oxc_parse(source))
    });

    group.bench_with_input(BenchmarkId::new(id, "swc"), &source, |b, source| {
        b.iter_with_large_drop(|| swc_parse(source))
    });

    group.bench_with_input(BenchmarkId::new(id, "rome"), &source, |b, source| {
        b.iter_with_large_drop(|| rome_parse(source))
    });

    let id = "multi-thread";
    group.bench_with_input(BenchmarkId::new(id, "oxc"), &source, |b, source| {
        b.iter(|| {
            (0..cpus).into_par_iter().for_each(|_| {
                oxc_parse(source);
            });
        })
    });

    group.bench_with_input(BenchmarkId::new(id, "swc"), &source, |b, source| {
        b.iter(|| {
            (0..cpus).into_par_iter().for_each(|_| {
                _ = swc_parse(source);
            });
        });
    });

    group.bench_with_input(BenchmarkId::new(id, "rome"), &source, |b, source| {
        b.iter(|| {
            (0..cpus).into_par_iter().for_each(|_| {
                rome_parse(source);
            });
        });
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
