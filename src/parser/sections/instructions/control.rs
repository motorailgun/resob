use num_derive::FromPrimitive;
use crate::parser::sections::Instruction;

#[derive(Eq, PartialEq, Debug, Clone, FromPrimitive)]
pub enum ControlInstruction {
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

pub fn parse_control_instruction(input: &[u8]) -> nom::IResult<&[u8], Instruction> {
    todo!()
}

pub fn parse_block(input: &[u8]) -> nom::IResult<&[u8], Vec<Instruction>> {
    todo!()
}
