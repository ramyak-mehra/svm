use crate::{frame::Frame, instruction::*, stack};
use std::{collections::VecDeque, default};

struct Vm {
    halted: bool,
    ip: usize, //Instruction Pointer
    stack: VecDeque<Value>,
    program: Vec<Value>,
    frames: VecDeque<Frame>,
}

impl Vm {
    pub fn new(program: Vec<Value>) -> Self {
        Self {
            halted: false,
            ip: 0,
            stack: VecDeque::new(),
            program,
            frames: stack![Frame::default()],
        }
    }
    pub fn run(&mut self) {
        while !self.halted {
            self.step();
        }
    }

    fn step(&mut self) {
        if !self.halted {
            self.execute();
        }
    }

    fn current_frame_mut(&mut self) -> &mut Frame {
        self.frames.front_mut().unwrap()
    }
    fn current_frame(&self) -> &Frame {
        self.frames.front().unwrap()
    }

    fn execute(&mut self) {
        let value = self.nextValue();
        match value {
            Value::Instruction(i) => match i {
                Instruction::Halt => self.halted = true,
                Instruction::Pop => {
                    assert!(self.stack.len() >= 1);
                    self.stack.pop_front();
                }
                Instruction::Push => {
                    let v = self.nextValue().get_data().unwrap();
                    self.stack.push_front(Value::Data(v));
                }
                Instruction::Dup => {
                    assert!(self.stack.len() >= 1);
                    let v = self.stack.pop_front().unwrap();
                    self.stack.push_front(v.clone());
                    self.stack.push_front(v);
                }
                Instruction::Jmp => {
                    let v = self.nextValue().get_data().unwrap();
                    self.ip = v as usize;
                }
                Instruction::Jif => {
                    assert!(self.stack.len() >= 1);
                    let c = self.stack.pop_front().unwrap().vto_bool().unwrap();
                    let d1 = self.nextValue().get_data().unwrap();

                    if c {
                        self.ip = d1 as usize;
                    }
                }

                Instruction::Not => {
                    assert!(self.stack.len() >= 1);
                    let v1 = self.stack.pop_front().unwrap().vto_bool().unwrap();
                    let r = match v1 {
                        true => Value::v_false(),
                        false => Value::v_true(),
                    };
                    self.stack.push_front(r);
                }
                Instruction::Load => {
                    let v1 = self.nextValue().get_data().unwrap();
                    let var = self.current_frame().get(v1);

                    self.stack.push_front(Value::Data(var));
                }

                Instruction::Store => {
                    assert!(self.stack.len() >= 1);
                    let var = self.nextValue().get_data().unwrap();
                    let val = self.stack.pop_front().unwrap().get_data().unwrap();
                    self.current_frame_mut().set(var, val);
                }

                Instruction::Call => {
                    let address = self.nextValue().get_data().unwrap();
                    assert!(address > 0 && address < self.program.len().try_into().unwrap());
                    self.frames.push_front(Frame::new(self.ip));

                    self.ip = address as usize;
                }
                Instruction::Ret => {
                    assert!(self.frames.len() > 1);
                    let return_dddress = self.current_frame().return_address();

                    self.frames.pop_front();

                    self.ip = return_dddress;
                }

                Instruction::Add
                | Instruction::Div
                | Instruction::Mul
                | Instruction::Sub
                | Instruction::And
                | Instruction::Or
                | Instruction::Iseq
                | Instruction::Isge
                | Instruction::Isgt => {
                    assert!(self.stack.len() >= 2);
                    let d1 = self.stack.pop_front().unwrap().get_data().unwrap();
                    let d2 = self.stack.pop_front().unwrap().get_data().unwrap();
                    let r = Vm::execute_binary(i, d2, d1);
                    self.stack.push_front(Value::Data(r));
                }
            },
            Value::Data(_) => panic!("Cannot execute on data"),
        }
    }

    fn execute_binary(i: Instruction, d1: Operand, d2: Operand) -> Operand {
        match i {
            Instruction::Add => d1 + d2,
            Instruction::Sub => d1 - d2,
            Instruction::Mul => d1 * d2,
            Instruction::Div => d1 / d2,
            Instruction::And => (Value::to_bool(d1) && Value::to_bool(d2)).into(),
            Instruction::Or => (Value::to_bool(d1) || Value::to_bool(d2)).into(),
            Instruction::Isgt => (d1 > d2).into(),
            Instruction::Isge => (d1 >= d2).into(),
            Instruction::Iseq => (d1 == d2).into(),
            Instruction::Dup
            | Instruction::Halt
            | Instruction::Pop
            | Instruction::Push
            | Instruction::Jif
            | Instruction::Jmp
            | Instruction::Not
            | Instruction::Load
            | Instruction::Store
            | Instruction::Call
            | Instruction::Ret => panic!("Not a binary op"),
        }
    }

    fn nextValue(&mut self) -> Value {
        if self.ip >= self.program.len() {
            panic!("IP out of bounds")
        }
        let v = &self.program[self.ip];
        self.ip += 1;
        v.to_owned()
    }
}

