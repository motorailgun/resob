use crate::parser::sections::Instruction;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Index {
    Local(u32),
    Global(u32),
}

pub fn parse_variable_instruction(input: &[u8]) -> nom::IResult<&[u8], Instruction> {
    let (rest, (op, index)) = nom::sequence::pair(nom::number::complete::le_u8, nom_leb128::leb128_u32)(input)?;
    match op {
        0x20..=0x22 => {
            Ok((
                rest,
                Instruction {
                    opcode: op,
                    arguments: super::OpArgument::Index(Index::Local(index)),
                }
            ))
        },
        _ => {
            Ok((
                rest,
                Instruction {
                    opcode: op,
                    arguments: super::OpArgument::Index(Index::Global(index)),
                }
            ))
        }
    }
}
