use nom::{bytes::complete::tag, number::complete::le_u32, IResult};

pub const WASM_MAGIC: &str = "\0asm";

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Module {
    pub magic: String,
    pub version: u32,
    pub sections: super::section::Sections,
}

impl Module {
    pub fn decode(input: &[u8]) -> IResult<&[u8], Module> {
        let (rest, magic) = tag(WASM_MAGIC)(input)?;
        let (rest, version) = le_u32(rest)?;
        let (rest, sections) = super::section::parse_sections(rest)?;

        Ok((
            rest,
            Module {
                magic: String::from_utf8_lossy(magic).into(),
                version,
                sections,
            }
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::section::Sections;
    use super::*;

    #[test]
    fn decode_header_only() -> anyhow::Result<()> {
        let wasm = wat::parse_str("(module)")?;
        let module = Module::decode(&wasm)?;
        assert_eq!(
            module,
            Module {
                magic: WASM_MAGIC.to_string(),
                version: 1,
                sections: Sections::new(),
            }
        );

        Ok(())
    }
}
