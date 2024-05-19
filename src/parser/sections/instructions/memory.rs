use crate::parser::sections::Instruction;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct MemArg {
    pub align: u32,
    pub offset: u32,
}

pub fn parse_memory_instruction(input: &[u8]) -> nom::IResult<&[u8], Instruction> {
    let (rest, op) = nom::number::complete::le_u8(input)?;
    todo!()
}
