struct Type {}
struct Code {}
struct Function {}
struct Memory {}
struct Data {}
struct Export {}
struct Import {}

pub const WASM_MAGIC: &str = "\0asm";

#[derive(Eq, PartialEq, Debug)]
pub struct Module {
    pub magic: String,
    pub version: u32,
}

#[derive(Eq, PartialEq, Debug, num_derive::FromPrimitive)]
pub enum SectionCode {
    Type = 0x01,
    Import = 0x02,
    Function = 0x03,
    Memory = 0x05,
    Export = 0x07,
    Code = 0x0a,
    Data = 0x0b,
}

#[derive(Eq, PartialEq, Debug)]
enum Section {
    Type,
    Import,
    Function,
    Memory,
    Export,
    Code,
    Data,
}

impl Section {
    pub fn new(input: &Vec<u8>) -> Section {
        todo!()
    }
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
