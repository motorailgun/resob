use super::types::*;
use nom::{Err::Error, bytes::complete::{tag, take}, error::{self, ErrorKind}, multi::many0, number::complete::le_u8, sequence::pair, IResult};
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
pub enum Section {
    Custom,
    Type(TypeSection),
    Import,
    Function(FunctionSection),
    Memory,
    Export,
    Code(CodeSection),
    Data,
}


pub fn parse_section(input: &[u8]) -> IResult<&[u8], Section> {
    let (rest, (section_code, section_size)) = pair(le_u8, leb128_u32)(input)?;
    let header_length = input.len() - rest.len();

    let (input, _header) = take(header_length)(input)?;
    let (input, body) = take(section_size)(input)?;
    let scode = SectionCode::from_u8(section_code).unwrap();

    use SectionCode::*;
    match scode {
        Type => parse_type_section(body),
        Function => parse_function_section(body),
        _ => Err(Error(error::Error::new(input, ErrorKind::Fail))),
    }
}

pub fn parse_sections(input: &[u8]) -> IResult<&[u8], Sections> {    
    let mut sections = Sections::new();
    let (rest, section_vec) = many0(parse_section)(input)?;

    section_vec.into_iter().for_each(|section| {
        use Section::*;

        match section {
            Custom | Import | Memory | Export | Code | Data => (),
            Type(sect) => sections.types = Some(sect),
            Function(sect) => sections.functions = Some(sect),
        };
    });

    Ok((rest, sections))
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Sections {
    types: Option<TypeSection>,
    functions: Option<FunctionSection>,
}

impl Sections {
    pub fn new() -> Self {
        Sections {
            types: None,
            functions: None,
        }
    }
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

pub fn parse_type_section<'a>(input: &'a [u8]) -> IResult<&'a [u8], Section> {
    let (rest, function_types) = parse_vec(parse_function_type, input)?;
    Ok((rest, Section::Type(TypeSection { function_types })))
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

pub fn parse_function_section<'a>(input: &'a [u8]) -> IResult<&'a [u8], Section> {
    let (rest, table) = parse_vec(f, input)?;
    Ok((rest, Section::Function(FunctionSection{table})))
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Instruction {
    End,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct FunctionLocal {
    pub count: u32,
    pub value_type: ValueType,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Function {
    pub locals: Vec<FunctionLocal>,
    pub code: Vec<Instruction>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct CodeSection {
    pub functions: Vec<Function>,
}

pub fn parse_code_section(input: &[u8]) -> IResult<&[u8], Section> {
    let (rest, functions) = parse_vec(parse_function, input)?;
    Ok((
        rest,
        Section::Code( CodeSection {
            functions
        }),
    ))
}

pub fn parse_function(input: &[u8]) -> IResult<&[u8], Function> {
    todo!()
}

pub fn parse_function_local(input: &[u8]) -> IResult<&[u8], FunctionLocal> {
    todo!()
}

pub fn parse_instructions(input: &[u8]) -> IResult<&[u8], Instruction> {
    todo!()
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
