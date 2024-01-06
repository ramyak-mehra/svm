use self::instruction::Instruction;
use self::operand::Operand;
pub mod instruction;
pub mod operand;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Instruction(Instruction),
    Data(Operand),
}

impl Token {
    pub fn data(d: Operand) -> Token {
        Token::Data(d)
    }
}

impl TryInto<bool> for Token {
    type Error = String;

    fn try_into(self) -> Result<bool, Self::Error> {
        match self {
            Token::Instruction(_) => Err("Invalid type conversion ".to_owned()),
            Token::Data(d) => match d {
                Operand::Bool(v) => Ok(v),
                _ => Err("Invalid type conversion ".to_owned()),
            },
        }
    }
}

impl TryInto<Operand> for Token {
    type Error = String;

    fn try_into(self) -> Result<Operand, Self::Error> {
        match self {
            Token::Instruction(_) => Err("Token is an instruction".to_owned()),
            Token::Data(d) => Ok(d),
        }
    }
}

mod test {
    use super::{operand::Operand, Token};

    #[test]
    fn test_to_operand() {
        let t = Token::Instruction(super::instruction::Instruction::Add);

        let o: Result<Operand, _> = t.try_into();
        assert!(o.is_err());

        let t = Token::Data(Operand::Null);
        let o: Result<Operand, _> = t.try_into();
        assert!(o.is_ok());
    }
    #[test]
    fn test_to_bool() {
        let t = Token::Instruction(super::instruction::Instruction::Add);
        let o: Result<bool, _> = t.try_into();
        assert!(o.is_err());

        let t = Token::Data(Operand::Null);
        let o: Result<bool, _> = t.try_into();
        assert!(o.is_err());

        let t = Token::Data(Operand::Bool(true));
        let o: Result<bool, _> = t.try_into();
        assert!(o.is_ok());
    }
}
