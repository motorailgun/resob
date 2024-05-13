struct TypeSection {}
struct CodeSection {}
struct FunctionSection {}
struct MemorySection {}
struct DataSection {}
struct ExportSection {}
struct ImportSection {}

pub const WASM_MAGIC: &str = "\0asm";

#[derive(Eq, PartialEq, Debug)]
pub struct Module {
    pub magic: String,
    pub version: u32,
}

impl Module {
    pub fn new(input: &Vec<u8>) -> Module {
        let magic: String = String::from_utf8_lossy(&input.get(0..=3).unwrap().to_vec()).into_owned();
        let version = u32::from_le_bytes(input.get(4..=7).unwrap()[0..4].try_into().unwrap());

        dbg!(magic.clone());

        if magic != WASM_MAGIC.to_string() {
            panic!()
        } else {
            Module {
                magic,
                version,
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_header_only() {
        let wasm = wat::parse_str("(module)").unwrap();
        let module = Module::new(&wasm);
        assert_eq!(module, Module{
            magic: WASM_MAGIC.to_string(),
            version: 1,
        });
    }
}