#[cfg(not(codspeed))]
pub use criterion::*;

#[cfg(codspeed)]
pub use codspeed_criterion_compat::*;

use rayon::prelude::*;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

trait Bencher {
    type ParseOutput;

    const ID: &'static str;

    fn parse(source: &str) -> Self::ParseOutput;

    fn bench(c: &mut Criterion, filename: &str, source: &str) {
        let mut group = c.benchmark_group(Self::ID);

        let cpus = num_cpus::get_physical();
        let id = BenchmarkId::new(filename, "single-thread");
        group.bench_with_input(id, &source, |b, source| b.iter(|| Self::parse(source)));

        let id = BenchmarkId::new(filename, "no-drop");
        group.bench_with_input(id, &source, |b, source| {
            b.iter_with_large_drop(|| Self::parse(source))
        });

        let id = BenchmarkId::new(filename, "parallel");
        group.bench_with_input(id, &source, |b, source| {
            b.iter(|| {
                (0..cpus).into_par_iter().for_each(|_| {
                    Self::parse(source);
                });
            })
        });

        group.finish();
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

fn parser_benchmark(c: &mut Criterion) {
    let filename = "typescript.js";

    let source = std::fs::read_to_string(filename).unwrap();

    OxcBencher::bench(c, filename, &source);
    SwcBencher::bench(c, filename, &source);
    RomeBencher::bench(c, filename, &source);
}

criterion_group!(parser, parser_benchmark);
criterion_main!(parser);
