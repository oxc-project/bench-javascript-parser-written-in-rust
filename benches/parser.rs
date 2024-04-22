use criterion::{measurement::WallTime, *};
use rayon::prelude::*;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

trait TheBencher {
    type ParseOutput;

    const ID: &'static str;

    fn parse(source: &str) -> Self::ParseOutput;

    fn bench(g: &mut BenchmarkGroup<'_, WallTime>, source: &str) {
        let cpus = num_cpus::get_physical();
        let id = BenchmarkId::new(Self::ID, "single-thread");
        g.bench_with_input(id, &source, |b, source| b.iter(|| Self::parse(source)));

        let id = BenchmarkId::new(Self::ID, "no-drop");
        g.bench_with_input(id, &source, |b, source| {
            b.iter_with_large_drop(|| Self::parse(source))
        });

        let id = BenchmarkId::new(Self::ID, "parallel");
        g.bench_with_input(id, &source, |b, source| {
            b.iter(|| {
                (0..cpus).into_par_iter().for_each(|_| {
                    Self::parse(source);
                });
            })
        });
    }
}

struct OxcBencher;

impl TheBencher for OxcBencher {
    type ParseOutput = oxc::allocator::Allocator;

    const ID: &'static str = "oxc";

    fn parse(source: &str) -> Self::ParseOutput {
        let allocator = oxc::allocator::Allocator::default();
        let source_type = oxc::span::SourceType::default();
        _ = oxc::parser::Parser::new(&allocator, source, source_type).parse();
        allocator
    }
}

struct SwcBencher;

impl TheBencher for SwcBencher {
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

struct BiomeBencher;

impl TheBencher for BiomeBencher {
    type ParseOutput = biome_js_parser::Parse<biome_js_syntax::JsModule>;

    const ID: &'static str = "biome";

    fn parse(source: &str) -> Self::ParseOutput {
        biome_js_parser::parse_module(source, biome_js_parser::JsParserOptions::default())
    }
}

fn parser_benchmark(c: &mut Criterion) {
    let filename = "typescript.js";
    let source = std::fs::read_to_string(filename).unwrap();

    let mut g = c.benchmark_group(filename);
    OxcBencher::bench(&mut g, &source);
    SwcBencher::bench(&mut g, &source);
    BiomeBencher::bench(&mut g, &source);
    g.finish();
}

criterion_group!(parser, parser_benchmark);
criterion_main!(parser);
