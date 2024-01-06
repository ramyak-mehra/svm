use crate::token::Token;
use instruction::Instruction;
#[derive(Debug, PartialEq, Eq, Clone, Instruction)]
pub enum Instruction {
    Halt,
    Push,
    Pop,
    Dup,
    Add,
    Sub,
    Mul,
    Div,
    Not,
    And,
    Or,
    Iseq,
    Isgt,
    Isge,
    Jmp,
    Jif,

    Load,
    Store,

    Ret,
    Call,

    Write,
}
