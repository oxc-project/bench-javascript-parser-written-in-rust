use std::{env, fs, path::Path};

use bench_parser::swc;

pub fn main() {
    let path = env::args().nth(1).unwrap();
    let path = Path::new(&path);
    let source_text = fs::read_to_string(path).unwrap();
    let _ = biome::swc(path, &source_text);
}
