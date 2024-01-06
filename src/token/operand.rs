use std::ops::{Add, AddAssign, BitAnd, BitOr, Div, Mul, Sub};

use super::Token;

#[derive(Debug, Clone)]

pub enum Operand {
    Null,
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
}

impl PartialEq<usize> for Operand {
    fn eq(&self, other: &usize) -> bool {
        match self {
            Operand::Int(v) => *v as usize == *other,
            _ => panic!("Invalid operation, please check type"),
        }
    }
}
impl PartialEq<Operand> for usize {
    fn eq(&self, other: &Operand) -> bool {
        match other {
            Operand::Int(v) => *v as usize == *self,
            _ => panic!("Invalid operation, please check type"),
        }
    }
}

impl PartialOrd<usize> for Operand {
    fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
        match self {
            Operand::Int(v) => v.partial_cmp(&(*other as i64)),
            Operand::Float(v) => v.partial_cmp(&(*other as f64)),
            _ => None,
        }
    }
}

impl PartialOrd for Operand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Int(l0), Self::Int(r0)) => l0.partial_cmp(r0),
            (Self::Float(l0), Self::Float(r0)) => l0.partial_cmp(r0),
            (Self::Str(l0), Self::Str(r0)) => l0.partial_cmp(r0),
            _ => panic!("Invalid comparison, please check type"),
        }
    }
}

impl From<bool> for Operand {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}
impl TryInto<String> for Operand {
    type Error = String;

    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            Operand::Str(s) => Ok(s),
            _ => Err("Invalid type conversion".to_owned()),
        }
    }
}
impl TryInto<usize> for Operand {
    type Error = String;

    fn try_into(self) -> Result<usize, Self::Error> {
        if let Operand::Int(v) = self {
            return if v >= 0 {
                Ok(v as usize)
            } else {
                Err("Invalid type conversion, i64 < 0".to_owned())
            };
        }
        Err("Invalid type conversion ".to_owned())
    }
}

impl PartialEq for Operand {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::Float(l0), Self::Float(r0)) => l0 == r0,
            (Self::Str(l0), Self::Str(r0)) => l0 == r0,
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Add for Operand {
    type Output = Operand;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Operand::Null, Operand::Null) => Operand::Null,
            (Operand::Int(l), Operand::Int(r)) => Operand::Int(l + r),
            (Operand::Float(l), Operand::Float(r)) => Operand::Float(l + r),
            (Operand::Float(l), Operand::Int(r)) => Operand::Float(l + r as f64),
            (Operand::Int(l), Operand::Float(r)) => Operand::Float(l as f64 + r),
            (Operand::Str(l), Operand::Str(r)) => {
                let mut l = l.clone();
                l.push_str(&r);
                Operand::Str(l)
            }
            _ => {
                panic!("Invalid operation, please check type")
            }
        }
    }
}

impl Mul for Operand {
    type Output = Operand;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Operand::Null, Operand::Null) => Operand::Null,
            (Operand::Int(l), Operand::Int(r)) => Operand::Int(l * r),
            (Operand::Float(l), Operand::Float(r)) => Operand::Float(l * r),
            (Operand::Float(l), Operand::Int(r)) => Operand::Float(l * r as f64),
            (Operand::Int(l), Operand::Float(r)) => Operand::Float(l as f64 * r),

            // (Operand::Int(l), Operand::Float(r)) => Operand::Float(l as f64 + r),
            _ => {
                panic!("Invalid operation, please check type")
            }
        }
    }
}

impl Div for Operand {
    type Output = Operand;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Operand::Null, Operand::Null) => Operand::Null,
            (Operand::Int(l), Operand::Int(r)) => Operand::Int(l / r),
            (Operand::Float(l), Operand::Float(r)) => Operand::Float(l / r),
            (Operand::Int(l), Operand::Float(r)) => Operand::Float(l as f64 / r),
            (Operand::Float(l), Operand::Int(r)) => Operand::Float(l / r as f64),
            _ => panic!("Invalid operation, please check type"),
        }
    }
}

