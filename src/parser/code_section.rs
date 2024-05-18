use super::section::Section;
use super::types::*;
use log::*;
use nom::{
    bytes::complete::{tag, take},
    error::ErrorKind,
    multi::many0,
    number::complete::le_u8,
    sequence::terminated,
    IResult,
};
use nom_leb128::leb128_u32;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Instruction {
    opcode: u8,
    arguments: Vec<u8>,
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
    Ok((rest, Section::Code(CodeSection { functions })))
}

pub fn parse_function(input: &[u8]) -> IResult<&[u8], Function> {
    let (rest, size) = leb128_u32(input)?;
    let (remining, body) = take(size as usize)(rest)?;

    let (rest, locals) = parse_vec(parse_function_local, &body)?;
    let (rest, code) = terminated(many0(parse_instruction), tag([0x0b]))(rest)?;

    if rest.len() != 0 {
        warn!(
            "parse_instruction() didn't consume all the bytes, remaining: {}",
            rest.len()
        );
    }

    Ok((remining, Function { locals, code }))
}

pub fn parse_function_local(input: &[u8]) -> IResult<&[u8], FunctionLocal> {
    let (rest, count) = leb128_u32(input)?;
    let (rest, value_type) = parse_value_type(rest)?;

    Ok((rest, FunctionLocal { count, value_type }))
}

pub fn parse_instruction(input: &[u8]) -> IResult<&[u8], Instruction> {
    let (_rest, byte) = le_u8(input)?;

    // instructions may take up to 3 arguments (as of ver 1)
    // may be decided by the genre of the op (control, args, etc)
    let (rest, instruction) = match byte {
        // end
        0x0b => Err(nom::Err::Error(nom::error::Error::new(
            input,
            ErrorKind::AlphaNumeric,
        ))),
        // control instruction (5.4.1)
        0x00..=0x11 => todo!(),
        // parametric instruction (5.4.2)
        0x1a..=0x1b => todo!(),
        // variable instruction (5.4.3)
        0x20..=0x24 => todo!(),
        // memory instruction (5.4.4)
        0x28..=0x40 => todo!(),
        // numeric instruction (5.4.5)
        0x41..=0xbf => todo!(),
        // unknown
        _ => Err(nom::Err::Failure(nom::error::Error::new(
            input,
            ErrorKind::Fail,
        ))),
    }?;

    Ok((rest, instruction))
}
