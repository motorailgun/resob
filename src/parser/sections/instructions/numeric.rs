use crate::parser::sections::Instruction;

#[derive(Debug, Clone, PartialEq)]
pub enum InstantValue {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

pub fn parse_numeric_instruction(input: &[u8]) -> nom::IResult<&[u8], Instruction> {
    let (rest, op) = nom::number::complete::le_u8(input)?;
    match op {
        // consts
        0x41..=0x44 => {
            let (rest, value) = match op {
                0x41 => nom::combinator::map(nom_leb128::leb128_i32::<&[u8], nom::error::Error<_>>, |num| InstantValue::I32(num))(input),
                0x42 => nom::combinator::map(nom_leb128::leb128_i64, |num| InstantValue::I64(num))(input),
                0x43 => nom::combinator::map(nom::number::complete::le_f32, |num| InstantValue::F32(num))(input),
                0x44 => nom::combinator::map(nom::number::complete::le_f64, |num| InstantValue::F64(num))(input),
                _ => panic!("BUG: only opcode in [0x41, 0x44] should be here, got {:#04x}", op),
            }.unwrap();
            
            Ok((
                rest,
                Instruction {
                    opcode: op,
                    arguments: super::OpArgument::InstantValue(value),
                }
            ))
        },
        _ => Ok((
            rest,
            Instruction {
                opcode: op,
                arguments: super::OpArgument::None,
            }
        )),
    }
}
