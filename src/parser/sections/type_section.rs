use super::types::*;
use super::Section;
use nom::{bytes::complete::tag, IResult};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct FuncType {
    pub params: Vec<ValueType>,
    pub results: Vec<ValueType>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct TypeSection {
    pub function_types: Vec<FuncType>,
}

pub fn parse_type_section(input: &[u8]) -> IResult<&[u8], Section> {
    let (rest, function_types) = parse_vec(parse_function_type, input)?;
    Ok((rest, Section::Type(TypeSection { function_types })))
}

fn parse_function_type(input: &[u8]) -> IResult<&[u8], FuncType> {
    let (body, _) = tag([0x60])(input)?;

    let (rest, params) = parse_vec(parse_value_type, body)?;
    let (rest, results) = parse_vec(parse_value_type, rest)?;

    Ok((rest, FuncType { params, results }))
}
