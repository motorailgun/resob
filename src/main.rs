pub mod parser;

use parser::module::Module;
use std::path;

fn main() {
    env_logger::init();

    let path = path::Path::new("examples/simple_func_with_local.wat");
    let wasm = wat::parse_file(path).unwrap();

    let parsed = Module::decode(&wasm);
    dbg!(&parsed);
    dbg!(parsed.unwrap().1);
}
