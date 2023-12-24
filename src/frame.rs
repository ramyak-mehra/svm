use std::collections::HashMap;

use crate::instruction::Operand;

#[derive(Debug, Default)]
pub struct Frame {
    variables: HashMap<i64, Operand>,
    return_address: usize,
}

impl Frame {
    pub fn new(address: usize) -> Frame {
        Self {
            variables: Default::default(),
            return_address: address,
        }
    }
    pub fn return_address(&self) -> usize {
        self.return_address
    }
    pub fn get(&self, var: i64) -> Operand {
        match self.variables.get(&var) {
            Some(v) => *v,
            None => 0,
        }
    }

    pub fn set(&mut self, var: i64, val: Operand) {
        self.variables.insert(var, val);
    }
    pub fn values(&self) -> Vec<Operand> {
        self.variables.values().map(|v| *v).collect()
    }
}
