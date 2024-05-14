use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::{tag, take}, multi::count, number::complete::le_u8, sequence::{self, pair, tuple}, IResult
};
use nom_leb128::leb128_u32;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use super::types::*;

#[derive(Eq, PartialEq, Debug, FromPrimitive)]
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
    pub body: Vec<u8>,
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
            body: body.to_vec().clone(),
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

#[derive(Eq, PartialEq, Debug)]
pub struct FuncType {
    pub params: Vec<ValueType>,
    pub results: Vec<ValueType>
}

#[derive(Eq, PartialEq, Debug)]
pub struct TypeSection {
    pub function_types: Vec<FuncType>,
}

fn parse_type_section(input: &[u8]) -> IResult<&[u8], TypeSection> {
    let (body, num_types) = leb128_u32(input)?;
    let (rest, function_types) = count(parse_function_type, num_types as usize)(body)?;

    Ok((rest, TypeSection{function_types}))
}

fn parse_function_type(input: &[u8]) -> IResult<&[u8], FuncType> {
    let (body, func) = le_u8(input)?;
    if func != 0x60 {
        return Err(nom::Err::Incomplete(nom::Needed::new(1)))
    }
    let (rest, num_params) = leb128_u32(body)?;
    let (rest, params) = count(parse_value_type, num_params as usize)(rest)?;
    let (rest, num_results) = leb128_u32(rest)?;
    let (rest, results) = count(parse_value_type, num_results as usize)(rest)?;

    Ok((
        rest,
        FuncType{
            params,
            results,
        }
    ))
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

        let Section {
            code, size, header_length, body
        } = section;

        assert_eq!(code, SectionCode::Type);
        assert_eq!(size, 0x04);
        assert_eq!(header_length, 2);
        assert_eq!(body.len(), size as usize)
    }
}
