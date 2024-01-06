use crate::{
    data,
    frame::Frame,
    int, stack, tbool,
    token::{instruction::Instruction, operand::Operand, *},
};
use std::{collections::VecDeque, default};

struct Vm {
    halted: bool,
    ip: usize, //Instruction Pointer
    stack: VecDeque<Token>,
    program: Vec<Token>,
    frames: VecDeque<Frame>,
}

impl Vm {
    pub fn new(program: Vec<Token>) -> Self {
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
        let value = self.nextToken();
        match value {
            Token::Instruction(i) => match i {
                Instruction::Halt => self.halted = true,
                Instruction::Pop => {
                    assert!(self.stack.len() >= 1);
                    self.stack.pop_front();
                }
                Instruction::Push => {
                    let v = self.nextToken().try_into().unwrap();
                    self.stack.push_front(data!(v));
                }
                Instruction::Dup => {
                    assert!(self.stack.len() >= 1);
                    let v = self.stack.pop_front().unwrap();
                    self.stack.push_front(v.clone());
                    self.stack.push_front(v);
                }
                Instruction::Jmp => {
                    let v: Operand = self.nextToken().try_into().unwrap();
                    self.ip = v.try_into().unwrap();
                }
                Instruction::Jif => {
                    assert!(self.stack.len() >= 1);
                    let c = self.stack.pop_front().unwrap();
                    let d1: Operand = self.nextToken().try_into().unwrap();

                    if c.try_into().unwrap() {
                        self.ip = d1.try_into().unwrap();
                    }
                }

                Instruction::Not => {
                    assert!(self.stack.len() >= 1);
                    let v1 = self.stack.pop_front().unwrap();
                    let r = match v1.try_into().unwrap() {
                        true => tbool!(false),
                        false => tbool!(true),
                    };
                    self.stack.push_front(r);
                }
                Instruction::Load => {
                    let v1: Operand = self.nextToken().try_into().unwrap();
                    let var = self.current_frame().get(v1.try_into().unwrap());

                    self.stack.push_front(Token::Data(var));
                }

                Instruction::Store => {
                    assert!(self.stack.len() >= 1);
                    let var: Operand = self.nextToken().try_into().unwrap();
                    let val = self.stack.pop_front().unwrap().try_into().unwrap();
                    self.current_frame_mut().set(var.try_into().unwrap(), val);
                }

                Instruction::Call => {
                    let address: Operand = self.nextToken().try_into().unwrap();
                    assert!(address > 0 && address < self.program.len());
                    self.frames.push_front(Frame::new(self.ip));

                    self.ip = address.try_into().unwrap();
                }
                Instruction::Ret => {
                    assert!(self.frames.len() > 1);
                    let return_dddress = self.current_frame().return_address();

                    self.frames.pop_front();

                    self.ip = return_dddress;
                }
                Instruction::Write => {
                    if let Some(v) = self.stack.front() {
                        print!("{:?}", v);
                    }
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
                    let d2 = self.stack.pop_front().unwrap().try_into().unwrap();
                    let d1 = self.stack.pop_front().unwrap().try_into().unwrap();
                    let r = Vm::execute_binary(i, d1, d2);
                    self.stack.push_front(Token::Data(r));
                }
            },
            Token::Data(_) => panic!("Cannot execute on data"),
        }
    }

    fn execute_binary(i: Instruction, d1: Operand, d2: Operand) -> Operand {
        match i {
            Instruction::Add => d1 + d2,
            Instruction::Sub => d1 - d2,
            Instruction::Mul => d1 * d2,
            Instruction::Div => d1 / d2,
            Instruction::And => d1 & d2,
            Instruction::Or => d1 | d2,
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
            | Instruction::Write
            | Instruction::Ret => panic!("Not a binary op"),
        }
    }

