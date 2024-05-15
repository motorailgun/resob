use super::types::*;
use log::*;
use nom::{bytes::complete::{tag, take}, error::{self, ErrorKind}, multi::many0, number::complete::le_u8, sequence::{pair, terminated}, Err::Error, IResult};
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
    Custom(GenericSection),
    Type(TypeSection),
    Import(GenericSection),
    Function(FunctionSection),
    Memory(GenericSection),
    Export(GenericSection),
    Code(CodeSection),
    Data(GenericSection),
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct GenericSection {}


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
            Custom(s) => sections.custom = Some(s),
            Import(s) => sections.import = Some(s),
            Memory(s) => sections.memory = Some(s),
            Export(s) => sections.export = Some(s),
            Data(s) => sections.data = Some(s),
            Type(sect) => sections.types = Some(sect),
            Function(sect) => sections.functions = Some(sect),
            Code(sect) => sections.code = Some(sect),
        };
    });

    Ok((rest, sections))
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Sections {
    types: Option<TypeSection>,
    functions: Option<FunctionSection>,
    code: Option<CodeSection>,
    custom: Option<GenericSection>,
    import: Option<GenericSection>,
    memory: Option<GenericSection>,
    export: Option<GenericSection>,
    data: Option<GenericSection>,
}

impl Sections {
    pub fn new() -> Self {
        Sections {
            types: None,
            functions: None,
            code: None,
            custom: None,
            import: None,
            memory: None,
            export: None,
            data: None,
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

#[derive(Eq, PartialEq, Debug, Clone, FromPrimitive)]
pub enum Instruction {
    Unreachable = 0x00,
    Nop = 0x01,
    Block = 0x02,
    Loop = 0x03,
    If = 0x04,
    Else = 0x05,
    Br = 0x0c,
    BrIf = 0x0d,
    BrTable = 0x0e,
    Return = 0x0f,
    Call = 0x10,
    CallIndirect = 0x11,
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
    let (rest, size) = leb128_u32(input)?;
    let (remining, body) = take(size as usize)(rest)?;

    let (rest, locals) = parse_vec(parse_function_local, &body)?;
    let (rest, code) = terminated(many0(parse_instruction), tag([0x0b]))(rest)?;

    if rest.len() != 0 {
        warn!("parse_instruction() didn't consume all the bytes, remaining: {}", rest.len());
    }

    Ok((
        remining,
        Function {
            locals,
            code,
        }
    ))
}

pub fn parse_function_local(input: &[u8]) -> IResult<&[u8], FunctionLocal> {
    let (rest, count) = leb128_u32(input)?;
    let (rest, value_type) = parse_value_type(rest)?;

    Ok((
        rest,
        FunctionLocal {
            count,
            value_type,
        }
    ))
}

pub fn parse_instruction(input: &[u8]) -> IResult<&[u8], Instruction> {
    let (rest, byte) = le_u8(input)?;
    let instruction = Instruction::from_u8(byte).ok_or_else(|| {
        warn!("no known instruction: {:04x}", byte);
        nom::Err::Failure(nom::error::Error::new(input, ErrorKind::Fail))
    })?;

    Ok((
        rest,
        instruction,
    ))
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
