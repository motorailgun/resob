pub mod parser;

use std::path;
use parser::module::Module;

fn main() {
    env_logger::init();

    let path = path::Path::new("examples/only_simple_func.wat");
    let wasm = wat::parse_file(path).unwrap();

    let parsed = Module::decode(&wasm);
    dbg!(&parsed);
    dbg!(parsed.unwrap().1);
}
