#[derive(Debug, PartialEq, Eq, Clone, instruction::Instruction)]
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
    // Write,
}

pub type Operand = i64;
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Value {
    Instruction(Instruction),
    Data(Operand),
}

impl Value {
    pub fn get_data(&self) -> Result<Operand, &str> {
        match self {
            Value::Instruction(_) => Err(""),
            Value::Data(d) => Ok(*d),
        }
    }
    pub fn number(d: Operand) -> Value {
        Value::Data(d)
    }

    pub fn to_bool(d: Operand) -> bool {
        d != 0
    }
    pub fn vto_bool(&self) -> Result<bool, &str> {
        match self {
            Value::Instruction(_) => Err(""),
            Value::Data(d) => Ok(d != &0),
        }
    }

    pub fn v_true() -> Value {
        Value::Data(1)
    }
    pub fn v_false() -> Value {
        Value::Data(0)
    }
}
