pub mod code_section;
pub mod function_section;
pub mod instructions;
pub mod type_section;
pub mod types;

use code_section::*;
use function_section::*;
use log::*;
use nom::{
    bytes::complete::take,
    error::{self, ErrorKind},
    multi::many0,
    number::complete::le_u8,
    sequence::pair,
    Err::Error,
    IResult,
};
use nom_leb128::leb128_u32;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;
use type_section::*;

#[derive(Eq, PartialEq, Debug, FromPrimitive, ToPrimitive, Clone)]
#[repr(u8)]
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

#[derive(PartialEq, Debug, Clone)]
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
    let (rest, (section_code_raw, section_size)) = pair(le_u8, leb128_u32)(input)?;
    let header_length = input.len() - rest.len();

    debug!("parse_section: section code {:#04x}", section_code_raw);
    debug!("parse_section: section size:  {section_size}");

    let (input, _header) = take(header_length)(input)?;
    let (input, body) = take(section_size + 2 - header_length as u32)(input)?;
    let section_code = SectionCode::from_u8(section_code_raw);

    debug!("parse_section: body size: {}", &body.len());
    debug!("parse_section: section_code: {:?}", section_code);

    use SectionCode::*;
    match section_code {
        Some(scode) => Ok((
            input,
            match scode {
                Type => parse_type_section(body)?.1,
                Function => parse_function_section(body)?.1,
                Code => parse_code_section(body)?.1,
                _ => Section::Custom(GenericSection {}),
            },
        )),
        None => {
            warn!("no known section code: {:#04x}", section_code_raw);
            Err(Error(error::Error::new(input, ErrorKind::Fail)))?
        }
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

#[derive(PartialEq, Debug, Clone)]
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

impl Default for Sections {
    fn default() -> Self {
        Self::new()
    }
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

/* #[cfg(test)]
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
                body: vec![0x01, 0x02, 0x00, 0x0b],
            },
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
                function_types: vec![FuncType {
                    params: vec![],
                    results: vec![],
                }],
            }
        )
    }

    #[test]
    fn decode_simple_func_section() {
        let wasm = wat::parse_str("(module (func))").unwrap();
        let stripped_section = wasm[16..18].to_vec();
        let (_, section) = parse_function_section(&stripped_section).unwrap();

        dbg!(stripped_section);
        assert_eq!(section, FunctionSection { table: vec![0x00] })
    }
} */
