pub mod parser;
pub mod executor;

use parser::module::Module;
use std::path;

fn main() {
    env_logger::init();

    let path = path::Path::new("examples/simple_func_add.wat");
    let wasm = wat::parse_file(path).unwrap();

    let parsed = Module::decode(&wasm);
    dbg!(&parsed);
    dbg!(parsed.unwrap().1);
}
