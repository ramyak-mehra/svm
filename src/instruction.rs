pub const HALT: Value = Value::Instruction(Instruction::Halt);

pub const POP: Value = Value::Instruction(Instruction::Pop);
pub const PUSH: Value = Value::Instruction(Instruction::Push);
pub const DUP: Value = Value::Instruction(Instruction::Dup);

pub const ADD: Value = Value::Instruction(Instruction::Add);
pub const SUB: Value = Value::Instruction(Instruction::Sub);
pub const MUL: Value = Value::Instruction(Instruction::Mul);
pub const DIV: Value = Value::Instruction(Instruction::Divide);

pub const NOT: Value = Value::Instruction(Instruction::Not);
pub const AND: Value = Value::Instruction(Instruction::And);
pub const OR: Value = Value::Instruction(Instruction::Or);

pub const ISEQ: Value = Value::Instruction(Instruction::Iseq);
pub const ISGT: Value = Value::Instruction(Instruction::Isgt);
pub const ISGE: Value = Value::Instruction(Instruction::Isge);

pub const JMP: Value = Value::Instruction(Instruction::Jump);
pub const JIF: Value = Value::Instruction(Instruction::Jif);

pub const LOAD: Value = Value::Instruction(Instruction::Load);
pub const STORE: Value = Value::Instruction(Instruction::Store);

pub const RET: Value = Value::Instruction(Instruction::Ret);
pub const CALL: Value = Value::Instruction(Instruction::Call);

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Instruction {
    Halt,
    Push,
    Pop,
    Dup,
    Add,
    Sub,
    Mul,
    Divide,
    Not,
    And,
    Or,
    Iseq,
    Isgt,
    Isge,
    Jump,
    Jif,

    Load,
    Store,

    Ret,
    Call,
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