    fn nextToken(&mut self) -> Token {
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

    use crate::{data, int, stack, tbool, tint, token::instruction::*, tstr};

    use super::Vm;

    #[test]
    fn push_halt() {
        let mut vm = Vm::new(vec![PUSH, tint!(10), PUSH, tint!(12), HALT]);
        vm.run();
        assert_eq!(vm.ip, 5);
        assert!(vm.halted);

        assert_eq!(vm.stack, stack![tint!(12), tint!(10)]);
    }
    #[test]
    fn add() {
        let mut vm = Vm::new(vec![PUSH, tint!(10), PUSH, tint!(12), ADD, HALT]);
        vm.run();
        assert_eq!(vm.ip, 6);
        assert!(vm.halted);
        assert_eq!(vm.stack, stack![tint!(22)]);
    }

    #[test]
    fn sub() {
        let mut vm = Vm::new(vec![PUSH, tint!(10), PUSH, tint!(12), SUB, HALT]);
        vm.run();
        assert_eq!(vm.ip, 6);
        assert!(vm.halted);
        assert_eq!(vm.stack, stack![tint!(-2)]);
    }
    #[test]
    fn mul() {
        let mut vm = Vm::new(vec![PUSH, tint!(10), PUSH, tint!(12), MUL, HALT]);
        vm.run();
        assert_eq!(vm.ip, 6);
        assert!(vm.halted);
        assert_eq!(vm.stack, stack![tint!(120)]);
    }
    #[test]
    fn divide() {
        let mut vm = Vm::new(vec![PUSH, tint!(20), PUSH, tint!(2), DIV, HALT]);
        vm.run();
        assert_eq!(vm.ip, 6);
        assert!(vm.halted);
        assert_eq!(vm.stack, stack![tint!(10)]);
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
            tint!(1),
            PUSH,
            tint!(2),
            PUSH,
            tint!(3),
            MUL,
            ADD,
            PUSH,
            tint!(7),
            DIV,
            HALT,
        ]);
        vm.run();
        assert_eq!(vm.ip, 12);
        assert!(vm.halted);
        assert_eq!(vm.stack, stack![tint!(1)]);
    }

    #[test]
    fn test_not() {
        let mut vm = Vm::new(vec![PUSH, tbool!(true), NOT, HALT]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 4);
        assert_eq!(vm.stack, stack![tbool!(false)]);

        let mut vm = Vm::new(vec![PUSH, tbool!(false), NOT, HALT]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 4);
        assert_eq!(vm.stack, stack![tbool!(true)]);
    }

    #[test]
    #[should_panic]
    fn uniary_inseffiient() {
        let mut vm = Vm::new(vec![NOT, HALT]);
        vm.run();
    }

    #[test]
    fn test_and_true() {
        let mut vm = Vm::new(vec![PUSH, tbool!(true), PUSH, tbool!(true), AND, HALT]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 6);
        assert_eq!(vm.stack, stack![tbool!(true)]);
    }

    #[test]
    fn test_or() {
        let mut vm = Vm::new(vec![PUSH, tbool!(true), PUSH, tbool!(false), OR, HALT]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 6);
        assert_eq!(vm.stack, stack![tbool!(true)]);
    }

    #[test]
    fn test_pop() {
        let mut vm = Vm::new(vec![PUSH, tint!(1), POP, HALT]);
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
        let mut vm = Vm::new(vec![PUSH, tint!(1), DUP, HALT]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 4);
        assert_eq!(vm.stack, stack![tint!(1), tint!(1)]);
    }

    #[test]
    fn test_is_greater() {
        let mut vm = Vm::new(vec![PUSH, tint!(1), PUSH, tint!(2), ISGT, HALT]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 6);
        assert_eq!(vm.stack, stack![tbool!(false)]);

        let mut vm = Vm::new(vec![PUSH, tint!(2), PUSH, tint!(1), ISGT, HALT]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 6);
        assert_eq!(vm.stack, stack![tbool!(true)]);
    }

    #[test]
    fn test_is_greater_eq() {
        let mut vm = Vm::new(vec![PUSH, tint!(3), PUSH, tint!(2), ISGE, HALT]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 6);
        assert_eq!(vm.stack, stack![tbool!(true)]);

        let mut vm = Vm::new(vec![PUSH, tint!(2), PUSH, tint!(1), ISGE, HALT]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 6);
        assert_eq!(vm.stack, stack![tbool!(true)]);
    }

    #[test]
    fn test_is_eq() {
        let mut vm = Vm::new(vec![PUSH, tint!(1), PUSH, tint!(1), ISEQ, HALT]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 6);
        assert_eq!(vm.stack, stack![tbool!(true)]);

        let mut vm = Vm::new(vec![PUSH, tint!(2), PUSH, tint!(1), ISEQ, HALT]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 6);
        assert_eq!(vm.stack, stack![tbool!(false)]);
    }

    #[test]
    fn test_jump() {
        let mut vm = Vm::new(vec![JMP, tint!(3), HALT, JMP, tint!(2)]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 3);
    }

    #[test]
    fn test_jump_conditional() {
        let mut vm = Vm::new(vec![
            PUSH,
            tbool!(true),
            JIF,
            tint!(5),
            POP,
            PUSH,
            tbool!(false),
            JIF,
            tint!(4),
            HALT,
        ]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 10);
    }

    #[test]
    fn test_load() {
        let mut vm = Vm::new(vec![LOAD, tstr!(String::from("a")), HALT]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 3);
    }
    #[test]
    fn test_store() {
        let mut vm = Vm::new(vec![PUSH, tint!(42), STORE, tstr!(String::from("a")), HALT]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 5);
        assert!(vm.stack.is_empty());
        assert_eq!(vm.current_frame().values(), vec![42]);
    }

    #[test]
    fn test_store_and_load() {
        let mut vm = Vm::new(vec![
            PUSH,
            tint!(42),
            STORE,
            tstr!(String::from("a")),
            LOAD,
            tstr!(String::from("a")),
            HALT,
        ]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 7);
        assert_eq!(vm.current_frame().values(), vec![42]);
        assert_eq!(vm.stack, stack![tint!(42)]);
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
        let mut vm = Vm::new(vec![STORE, tint!(0), HALT]);
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
        let a = String::from("a");
        let b = String::from("b");
        let c = String::from("c");
        let mut vm = Vm::new(vec![
            // Init a with 6
            PUSH,
            tint!(6),
            STORE,
            tstr!(a.clone()),
            // Init b with 4
            PUSH,
            tint!(4),
            STORE,
            tstr!(b.clone()),
            // Load a and b on stack
            LOAD,
            tstr!(a.clone()), // Stack contains a
            LOAD,
            tstr!(b.clone()), // Stack contains b
            ISGT,             // Stack contains a > b
            JIF,
            tint!(21),
            // Else path
            LOAD,
            tstr!(b.clone()), // Stack contains b
            STORE,
            tstr!(c.clone()), // Set c to the stack based head c = b
            JMP,
            tint!(25),
            // This is the if path and address is 21
            LOAD,
            tstr!(a.clone()),
            STORE, // Set c to the stack head meaning c = a
            tstr!(c.clone()),
            // Done, this is address 25
            HALT,
        ]);
        vm.run();
        assert!(vm.halted);
        assert!(vm.stack.is_empty());
        assert_eq!(vm.current_frame().get(a), 6);
        assert_eq!(vm.current_frame().get(b), 4);
        assert_eq!(vm.current_frame().get(c), 6);
    }

    #[test]
    fn test_func_no_arguments_no_return() {
        let mut vm = Vm::new(vec![CALL, tint!(3), HALT, RET]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 3);
        assert!(vm.stack.is_empty());
    }

    #[test]
    fn test_func_no_arguments_with_return() {
        let mut vm = Vm::new(vec![CALL, tint!(3), HALT, PUSH, tint!(7), RET]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 3);
        assert_eq!(vm.stack, stack![tint!(7)]);
    }

    #[test]
    fn test_func_with_arguments_return() {
        let mut vm = Vm::new(vec![
            PUSH,
            tint!(3),
            CALL,
            tint!(5),
            HALT,
            PUSH,
            tint!(2),
            MUL,
            RET,
        ]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 5);
        assert_eq!(vm.stack, stack![tint!(6)]);
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
        let a = String::from("a");
        let b = String::from("b");
        let mut vm = Vm::new(vec![
            PUSH,
            tint!(6), // First argument
            PUSH,
            tint!(4), // Second argument
            CALL,
            tint!(7), // Call the func
            HALT,
            STORE,            // 7th instruction
            tstr!(b.clone()), // store b
            STORE,
            tstr!(a.clone()), // store a
            LOAD,
            tstr!(a.clone()),
            LOAD,
            tstr!(b.clone()),
            ISGE,
            JIF,
            tint!(21), // 17
            LOAD,
            tstr!(b.clone()),
            RET,
            LOAD, //21
            tstr!(a.clone()),
            RET,
        ]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 7);
        assert_eq!(vm.stack, stack![tint!(6)]);
    }

    #[test]
    // Run the test with --nocapture to see the output.
    fn test_write_stdout() {
        let mut vm = Vm::new(vec![PUSH, tint!(3), WRITE, HALT]);
        vm.run();
        assert!(vm.halted);
        assert_eq!(vm.ip, 4);
    }
}
