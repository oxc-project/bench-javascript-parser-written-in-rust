pub mod oxc {
    use std::path::Path;

    use oxc::{allocator::Allocator, parser::Parser, span::SourceType};

    pub fn parse(path: &Path, source: &str) -> Allocator {
        let allocator = Allocator::default();
        let source_type = SourceType::from_path(path).unwrap();
        _ = Parser::new(&allocator, source, source_type).parse();
        allocator
    }
}

pub mod swc {
    use std::path::Path;

    use swc_ecma_ast::Module;
    use swc_ecma_parser::{EsSyntax, Parser, StringInput, Syntax, TsSyntax};

    pub fn parse(path: &Path, source: &str) -> Module {
        let syntax = match path.extension().unwrap().to_str().unwrap() {
            "js" => Syntax::Es(EsSyntax::default()),
            "tsx" => Syntax::Typescript(TsSyntax {
                tsx: true,
                ..TsSyntax::default()
            }),
            _ => panic!("need to define syntax  for swc"),
        };
        let input = StringInput::new(source, Default::default(), Default::default());
        Parser::new(syntax, input, None).parse_module().unwrap()
    }
}

pub mod biome {
    use std::path::Path;

    use biome_js_parser::{JsParserOptions, Parse};
    use biome_js_syntax::{AnyJsRoot, JsFileSource};

    pub fn parse(path: &Path, source: &str) -> Parse<AnyJsRoot> {
        let options = JsParserOptions::default();
        let source_type = JsFileSource::try_from(path).unwrap();
        biome_js_parser::parse(source, source_type, options)
    }
}
