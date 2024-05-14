use nom::error;
use nom::multi::count;
use nom::number::complete::le_u8;
use nom::IResult;
use nom_leb128::leb128_u32;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Debug, Clone, PartialEq, Eq, FromPrimitive)]
pub enum ValueType {
    I32 = 0x7F,
    I64 = 0x7E,
    F32 = 0x7D,
    F64 = 0x7C,
}

pub fn parse_value_type(input: &[u8]) -> IResult<&[u8], ValueType> {
    let (rest, val) = le_u8(input)?;
    match ValueType::from_u8(val) {
        Some(v) => Ok((rest, v)),
        None => Err(nom::Err::Failure(error::Error {
            input,
            code: error::ErrorKind::Fail,
        })),
    }
}

pub fn parse_vec<F, R>(func: F, input: &[u8]) -> IResult<&[u8], Vec<R>>
where
    F: FnMut(&[u8]) -> IResult<&[u8], R>,
{
    let (body, size) = leb128_u32(input)?;
    let (rest, vec) = count(func, size as usize)(body)?;

    Ok((rest, vec))
}
