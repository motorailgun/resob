use self::memory::MemArg;
use self::numeric::InstantValue;

pub mod control;
pub mod memory;
pub mod numeric;
pub mod parametric;
pub mod variable;

#[derive(Debug, Clone, PartialEq)]
pub enum OpArgument {
    MemArg(MemArg),
    InstantValue(InstantValue),
    None,
}
