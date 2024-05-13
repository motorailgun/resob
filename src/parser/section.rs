use anyhow::Result;
use nom::{
    bytes::complete::take, number::complete::le_u8, sequence::pair, IResult
};
use nom_leb128::leb128_u32;
use num_traits::FromPrimitive;

#[derive(Eq, PartialEq, Debug, num_derive::FromPrimitive)]
pub enum SectionCode {
    Custom = 0x00,
    Type = 0x01,
    Import = 0x02,
    Function = 0x03,
    Memory = 0x05,
    Export = 0x07,
    Code = 0x0a,
    Data = 0x0b,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Section {
    pub code: SectionCode,
    pub size: u32,
    pub header_length: usize,
    pub inner: Vec<u8>,
}

impl Section {
    pub fn parse_section(input: &[u8]) -> IResult<&[u8], Section> {
        let (rest, (section_code, section_size)) = pair(le_u8, leb128_u32)(input)?;
        let header_length = input.len() - rest.len();

        let (input, _header) = take(header_length)(input)?;
        let (input, body) = take(section_size)(input)?;

        Ok((input, Section {
            code: SectionCode::from_u8(section_code).expect("invalid section code"),
            size: section_size,
            header_length,
            inner: body.to_vec().clone(),
        }))
    }
}

pub fn parse_sections(input: &Vec<u8>) -> Result<Vec<Section>> {
    let mut sections = Vec::new();
    let mut input = input as &[u8];

    while let Ok((rest, section)) = Section::parse_section(input) {
        sections.push(section);
        input = rest;
    }

    Ok(sections)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_simple_func_section() {
        let wasm = wat::parse_str("(module (func))").unwrap();
        let stripped_body = wasm[8..].to_vec();
        let (_, section) = Section::parse_section(&stripped_body).unwrap();

        dbg!(&section);

        assert_eq!(
            section,
            Section {
                code: SectionCode::Type,
                size: 0x04,
                header_length: 2,
                inner: stripped_body[2..].to_vec(),
            }
        );
    }
}