mod test {
    use std::collections::VecDeque;

    use crate::{instruction::*, num, stack};

    use super::Vm;

    #[test]
    fn push_halt() {
        let mut vm = Vm::new(vec![PUSH, num!(10), PUSH, num!(12), HALT]);
        vm.run();
        assert_eq!(vm.ip, 5);
        assert!(vm.halted);

        assert_eq!(vm.stack, stack!(num![12], num![10]));
    }
    #[test]
    fn add() {
        let mut vm = Vm::new(vec![PUSH, num!(10), PUSH, num!(12), ADD, HALT]);
        vm.run();
        assert_eq!(vm.ip, 6);
        assert!(vm.halted);
        assert_eq!(vm.stack, stack![num!(22)]);
    }

    #[test]
    fn sub() {
        let mut vm = Vm::new(vec![PUSH, num!(10), PUSH, num!(12), SUB, HALT]);
        vm.run();
        assert_eq!(vm.ip, 6);
        assert!(vm.halted);
        assert_eq!(vm.stack, stack![num!(-2)]);
    }
    #[test]
    fn mul() {
        let mut vm = Vm::new(vec![PUSH, num!(10), PUSH, num!(12), MUL, HALT]);
        vm.run();
        assert_eq!(vm.ip, 6);
        assert!(vm.halted);
        assert_eq!(vm.stack, stack![num!(120)]);
    }
    #[test]
    fn divide() {
        let mut vm = Vm::new(vec![PUSH, num!(20), PUSH, num!(2), DIV, HALT]);
        vm.run();
        assert_eq!(vm.ip, 6);
        assert!(vm.halted);
        assert_eq!(vm.stack, stack![num!(10)]);
    }
    #[test]
    #[should_panic]
    fn test_no_sufficient_params() {
        let mut vm = Vm::new(vec![SUB, HALT]);
        vm.run();
    }
    #[test]
    fn evaluate_expressions() {
        // We van evalute expressions in reverse polish notation.
        // 1 2 3 MUL ADD 7 DIV => (1 + 2 * 3) / 7

        let mut vm = Vm::new(vec![
            PUSH,
            num!(1),
            PUSH,
            num!(2),
            PUSH,
            num!(3),
            MUL,
            ADD,
            PUSH,
            num!(7),
            DIV,
            HALT,
        ]);
        vm.run();
        assert_eq!(vm.ip, 12);
        assert!(vm.halted);
        assert_eq!(vm.stack, stack![num!(1)]);
    }

    #[test]
    fn test_not() {
        let mut vm = Vm::new(vec![PUSH, num!(1), NOT, HALT]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 4);
        assert_eq!(vm.stack, stack![num!(0)]);

        let mut vm = Vm::new(vec![PUSH, num!(0), NOT, HALT]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 4);
        assert_eq!(vm.stack, stack![num!(1)]);
    }

    #[test]
    #[should_panic]
    fn uniary_inseffiient() {
        let mut vm = Vm::new(vec![NOT, HALT]);
        vm.run();
    }

    #[test]
    fn test_and_true() {
        let mut vm = Vm::new(vec![PUSH, num!(1), PUSH, num!(1), AND, HALT]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 6);
        assert_eq!(vm.stack, stack![num!(1)]);
    }

    #[test]
    fn test_or() {
        let mut vm = Vm::new(vec![PUSH, num!(1), PUSH, num!(0), OR, HALT]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 6);
        assert_eq!(vm.stack, stack![num!(1)]);
    }

    #[test]
    fn test_pop() {
        let mut vm = Vm::new(vec![PUSH, num!(1), POP, HALT]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 4);
        assert!(vm.stack.is_empty())
    }
    #[test]
    #[should_panic]
    fn test_pop_insufficient() {
        let mut vm = Vm::new(vec![POP, HALT]);
        vm.step();
    }

    #[test]
    fn test_dup() {
        let mut vm = Vm::new(vec![PUSH, num!(1), DUP, HALT]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 4);
        assert_eq!(vm.stack, stack![num!(1), num!(1)]);
    }

    #[test]
    fn test_is_greater() {
        let mut vm = Vm::new(vec![PUSH, num!(1), PUSH, num!(2), ISGT, HALT]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 6);
        assert_eq!(vm.stack, stack![num!(0)]);

        let mut vm = Vm::new(vec![PUSH, num!(2), PUSH, num!(1), ISGT, HALT]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 6);
        assert_eq!(vm.stack, stack![num!(1)]);
    }

    #[test]
    fn test_is_greater_eq() {
        let mut vm = Vm::new(vec![PUSH, num!(1), PUSH, num!(1), ISGE, HALT]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 6);
        assert_eq!(vm.stack, stack![num!(1)]);

        let mut vm = Vm::new(vec![PUSH, num!(2), PUSH, num!(1), ISGE, HALT]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 6);
        assert_eq!(vm.stack, stack![num!(1)]);
    }

