use criterion::{
    criterion_group, criterion_main, measurement::Measurement, BenchmarkGroup, BenchmarkId,
    Criterion,
};
use rayon::prelude::*;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

trait Bencher {
    type ParseOutput;

    const ID: &'static str;

    fn parse(source: &str) -> Self::ParseOutput;

    fn bench<T: Measurement>(group: &mut BenchmarkGroup<T>, source: &str) {
        let cpus = num_cpus::get_physical();
        let id = BenchmarkId::new(Self::ID, "single-thread");
        group.bench_with_input(id, &source, |b, source| b.iter(|| Self::parse(source)));

        let id = BenchmarkId::new(Self::ID, "no-drop");
        group.bench_with_input(id, &source, |b, source| {
            b.iter_with_large_drop(|| Self::parse(source))
        });

        let id = BenchmarkId::new(Self::ID, "parallel");
        group.bench_with_input(id, &source, |b, source| {
            b.iter(|| {
                (0..cpus).into_par_iter().for_each(|_| {
                    Self::parse(source);
                });
            })
        });
    }
}

struct OxcBencher;

impl Bencher for OxcBencher {
    type ParseOutput = oxc_allocator::Allocator;

    const ID: &'static str = "oxc";

    fn parse(source: &str) -> Self::ParseOutput {
        let allocator = oxc_allocator::Allocator::default();
        let source_type = oxc_span::SourceType::default();
        _ = oxc_parser::Parser::new(&allocator, source, source_type).parse();
        allocator
    }
}

struct SwcBencher;

impl Bencher for SwcBencher {
    type ParseOutput = swc_ecma_parser::PResult<swc_ecma_ast::Module>;

    const ID: &'static str = "swc";

    fn parse(source: &str) -> Self::ParseOutput {
        use swc_ecma_parser::{Parser, StringInput, Syntax};
        Parser::new(
            Syntax::Es(Default::default()),
            StringInput::new(source, Default::default(), Default::default()),
            None,
        )
        .parse_module()
    }
}

struct RomeBencher;

impl Bencher for RomeBencher {
    type ParseOutput = rome_js_parser::Parse<rome_js_syntax::JsModule>;

    const ID: &'static str = "rome";

    fn parse(source: &str) -> Self::ParseOutput {
        rome_js_parser::parse_module(source)
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let filename = "typescript.js";

    let source = std::fs::read_to_string(filename).unwrap();

    let mut group = c.benchmark_group(filename);

    OxcBencher::bench(&mut group, &source);
    SwcBencher::bench(&mut group, &source);
    RomeBencher::bench(&mut group, &source);

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
