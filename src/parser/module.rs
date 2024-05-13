use anyhow::{ensure, Context, Result};

pub const WASM_MAGIC: &str = "\0asm";

#[derive(Eq, PartialEq, Debug)]
pub struct Module {
    pub magic: String,
    pub version: u32,
    pub sections: Vec<super::section::Section>,
}

impl Module {
    pub fn new(input: &Vec<u8>) -> Result<Module> {
        let (version, offset) = Self::decode(input)?;

        Ok(Module {
            magic: WASM_MAGIC.into(),
            version,
            sections: super::section::parse_sections(
                &input.get(offset..).unwrap_or_else(|| &[]).to_vec(),
            )?,
        })
    }

    pub fn decode(input: &Vec<u8>) -> Result<(u32, usize)> {
        let magic_part = input.get(0..=3).context("input less than 4 bytes")?;
        let version_part = input.get(4..=7).context("input less than 8 bytes")?;

        let magic: String = String::from_utf8_lossy(magic_part).into();
        let version = u32::from_le_bytes(version_part.try_into()?);

        ensure!(magic == WASM_MAGIC.to_string(), "magic number mismatch!");

        Ok((version, 8usize))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_header_only() -> anyhow::Result<()> {
        let wasm = wat::parse_str("(module)")?;
        let module = Module::new(&wasm)?;
        assert_eq!(
            module,
            Module {
                magic: WASM_MAGIC.to_string(),
                version: 1,
                sections: vec![],
            }
        );

        Ok(())
    }
}