    #[test]
    fn test_is_eq() {
        let mut vm = Vm::new(vec![PUSH, num!(1), PUSH, num!(1), ISEQ, HALT]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 6);
        assert_eq!(vm.stack, stack![num!(1)]);

        let mut vm = Vm::new(vec![PUSH, num!(2), PUSH, num!(1), ISEQ, HALT]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 6);
        assert_eq!(vm.stack, stack![num!(0)]);
    }

    #[test]
    fn test_jump() {
        let mut vm = Vm::new(vec![JMP, num!(3), HALT, JMP, num!(2)]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 3);
    }

    #[test]
    fn test_jump_conditional() {
        let mut vm = Vm::new(vec![
            PUSH,
            num!(1),
            JIF,
            num!(5),
            POP,
            PUSH,
            num!(0),
            JIF,
            num!(4),
            HALT,
        ]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 10);
    }

    #[test]
    fn test_load() {
        let mut vm = Vm::new(vec![LOAD, num!(0), HALT]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 3);
    }
    #[test]
    fn test_store() {
        let mut vm = Vm::new(vec![PUSH, num!(42), STORE, num!(0), HALT]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 5);
        assert!(vm.stack.is_empty());
        assert_eq!(vm.current_frame().values(), vec![42]);
    }

    #[test]
    fn test_store_and_load() {
        let mut vm = Vm::new(vec![PUSH, num!(42), STORE, num!(0), LOAD, num!(0), HALT]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 7);
        assert_eq!(vm.current_frame().values(), vec![42]);
        assert_eq!(vm.stack, stack![num!(42)]);
    }

    #[test]
    #[should_panic]
    fn test_load_panic() {
        let mut vm = Vm::new(vec![LOAD]);
        vm.run();
    }

    #[test]
    #[should_panic]
    fn test_store_panic() {
        let mut vm = Vm::new(vec![STORE]);
        vm.run();
    }

    #[test]
    #[should_panic]
    fn test_store_panic2() {
        let mut vm = Vm::new(vec![STORE, num!(0), HALT]);
        vm.run();
    }
    #[test]
    fn test_if() {
        /*
         * The code is:
         * if (a > b) {
         *     c = a;
         * } else {
         *     c = b;
         * }
         *
         * We're going to use variable 0 as "a", variable 1 as "b", variable 2 as "c".
         */
        let mut vm = Vm::new(vec![
            // Init a with 6
            PUSH,
            num!(6),
            STORE,
            num!(0),
            // Init b with 4
            PUSH,
            num!(4),
            STORE,
            num!(1),
            // Load a and b on stack
            LOAD,
            num!(0), // Stack contains a
            LOAD,
            num!(1), // Stack contains b
            ISGT,    // Stack contains a > b
            JIF,
            num!(21),
            // Else path
            LOAD,
            num!(1), // Stack contains b
            STORE,
            num!(2), // Set c to the stack based head c = b
            JMP,
            num!(25),
            // This is the if path and address is 21
            LOAD,
            num!(0),
            STORE, // Set c to the stack head meaning c = a
            num!(2),
            // Done, this is address 25
            HALT,
        ]);
        vm.run();
        assert!(vm.halted);
        assert!(vm.stack.is_empty());
        assert_eq!(vm.current_frame().get(0), 6);
        assert_eq!(vm.current_frame().get(1), 4);
        assert_eq!(vm.current_frame().get(2), 6);
    }

    #[test]
    fn test_func_no_arguments_no_return() {
        let mut vm = Vm::new(vec![CALL, num!(3), HALT, RET]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 3);
        assert!(vm.stack.is_empty());
    }

    #[test]
    fn test_func_no_arguments_with_return() {
        let mut vm = Vm::new(vec![CALL, num!(3), HALT, PUSH, num!(7), RET]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 3);
        assert_eq!(vm.stack, stack![num!(7)]);
    }

    #[test]
    fn test_func_with_arguments_return() {
        let mut vm = Vm::new(vec![
            PUSH,
            num!(3),
            CALL,
            num!(5),
            HALT,
            PUSH,
            num!(2),
            MUL,
            RET,
        ]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 5);
        assert_eq!(vm.stack, stack![num!(6)]);
    }

    /*
     * int max(int a, int b) {
     *     if (a > b) {
     *         return a;
     *     } else {
     *         return b;
     *     }
     * }
     */

    #[test]
    fn test_max_ab() {
        let mut vm = Vm::new(vec![
            PUSH,
            num!(6), // First argument
            PUSH,
            num!(4), // Second argument
            CALL,
            num!(7), // Call the func
            HALT,
            STORE,   // 7th instruction
            num!(1), // store b
            STORE,
            num!(0), // store a
            LOAD,
            num!(0),
            LOAD,
            num!(1),
            ISGE,
            JIF,
            num!(21), // 17
            LOAD,
            num!(1),
            RET,
            LOAD, //21
            num!(0),
            RET,
        ]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 7);
        assert_eq!(vm.stack, stack![num!(6)]);
    }
}
