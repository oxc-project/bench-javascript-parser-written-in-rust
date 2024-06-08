use std::path::Path;

use criterion::{measurement::WallTime, *};
use rayon::prelude::*;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

trait TheBencher {
    type ParseOutput;

    const ID: &'static str;

    fn parse(filename: &Path, source: &str) -> Self::ParseOutput;

    fn bench(g: &mut BenchmarkGroup<'_, WallTime>, path: &Path, source: &str) {
        let cpus = num_cpus::get_physical();
        let id = BenchmarkId::new(Self::ID, "single-thread");
        g.bench_with_input(id, &source, |b, source| {
            b.iter(|| Self::parse(path, source))
        });

        let id = BenchmarkId::new(Self::ID, "no-drop");
        g.bench_with_input(id, &source, |b, source| {
            b.iter_with_large_drop(|| Self::parse(path, source))
        });

        let id = BenchmarkId::new(Self::ID, "parallel");
        g.bench_with_input(id, &source, |b, source| {
            b.iter(|| {
                (0..cpus).into_par_iter().for_each(|_| {
                    Self::parse(path, source);
                });
            })
        });
    }
}

struct OxcBencher;

impl TheBencher for OxcBencher {
    type ParseOutput = oxc::allocator::Allocator;

    const ID: &'static str = "oxc";

    fn parse(path: &Path, source: &str) -> Self::ParseOutput {
        let allocator = oxc::allocator::Allocator::default();
        let source_type = oxc::span::SourceType::from_path(path).unwrap();
        _ = oxc::parser::Parser::new(&allocator, source, source_type).parse();
        allocator
    }
}

struct SwcBencher;

impl TheBencher for SwcBencher {
    type ParseOutput = swc_ecma_parser::PResult<swc_ecma_ast::Module>;

    const ID: &'static str = "swc";

    fn parse(path: &Path, source: &str) -> Self::ParseOutput {
        use swc_ecma_parser::{EsConfig, Parser, StringInput, Syntax, TsConfig};
        let syntax = match path.extension().unwrap().to_str().unwrap() {
            "js" => Syntax::Es(EsConfig::default()),
            "tsx" => Syntax::Typescript(TsConfig {
                tsx: true,
                ..TsConfig::default()
            }),
            _ => panic!("need to define syntax  for swc"),
        };
        Parser::new(
            syntax,
            StringInput::new(source, Default::default(), Default::default()),
            None,
        )
        .parse_module()
    }
}

struct BiomeBencher;

impl TheBencher for BiomeBencher {
    type ParseOutput = biome_js_parser::Parse<biome_js_syntax::AnyJsRoot>;

    const ID: &'static str = "biome";

    fn parse(path: &Path, source: &str) -> Self::ParseOutput {
        let options = biome_js_parser::JsParserOptions::default();
        let source_type = biome_js_syntax::JsFileSource::try_from(path).unwrap();
        biome_js_parser::parse(source, source_type, options)
    }
}

fn parser_benchmark(c: &mut Criterion) {
    let filenames = ["typescript.js", "cal.com.tsx"];
    for filename in filenames {
        let path = Path::new("files").join(filename);
        let source = std::fs::read_to_string(&path).unwrap();
        let mut g = c.benchmark_group(filename);
        OxcBencher::bench(&mut g, &path, &source);
        SwcBencher::bench(&mut g, &path, &source);
        BiomeBencher::bench(&mut g, &path, &source);
        g.finish();
    }
}

criterion_group!(parser, parser_benchmark);
criterion_main!(parser);