impl Sub for Operand {
    type Output = Operand;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Operand::Null, Operand::Null) => Operand::Null,
            (Operand::Int(l), Operand::Int(r)) => Operand::Int(l - r),
            (Operand::Float(l), Operand::Float(r)) => Operand::Float(l - r),
            (Operand::Int(l), Operand::Float(r)) => Operand::Float(l as f64 - r),
            (Operand::Float(l), Operand::Int(r)) => Operand::Float(l - r as f64),

            _ => panic!("Invalid operation, please check type"),
        }
    }
}

impl BitAnd for Operand {
    type Output = Operand;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Operand::Null, Operand::Null) => Operand::Null,
            (Operand::Bool(l), Operand::Bool(r)) => Operand::Bool(l & r),
            _ => panic!("Invalid operation, please check type"),
        }
    }
}

impl BitOr for Operand {
    type Output = Operand;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Operand::Null, Operand::Null) => Operand::Null,
            (Operand::Bool(l), Operand::Bool(r)) => Operand::Bool(l | r),
            _ => panic!("Invalid operation, please check type"),
        }
    }
}

impl Add<i64> for Operand {
    type Output = Operand;

    fn add(self, rhs: i64) -> Self::Output {
        match self {
            Operand::Int(l) => Operand::Int(l + rhs),
            Operand::Float(l) => Operand::Float(l + rhs as f64),
            Operand::Str(s) => {
                let mut s = s.clone();
                s.push_str(&rhs.to_string());
                Operand::Str(s)
            }
            _ => panic!("Invalid operation, please check type"),
        }
    }
}

impl Add<f64> for Operand {
    type Output = Operand;

    fn add(self, rhs: f64) -> Self::Output {
        match self {
            Operand::Int(l) => Operand::Float(l as f64 + rhs),
            Operand::Float(l) => Operand::Float(l + rhs),
            Operand::Str(s) => {
                let mut s = s.clone();
                s.push_str(&rhs.to_string());
                Operand::Str(s)
            }
            _ => panic!("Invalid operation, please check type"),
        }
    }
}

impl Add<String> for Operand {
    type Output = Operand;

    fn add(self, rhs: String) -> Self::Output {
        match self {
            Operand::Int(l) => {
                let mut s = rhs.clone();
                s.push_str(&l.to_string());
                Operand::Str(rhs)
            }
            Operand::Float(l) => {
                let mut s = rhs.clone();
                s.push_str(&l.to_string());
                Operand::Str(rhs)
            }

            Operand::Str(s) => {
                let mut s = s.clone();
                s.push_str(&rhs.to_string());
                Operand::Str(s)
            }
            _ => panic!("Invalid operation, please check type"),
        }
    }
}

impl Add<bool> for Operand {
    type Output = Operand;

    fn add(self, rhs: bool) -> Self::Output {
        match self {
            _ => panic!("Invalid operation, please check type"),
        }
    }
}

impl AddAssign for Operand {
    fn add_assign(&mut self, rhs: Self) {
        match (self, rhs) {
            (Operand::Null, Operand::Null) => {}
            (Operand::Int(l), Operand::Int(r)) => *l += r,
            (Operand::Float(l), Operand::Float(r)) => *l += r,
            (Operand::Str(l), Operand::Str(r)) => l.push_str(&r),
            _ => {
                panic!("Invalid operation, please check type")
            }
        }
    }
}
mod test {
    use crate::{bool, float, int, str, token::Operand};

    #[test]
    fn test_add_assign() {
        let mut a = int!(10);
        a += int!(10);
        assert_eq!(a, int!(20));

        let mut a = float!(1.2);
        a += float!(1.5);
        assert_eq!(a, float!(2.7));

        let mut a = str!(String::from("Hello "));
        a += str!(String::from("world"));
        assert_eq!(a, str!(String::from("Hello world")));
    }

    #[test]
    #[should_panic]
    fn test_add_assign_panic() {
        let mut a = bool!(true);
        a += bool!(false);
    }
    //TODO: Add more tests
}
