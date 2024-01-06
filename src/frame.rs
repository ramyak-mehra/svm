use std::collections::HashMap;

use crate::token::operand::Operand;

#[derive(Debug, Default)]
pub struct Frame {
    variables: HashMap<String, Operand>, // TODO(optimization): Use usize for key
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
    pub fn get(&self, var: String) -> Operand {
        match self.variables.get(&var) {
            Some(v) => v.clone(),
            None => Operand::Null,
        }
    }

    pub fn set(&mut self, var: String, val: Operand) {
        self.variables.insert(var, val);
    }

    pub fn values(&self) -> Vec<Operand> {
        self.variables.values().map(|v| v.clone()).collect()
    }
}
