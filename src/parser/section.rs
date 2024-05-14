use super::types::*;
use anyhow::Result;
use nom::{bytes::complete::{tag, take}, number::complete::le_u8, sequence::pair, IResult};
use nom_leb128::leb128_u32;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Eq, PartialEq, Debug, FromPrimitive, Clone)]
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

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Section {
    pub code: SectionCode,
    pub size: u32,
    pub header_length: usize,
    pub body: Vec<u8>,
}

impl Section {
    pub fn parse_section(input: &[u8]) -> IResult<&[u8], Section> {
        let (rest, (section_code, section_size)) = pair(le_u8, leb128_u32)(input)?;
        let header_length = input.len() - rest.len();

        let (input, _header) = take(header_length)(input)?;
        let (input, body) = take(section_size)(input)?;

        Ok((
            input,
            Section {
                code: SectionCode::from_u8(section_code).expect("invalid section code"),
                size: section_size,
                header_length,
                body: body.to_vec().clone(),
            },
        ))
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

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Sections {
    types: Option<TypeSection>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct FuncType {
    pub params: Vec<ValueType>,
    pub results: Vec<ValueType>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct TypeSection {
    pub function_types: Vec<FuncType>,
}

pub fn parse_type_section<'a>(input: &'a [u8]) -> IResult<&'a [u8], TypeSection> {
    let (rest, function_types) = parse_vec(parse_function_type, input)?;
    Ok((rest, TypeSection { function_types }))
}

fn parse_function_type(input: &[u8]) -> IResult<&[u8], FuncType> {
    let (body, _) = tag([0x60])(input)?;

    let (rest, params) = parse_vec(parse_value_type, body)?;
    let (rest, results) = parse_vec(parse_value_type, rest)?;

    Ok((rest, FuncType { params, results }))
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct FunctionSection {
    table: Vec<u32>,
}

// needed for type assertion
fn f(i: &[u8]) -> IResult<&[u8], u32> {
    leb128_u32(i)
}

pub fn parse_function_section<'a>(input: &'a [u8]) -> IResult<&'a [u8], FunctionSection> {
    let (rest, table) = parse_vec(f, input)?;
    Ok((rest, FunctionSection{table}))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_simple_sections() {
        let wasm = wat::parse_str("(module (func))").unwrap();
        let stripped_body = wasm[8..].to_vec();
        let sections = parse_sections(&stripped_body).unwrap();

        dbg!(&sections);

        let sec_vec = vec![
            Section {
                code: SectionCode::Type,
                size: 0x04,
                header_length: 2,
                body: vec![0x01, 0x60, 0x00, 0x00],
            },
            Section {
                code: SectionCode::Function,
                size: 0x02,
                header_length: 2,
                body: vec![0x01, 0x00],
            },
            Section {
                code: SectionCode::Code,
                size: 0x04,
                header_length: 2,
                body: vec![0x01, 0x02, 0x00, 0x0b]
            }
        ];

        assert_eq!(sections, sec_vec);
    }

    #[test]
    fn decode_simple_type_section() {
        let wasm = wat::parse_str("(module (func))").unwrap();
        let stripped_section = wasm[10..14].to_vec();
        let (_, section) = parse_type_section(&stripped_section).unwrap();

        dbg!(stripped_section);

        assert_eq!(
            section,
            TypeSection {
                function_types: vec![
                    FuncType {
                        params: vec![],
                        results: vec![],
                    }
                ],
            }
        )
    }

    #[test]
    fn decode_simple_func_section() {
        let wasm = wat::parse_str("(module (func))").unwrap();
        let stripped_section = wasm[16..18].to_vec();
        let (_, section) = parse_function_section(&stripped_section).unwrap();

        dbg!(stripped_section);
        assert_eq!(section, FunctionSection{ table: vec![0x00] })
    }
}
