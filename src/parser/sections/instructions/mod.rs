use self::memory::MemArg;

pub mod control;
pub mod memory;
pub mod numeric;
pub mod parametric;
pub mod variable;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpArgument {
    MemArg(MemArg),
}
