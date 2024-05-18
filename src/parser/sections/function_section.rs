use super::types::*;
use super::Section;
use nom::IResult;
use nom_leb128::leb128_u32;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct FunctionSection {
    table: Vec<u32>,
}

// needed for type assertion
fn f(i: &[u8]) -> IResult<&[u8], u32> {
    leb128_u32(i)
}

pub fn parse_function_section(input: &[u8]) -> IResult<&[u8], Section> {
    let (rest, table) = parse_vec(f, input)?;
    Ok((rest, Section::Function(FunctionSection { table })))
}
